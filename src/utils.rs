use std::str::FromStr;
use regex::Regex;
use std::time::{Duration, Instant};



pub fn get_numbers<T: FromStr>(source: &str) -> Vec<T>
where T::Err: std::fmt::Debug
/* add to toml
[dependencies]
regex = "1.3.9" 
*/
{
    // Use a regular expression to match the first sequence of digits in the string
    // Support negative and floating point numbers.
    let re = Regex::new(r"-?\d+(\.\d+)?").unwrap();
    let mut result: Vec<T> = vec![];
    for captures in re.captures_iter(source) {
        let digit_string = captures.get(0).unwrap().as_str();
        let number = digit_string.parse().unwrap();
        result.push(number);
    }
    return result;
}

pub struct Timer<'a> {
    name: &'a str,
    start: Instant,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        let start = Instant::now();
        Timer { name, start }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.start)
    }
}
