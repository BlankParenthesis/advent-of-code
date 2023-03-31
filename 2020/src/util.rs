use std::fs;

pub trait Pluralizable<T> {
    fn pluralize(self, count: T) -> Self;
}

impl Pluralizable<usize> for String {
    fn pluralize(self, count: usize) -> String {
        match count {
            1 => self,
            _ => format!("{}s", self),
        }
    }
}

pub fn print_day(i: u64) {
    print!("Day {}: ", i);
}

pub fn read_file<T, F>(location: &str, map_function: F) -> Vec<T> 
where F: FnMut(&str) -> T 
{
	read_file_delimiter(location, "\n", map_function)
}

pub fn read_file_delimiter<T, F>(location: &str, delimiter: &str, map_function: F) -> Vec<T> 
where F: FnMut(&str) -> T 
{
    fs::read_to_string(location)
        .unwrap()
        .split(delimiter)
		.map(map_function)
		.collect()
}
