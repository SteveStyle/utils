use std::fmt::Debug;
use std::time::Instant;
use std::{ops::Deref, ops::DerefMut, time::Duration};

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
        let timed = time(|| a + b, "");
        timed.print_duration();
        timed.print_all();
        assert_eq!(*timed, 12);
        assert!(timed.duration > Duration::from_secs(0));
    }
}
