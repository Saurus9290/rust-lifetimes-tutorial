//

fn main() {
    println!("🦀 Lifetime Elision");
    println!();
    
    demonstrate_elision_rules();
    demonstrate_when_elision_works();
    demonstrate_when_elision_fails();
}

fn demonstrate_elision_rules() {
    println!("📋 Lifetime Elision Rules");
    println!();
    println!("Rule 1: Each parameter gets its own lifetime");
    println!("Rule 2: If there's exactly one input lifetime, it's assigned to all outputs");
    println!("Rule 3: If there's a &self or &mut self, its lifetime is assigned to all outputs");
    println!();
    
    let text = "Hello, Rust world!";
    
    let first = first_word(text);
    println!("First word: {}", first);
    
    let last = last_word(text);
    println!("Last word: {}", last);
    
    let numbers = vec![1, 2, 3, 4, 5];
    let first_num = first_element(&numbers);
    match first_num {
        Some(n) => println!("First number: {}", n),
        None => println!("Empty slice"),
    }
    println!();
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn last_word(s: &str) -> &str {
    s.split_whitespace().last().unwrap_or("")
}

fn first_element<T>(slice: &[T]) -> Option<&T> {
    slice.first()
}

fn demonstrate_when_elision_works() {
    println!("✅ When Lifetime Elision Works");
    println!();
    
    let text = "The quick brown fox jumps over the lazy dog";
    
    let length = get_length(text);
    println!("Text length: {}", length);
    
    let trimmed = trim_whitespace("  spaced out  ");
    println!("Trimmed: '{}'", trimmed);
    
    let analyzer = TextAnalyzer::new(text);
    let word_count = analyzer.word_count();
    println!("Word count: {}", word_count);
    
    let first_char = analyzer.first_char();
    match first_char {
        Some(c) => println!("First character: {}", c),
        None => println!("Empty text"),
    }
    println!();
}

fn get_length(s: &str) -> usize {
    s.len()
}

fn trim_whitespace(s: &str) -> &str {
    s.trim()
}

struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }
    
    fn get_text(&self) -> &str {
        self.text
    }
    
    fn first_line(&self) -> &str {
        self.text.lines().next().unwrap_or("")
    }
    
    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
    
    fn first_char(&self) -> Option<char> {
        self.text.chars().next()
    }
    
    fn starts_with_word(&self, word: &str) -> bool {
        self.text.starts_with(word)
    }
}

fn demonstrate_when_elision_fails() {
    println!("❌ When Lifetime Elision Fails");
    println!();
    println!("These cases require explicit lifetime annotations:");
    println!("1. Multiple input lifetimes without &self");
    println!("2. No clear relationship between input and output");
    println!("3. Complex lifetime relationships");
    println!();
    
    let s1 = "Hello";
    let s2 = "World";
    
    let longer = longest_explicit(s1, s2);
    println!("Longer string: {}", longer);
    
    let combined = combine_strings(s1, s2);
    println!("Combined: {}", combined);
    println!();
}

fn longest_explicit<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn combine_strings(x: &str, y: &str) -> String {
    format!("{} {}", x, y)
}

struct Container<'a> {
    data: &'a str,
}

impl<'a> Container<'a> {
    fn get_data(&self) -> &str {
        self.data
    }
    
    fn compare(&self, other: &str) -> bool {
        self.data == other
    }
    
}

fn demonstrate_elision_rules_detailed() {
    println!("🔍 Detailed Elision Rules");
    println!();
    
    
    
}

trait Extractable {
    fn extract(&self) -> &str;
    
}

impl Extractable for String {
    fn extract(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_elision_functions() {
        let text = "Hello world from Rust";
        
        assert_eq!(first_word(text), "Hello");
        assert_eq!(last_word(text), "Rust");
        assert_eq!(get_length(text), 21);
        assert_eq!(trim_whitespace("  test  "), "test");
    }
    
    #[test]
    fn test_text_analyzer() {
        let text = "Line 1\nLine 2\nLine 3";
        let analyzer = TextAnalyzer::new(text);
        
        assert_eq!(analyzer.get_text(), text);
        assert_eq!(analyzer.first_line(), "Line 1");
        assert_eq!(analyzer.word_count(), 6);
        assert_eq!(analyzer.first_char(), Some('L'));
        assert!(analyzer.starts_with_word("Line"));
        assert!(!analyzer.starts_with_word("Word"));
    }
    
    #[test]
    fn test_container() {
        let data = "container data";
        let container = Container::new(data);
        
        assert_eq!(container.get_data(), data);
        assert!(container.compare(data));
        assert!(!container.compare("different"));
    }
    
    #[test]
    fn test_explicit_lifetimes() {
        let short = "Hi";
        let long = "Hello, World!";
        
        assert_eq!(longest_explicit(short, long), long);
        assert_eq!(longest_explicit(long, short), long);
        
        let combined = combine_strings(short, long);
        assert_eq!(combined, "Hi Hello, World!");
    }
    
    #[test]
    fn test_first_element() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(first_element(&numbers), Some(&1));
        
        let empty: &[i32] = &[];
        assert_eq!(first_element(empty), None);
        
        let strings = ["hello", "world"];
        assert_eq!(first_element(&strings), Some(&"hello"));
    }
}

impl<'a> Container<'a> {
    fn new(data: &'a str) -> Self {
        Container { data }
    }
}
