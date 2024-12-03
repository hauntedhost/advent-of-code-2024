pub mod template;

pub fn parse_u32_parts<'a>(line: &'a str) -> impl Iterator<Item = u32> + 'a {
    line.split_whitespace()
        .map(|s| s.parse::<u32>().expect("Failed to parse u32"))
}
