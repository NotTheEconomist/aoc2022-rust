use std::collections::HashSet;

use nom::{
    self,
    character::complete::anychar,
    combinator::{map, recognize, verify},
    multi::many_till,
    IResult,
};

fn no_repeats(s: &str) -> bool {
    let mut set = HashSet::new();
    for char in s.chars() {
        set.insert(char);
    }
    s.len() == set.len()
}

fn consecutive_characters_without_repeat(count: usize) -> impl Fn(&str) -> IResult<&str, String> {
    move |input: &str| {
        map(
            verify(nom::multi::count(anychar, count), |v: &Vec<char>| {
                no_repeats(&v.iter().collect::<String>())
            }),
            |v| v.into_iter().collect(),
        )(input)
    }
}

fn start_code(input: &str) -> IResult<&str, String> {
    consecutive_characters_without_repeat(4)(input)
}

fn message_code(input: &str) -> IResult<&str, String> {
    consecutive_characters_without_repeat(14)(input)
}

pub fn line_through_start_code(input: &str) -> IResult<&str, &str> {
    recognize(many_till(anychar, start_code))(input)
}

pub fn line_through_message_code(input: &str) -> IResult<&str, &str> {
    recognize(many_till(anychar, message_code))(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn start_code() {
        let (_, result) = super::start_code("abcd").unwrap();
        assert_eq!(result, "abcd".to_string());
        let (rest, result) = super::start_code("abcdefg").unwrap();
        assert_eq!((rest, result), ("efg", "abcd".to_string()));
        assert!(matches!(super::start_code("aabc"), Err(_)))
    }

    #[test]
    fn line_through_start_code() {
        let (_, result) = super::line_through_start_code("abcd").unwrap();
        assert_eq!(result, "abcd");
        let (rest, result) = super::line_through_start_code("lkjlkjmxyz").unwrap();
        assert_eq!(rest, "xyz");
        assert_eq!(result, "lkjlkjm");
    }
}
