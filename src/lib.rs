pub mod template;

pub fn line_into_u32_iter<'a>(line: &'a str) -> impl Iterator<Item = u32> + 'a {
    line.split_whitespace().map(parse_u32)
}

pub fn parse_u32(s: &str) -> u32 {
    s.parse::<u32>().expect("Failed to parse u32")
}
