// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            return Err(String::from("length is zero"));
        }

        let string_vec: Vec<&str> = s.split(",").collect();

        let name = string_vec[0].to_string();
        let age_result = string_vec[1].parse::<usize>();

        match age_result {
            Ok(v) => {
                Ok(Person { name, age: v })
            }
            Err(e) => {
                Err(String::from("parseing error"))
            }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        assert!("John,32".parse::<Person>().is_ok());
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John".parse::<Person>().unwrap();
    }
}