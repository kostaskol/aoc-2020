use std::fs;

pub fn read_lines(input: &str) -> Vec<String> {
    let content = fs::read_to_string(input).expect(&format!("Put something in {}, jeez..", input));

    content.split("\n").map(|s| s.to_string()).collect()
}
