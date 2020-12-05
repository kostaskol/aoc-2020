use std::fs;

pub fn read_lines(input: &str) -> Vec<String> {
    let content = fs::read_to_string(input).unwrap_or_else(|_| panic!("Put something in {}, jeez..", input));

    content.split('\n').map(|s| s.to_string()).collect()
}
