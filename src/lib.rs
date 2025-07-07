use regex::Regex;
use std::str::FromStr;

pub mod bit_array;
pub mod fixed_queue;
pub mod grid;
pub mod integer_interval;
pub mod intersect_sorted_iterators;
pub mod pos;
pub mod pos3d;
pub mod timer;

/// To use this crate, add `stephen-morris_utils = { path = "stephen-morris-utils" }` to the dependencies in Cargo.toml.
/// Alternatively, use the git repository by adding 'utils = { git = "https://github.com/SteveStyle/utils.git" }'
pub fn get_numbers<T: FromStr>(source: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug, /* add to toml
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
    result
}
