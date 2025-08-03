

pub fn example_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_longest() {
        assert_eq!(example_longest("short", "longer"), "longer");
        assert_eq!(example_longest("equal", "equal"), "equal");
    }
}
