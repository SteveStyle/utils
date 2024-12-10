use std::fmt::Debug;
use std::time::Instant;
use std::{ops::Deref, time::Duration};

pub struct Timed<T: Debug> {
    value: T,
    duration: Duration,
    tag: String,
}

impl<T: Debug> Deref for Timed<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
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

pub fn time<T: Debug, F>(f: F, tag: &str) -> Timed<T>
where
    F: FnOnce() -> T,
{
    let now = Instant::now();
    let value = f();
    let duration = now.elapsed();
    Timed::new(value, duration, tag)
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let a = 5;
        let b = 7;
        let timed = time(|| a + b, "");
        timed.print_duration();
        timed.print_all();
        assert_eq!(*timed, 12);
        assert!(timed.duration > Duration::from_secs(0));
    }
}
