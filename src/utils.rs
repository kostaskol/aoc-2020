use std::fs;

pub fn read_lines(input: &str) -> Vec<&str> {
    let content = fs::read_to_string(input).expect(&format!("Put something in {}, jeez..", input));

    content.split_whitespace().collect()
}
