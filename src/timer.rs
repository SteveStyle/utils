use std::fmt::Debug;
use std::hint::black_box;
use std::time::Instant;
use std::{ops::Deref, ops::DerefMut, time::Duration};

pub const DEFAULT_WARMUP: usize = 0;

pub struct Timed<T: Debug> {
    value: T,
    duration: Duration,
    tag: String,
}

impl<T: Debug> Deref for Timed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Debug> DerefMut for Timed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Debug> Timed<T> {
    pub fn new(value: T, duration: Duration, tag: &str) -> Self {
        Timed {
            value,
            duration,
            tag: String::from(tag),
        }
    }
    pub fn duration(&self) -> Duration {
        self.duration
    }
    pub fn print_duration(&self) {
        println!("{} duration: {:?}", self.tag, self.duration);
    }
    pub fn print_all(&self) {
        println!(
            "{} duration: {:?} with value {:?}",
            self.tag, self.duration, self.value
        );
    }
}

pub fn time<T: Debug, F>(mut f: F, tag: &str, warmup: usize) -> Timed<T>
where
    F: FnMut() -> T,
{
    let runs = warmup + 1;
    let mut durations = Vec::with_capacity(runs);

    let first_start = Instant::now();
    let mut value = black_box(f());
    durations.push(first_start.elapsed());

    for _ in 1..runs {
        let run_start = Instant::now();
        value = black_box(f());
        durations.push(run_start.elapsed());
    }

    durations.sort_unstable();
    let mid = durations.len() / 2;
    let duration = if durations.len() % 2 == 1 {
        durations[mid]
    } else {
        (durations[mid - 1] + durations[mid]) / 2
    };

    Timed::new(value, duration, tag)
}

pub fn inferred_tag_from_closure(closure: &str) -> String {
    let trimmed = closure.trim();

    if let Some(rest) = trimmed.strip_prefix("||") {
        let tag = rest.trim();
        return if tag.is_empty() {
            trimmed.to_string()
        } else {
            tag.to_string()
        };
    }

    if let Some(stripped) = trimmed.strip_prefix('|')
        && let Some(end) = stripped.find('|')
    {
        let tag = stripped[end + 1..].trim();
        return if tag.is_empty() {
            trimmed.to_string()
        } else {
            tag.to_string()
        };
    }

    trimmed.to_string()
}

#[macro_export]
macro_rules! time {
    ($closure:expr) => {{
        let __tag = $crate::timer::inferred_tag_from_closure(stringify!($closure));
        $crate::timer::time($closure, &__tag, $crate::timer::DEFAULT_WARMUP)
    }};
    ($closure:expr, $warmup:expr $(,)?) => {{
        let __tag = $crate::timer::inferred_tag_from_closure(stringify!($closure));
        $crate::timer::time($closure, &__tag, $warmup)
    }};
}

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    start: Instant,
    duration: Duration,
    running: bool,
}

impl Timer {
    pub fn new(running: bool) -> Timer {
        let start = Instant::now();
        Timer {
            start,
            duration: Duration::new(0, 0),
            running,
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            self.start = Instant::now();
            self.running = true;
        }
    }

    pub fn stop(&mut self) -> Duration {
        if self.running == true {
            self.duration += Instant::now().duration_since(self.start);
            self.running = false;
        }
        self.duration
    }

    pub fn reset(&mut self, running: bool) {
        self.start = Instant::now();
        self.duration = Duration::new(0, 0);
        self.running = running;
    }

    pub fn elapsed(&self) -> Duration {
        if self.running {
            self.duration + Instant::now().duration_since(self.start)
        } else {
            self.duration
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let a = 5;
        let b = 7;
        let timed = time(|| a + b, "", 0);
        timed.print_duration();
        timed.print_all();
        assert_eq!(*timed, 12);
        assert!(timed.duration > Duration::from_secs(0));
    }

    #[test]
    fn test_time_warmup_runs_closure() {
        let mut count = 0;
        let timed = time(
            || {
                count += 1;
                count
            },
            "",
            2,
        );

        assert_eq!(*timed, 3);
    }

    #[test]
    fn test_time_uses_median_over_all_runs() {
        let mut count = 0;
        let timed = time(
            || {
                count += 1;
                std::thread::sleep(Duration::from_millis(3));
                count
            },
            "",
            2,
        );

        assert_eq!(*timed, 3);
        assert!(timed.duration() >= Duration::from_millis(2));
        assert!(timed.duration() < Duration::from_millis(10));
    }

    #[test]
    fn test_inferred_tag_from_closure() {
        assert_eq!(inferred_tag_from_closure("|| a + b"), "a + b");
        assert_eq!(inferred_tag_from_closure("|x| x + 1"), "x + 1");
    }

    #[test]
    fn test_time_macro_uses_default_warmup() {
        let mut count = 0;
        let timed = crate::time!(|| {
            count += 1;
            count
        });

        assert_eq!(*timed, DEFAULT_WARMUP + 1);
    }

    #[test]
    fn test_time_macro_accepts_warmup_override() {
        let mut count = 0;
        let timed = crate::time!(
            || {
                count += 1;
                count
            },
            2,
        );

        assert_eq!(*timed, 3);
    }
}
