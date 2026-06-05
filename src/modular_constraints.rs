use std::ops::AddAssign;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModularConstraint {
    constraints: Vec<SimpleConstraint>,
}

impl ModularConstraint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn constraints(&self) -> &[SimpleConstraint] {
        &self.constraints
    }

    pub fn add_constraint(&mut self, constraint: SimpleConstraint) {
        self.constraints.push(constraint);
    }

    pub fn split_by_prime_powers(&mut self) {
        self.constraints = self
            .constraints
            .iter()
            .flat_map(SimpleConstraint::split_by_prime_powers)
            .collect();
    }

    pub fn merge_constraints(&mut self) -> Option<()> {
        if self.constraints.len() <= 1 {
            return Some(());
        }

        let mut constraints = std::mem::take(&mut self.constraints);
        constraints.sort_by(|left, right| {
            left.modulo
                .cmp(&right.modulo)
                .then(left.allowed.cmp(&right.allowed))
        });
        constraints.dedup();

        // Stream same-modulo intersection and cross-modulo merging to avoid
        // allocating an intermediate grouped vector.
        let mut constraints = constraints.into_iter();
        let mut current_group = constraints.next().expect("at least one constraint exists");
        let mut merged: Option<SimpleConstraint> = None;

        for constraint in constraints {
            if current_group.modulo == constraint.modulo {
                if !current_group.intersect_same_modulo(&constraint) {
                    return None;
                }
                continue;
            }

            if let Some(accumulated) = merged.as_mut() {
                accumulated.try_add_assign(current_group)?;
            } else {
                merged = Some(current_group);
            }
            current_group = constraint;
        }

        if let Some(accumulated) = merged.as_mut() {
            accumulated.try_add_assign(current_group)?;
        } else {
            merged = Some(current_group);
        }

        let merged = merged.expect("at least one constraint exists");
        self.constraints.push(merged);
        Some(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimpleConstraint {
    allowed: Vec<u64>,
    pub modulo: u64,
}

impl SimpleConstraint {
    pub fn new_allowed(modulo: u64, values: Vec<u64>) -> Self {
        assert!(modulo > 0, "modulo must be non-zero");
        Self {
            allowed: normalize_values(values, modulo),
            modulo,
        }
    }

    pub fn new_forbidden(modulo: u64, forbidden: Vec<u64>) -> Self {
        assert!(modulo > 0, "modulo must be non-zero");
        let forbidden = normalize_values(forbidden, modulo);

        if forbidden.is_empty() {
            return Self::new_allowed(modulo, (0..modulo).collect());
        }

        let mut allowed = Vec::with_capacity(modulo as usize - forbidden.len());
        let mut forbidden_iter = forbidden.iter().copied().peekable();

        for value in 0..modulo {
            if forbidden_iter.peek() == Some(&value) {
                forbidden_iter.next();
            } else {
                allowed.push(value);
            }
        }

        Self::new_allowed(modulo, allowed)
    }

    pub fn from_forbidden_value(value: u64, modulo: u64) -> Self {
        Self::new_forbidden(modulo, vec![value])
    }

    pub fn allowed_count(&self) -> u64 {
        self.allowed.len() as u64
    }

    pub fn forbidden_count(&self) -> usize {
        self.modulo as usize - self.allowed.len()
    }

    pub fn allowed_density(&self) -> f64 {
        self.allowed_count() as f64 / self.modulo as f64
    }

    pub fn disallowed_density(&self) -> f64 {
        self.forbidden_count() as f64 / self.modulo as f64
    }

    pub fn min_allowed(&self) -> Option<u64> {
        self.allowed.first().copied()
    }

    pub fn split_by_prime_powers(&self) -> Vec<Self> {
        let factors = prime_powers(self.modulo);
        factors
            .into_iter()
            .map(|factor| {
                let values = self.allowed.iter().map(|value| value % factor).collect();
                Self::new_allowed(factor, values)
            })
            .collect()
    }

    pub fn try_add_assign(&mut self, rhs: Self) -> Option<()> {
        let new_modulo = lcm(self.modulo, rhs.modulo);
        let mut new_values = Vec::new();

        let (outer, inner, outer_modulo, inner_modulo) = if self.allowed.len() <= rhs.allowed.len()
        {
            (&self.allowed, &rhs.allowed, self.modulo, rhs.modulo)
        } else {
            (&rhs.allowed, &self.allowed, rhs.modulo, self.modulo)
        };

        for &outer_value in outer {
            for &inner_value in inner {
                if let Some(value) =
                    solve_pair(outer_value, outer_modulo, inner_value, inner_modulo)
                {
                    new_values.push(value % new_modulo);
                }
            }
        }

        let new_values = normalize_values(new_values, new_modulo);
        if new_values.is_empty() {
            return None;
        }

        self.allowed = new_values;
        self.modulo = new_modulo;
        Some(())
    }

    fn intersect_same_modulo(&mut self, rhs: &Self) -> bool {
        debug_assert_eq!(self.modulo, rhs.modulo);

        let mut left = 0;
        let mut right = 0;
        let mut intersected = Vec::with_capacity(self.allowed.len().min(rhs.allowed.len()));

        while left < self.allowed.len() && right < rhs.allowed.len() {
            match self.allowed[left].cmp(&rhs.allowed[right]) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right += 1,
                std::cmp::Ordering::Equal => {
                    intersected.push(self.allowed[left]);
                    left += 1;
                    right += 1;
                }
            }
        }

        if intersected.is_empty() {
            return false;
        }

        self.allowed = intersected;
        true
    }
}

impl AddAssign for SimpleConstraint {
    fn add_assign(&mut self, rhs: Self) {
        self.try_add_assign(rhs)
            .expect("constraints are unsatisfiable after merge");
    }
}

fn solve_pair(a: u64, m: u64, b: u64, n: u64) -> Option<u64> {
    let a = a % m;
    let b = b % n;
    let g = gcd(m, n);
    let diff = b as i128 - a as i128;

    if diff % g as i128 != 0 {
        return None;
    }

    let m_reduced = (m / g) as i128;
    let n_reduced = (n / g) as i128;
    let inv = mod_inverse(m_reduced.rem_euclid(n_reduced), n_reduced)?;
    let step = (diff / g as i128 * inv).rem_euclid(n_reduced);
    let lcm = lcm(m, n) as i128;

    Some((a as i128 + m as i128 * step).rem_euclid(lcm) as u64)
}

fn mod_inverse(a: i128, modulo: i128) -> Option<i128> {
    let (g, x, _) = extended_gcd(a, modulo);
    if g != 1 {
        return None;
    }
    Some(x.rem_euclid(modulo))
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a.abs(), a.signum(), 0);
    }
    let (g, x1, y1) = extended_gcd(b, a.rem_euclid(b));
    let x = y1;
    let y = x1 - (a / b) * y1;
    (g, x, y)
}

fn normalize_values(values: Vec<u64>, modulo: u64) -> Vec<u64> {
    let mut values: Vec<u64> = values.into_iter().map(|value| value % modulo).collect();
    values.sort_unstable();
    values.dedup();
    values
}

fn prime_powers(mut n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![1];
    }

    let mut factors = Vec::new();
    let mut p = 2;

    while p <= n / p {
        if n % p == 0 {
            let mut power = 1;
            while n % p == 0 {
                power *= p;
                n /= p;
            }
            factors.push(power);
        }
        p += if p == 2 { 1 } else { 2 };
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_constraint_can_hold_multiple_values() {
        let constraint = SimpleConstraint::new_allowed(6, vec![1, 3]);
        assert_eq!(constraint.allowed_count(), 2);
        assert_eq!(constraint.forbidden_count(), 4);
        assert_eq!(constraint.min_allowed(), Some(1));
        assert_eq!(constraint.modulo, 6);
    }

    #[test]
    fn add_assign_merges_multi_value_constraints() {
        let mut lhs = SimpleConstraint::new_allowed(6, vec![1, 3]);
        let rhs = SimpleConstraint::new_allowed(4, vec![1]);

        lhs += rhs;

        assert_eq!(lhs.modulo, 12);
        assert_eq!(lhs.min_allowed(), Some(1));
    }

    #[test]
    fn split_constraint_by_prime_powers() {
        let constraint = SimpleConstraint::new_allowed(12, vec![1, 5, 9]);
        let split = constraint.split_by_prime_powers();

        assert_eq!(split.len(), 2);
        assert_eq!(split[0].modulo, 4);
        assert_eq!(split[1].modulo, 3);
    }

    #[test]
    fn modular_constraint_supports_add_split_and_merge() {
        let mut constraints = ModularConstraint::new();
        constraints.add_constraint(SimpleConstraint::new_allowed(6, vec![1, 3]));
        constraints.add_constraint(SimpleConstraint::new_forbidden(4, vec![1]));

        constraints.split_by_prime_powers();
        assert_eq!(constraints.constraints().len(), 3);

        constraints.merge_constraints().unwrap();
        assert_eq!(constraints.constraints().len(), 1);
        assert!(constraints.constraints()[0].min_allowed().is_some());
    }

    #[test]
    fn merge_returns_none_for_unsatisfiable_constraints() {
        let mut constraints = ModularConstraint::new();
        constraints.add_constraint(SimpleConstraint::new_allowed(2, vec![0]));
        constraints.add_constraint(SimpleConstraint::new_forbidden(2, vec![0]));

        assert!(constraints.merge_constraints().is_none());
    }
}
