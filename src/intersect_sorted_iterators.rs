pub struct IntersectionIterator<'a, T: Ord, I1: Iterator<Item = T>, I2: Iterator<Item = T>> {
    iter1: &'a mut I1,
    iter2: &'a mut I2,
    last1: Option<T>,
    // last2: Option<&'a T>,
    initial: bool,
}

#[allow(clippy::needless_lifetimes)]
impl<'a, T: Ord, I1: Iterator<Item = T>, I2: Iterator<Item = T>> Iterator
    for IntersectionIterator<'a, T, I1, I2>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // If this is the first call to next read the first values into last1 and last2.
        // Otherwise we previously found a match, the value stored in last1.  We must advance
        // at least one of the values past this match value, so advance last2.
        // We keep last1 in the struct, so we remember the value between calls.  We keep last2
        // as a local variable so we can return it via a move.
        let mut last2 = self.iter2.next();
        if self.initial {
            self.last1 = self.iter1.next();
            self.initial = false;
        } else {
            while last2 == self.last1 {
                last2 = self.iter2.next();
            }
        }
        let mut match_found = false;
        while let (Some(v1), Some(v2)) = (&self.last1, &last2) {
            match v1.cmp(v2) {
                std::cmp::Ordering::Less => self.last1 = self.iter1.next(),
                std::cmp::Ordering::Greater => last2 = self.iter2.next(),
                std::cmp::Ordering::Equal => {
                    match_found = true;
                    break;
                }
            }
        }

        if match_found { last2 } else { None }
    }
}

impl<'a, T: Ord, I1: Iterator<Item = T>, I2: Iterator<Item = T>>
    IntersectionIterator<'a, T, I1, I2>
{
    pub fn new(iter1: &'a mut I1, iter2: &'a mut I2) -> Self {
        Self {
            iter1,
            iter2,
            last1: None,
            initial: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntersectionIterator;

    #[test]
    fn test_strings() {
        let strings1 = [
            "aaa".to_string(),
            "bbb".to_string(),
            "ccc".to_string(),
            "ddd".to_string(),
            "ddd".to_string(),
            "ddd".to_string(),
            "ddd".to_string(),
            "eee".to_string(),
        ];
        let strings2 = [
            "bbb".to_string(),
            "ddd".to_string(),
            "ddd".to_string(),
            "fff".to_string(),
        ];
        let mut iter1 = strings1.into_iter();
        let mut iter2 = strings2.into_iter();
        let intersection = IntersectionIterator::new(&mut iter1, &mut iter2);
        for s in intersection {
            println!("{s}");
        }
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_numbers() {
        let v1 = vec![1, 3, 5, 7, 9, 11];
        let v2 = vec![0, 2, 4, 7, 8, 9, 10];
        let mut iter1 = v1.iter();
        let mut iter2 = v2.iter();
        let intersection = IntersectionIterator::new(&mut iter1, &mut iter2);
        for s in intersection {
            println!("{s}");
        }
    }
}
