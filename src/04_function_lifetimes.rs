//

fn main() {
    println!("🦀 Function Lifetimes");
    println!();
    
    demonstrate_basic_function_lifetimes();
    demonstrate_multiple_lifetimes();
    demonstrate_lifetime_relationships();
}

fn demonstrate_basic_function_lifetimes() {
    println!("📝 Basic Function Lifetimes");
    println!();
    
    let text = "Hello, Rust lifetimes!";
    let first = get_first_word(text);
    println!("First word: {}", first);
    
    let numbers = vec![1, 2, 3, 4, 5];
    let first_num = get_first_element(&numbers);
    match first_num {
        Some(n) => println!("First number: {}", n),
        None => println!("Empty vector"),
    }
    println!();
}

fn get_first_word<'a>(text: &'a str) -> &'a str {
    text.split_whitespace().next().unwrap_or("")
}

fn get_first_element<'a, T>(slice: &'a [T]) -> Option<&'a T> {
    slice.first()
}

fn demonstrate_multiple_lifetimes() {
    println!("🔗 Multiple Lifetime Parameters");
    println!();
    
    let name = "Alice";
    let greeting = "Hello";
    let message = create_greeting(greeting, name);
    println!("Message: {}", message);
    
    let text1 = "First part";
    let text2 = "Second part";
    let combined = combine_or_first(text1, text2, true);
    println!("Combined: {}", combined);
    println!();
}

fn create_greeting<'a, 'b>(greeting: &'a str, name: &'b str) -> String {
    format!("{}, {}!", greeting, name)
}

fn combine_or_first<'a, 'b>(first: &'a str, second: &'b str, use_first: bool) -> String {
    if use_first {
        first.to_string()
    } else {
        format!("{} {}", first, second)
    }
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demonstrate_lifetime_relationships() {
    println!("🔍 Lifetime Relationships");
    println!();
    
    let text = "Hello, world!";
    let prefix = get_prefix(text, 5);
    println!("Prefix: {}", prefix);
    
    let s1 = "Short";
    let s2 = "Much longer string";
    let longest = longest_with_announcement(s1, s2, "Finding the longest!");
    println!("Longest: {}", longest);
    
    let words = vec!["hello", "world", "rust"];
    let first = get_first_str(&words);
    match first {
        Some(word) => println!("First word: {}", word),
        None => println!("No words"),
    }
    println!();
}

fn get_prefix<'a>(text: &'a str, len: usize) -> &'a str {
    if text.len() <= len {
        text
    } else {
        &text[..len]
    }
}

fn get_first_str<'a>(words: &'a [&str]) -> Option<&'a str> {
    words.first().copied()
}

struct TextProcessor<'a> {
    text: &'a str,
}

impl<'a> TextProcessor<'a> {
    fn new(text: &'a str) -> Self {
        TextProcessor { text }
    }
    
    fn get_words(&self) -> Vec<&'a str> {
        self.text.split_whitespace().collect()
    }
    
    fn get_first_word(&self) -> Option<&'a str> {
        self.text.split_whitespace().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_first_word() {
        let text = "Hello world from Rust";
        assert_eq!(get_first_word(text), "Hello");
        
        let empty = "";
        assert_eq!(get_first_word(empty), "");
    }
    
    #[test]
    fn test_get_first_element() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(get_first_element(&numbers), Some(&1));
        
        let empty: &[i32] = &[];
        assert_eq!(get_first_element(empty), None);
    }
    
    #[test]
    fn test_get_prefix() {
        let text = "Hello, Rust!";
        assert_eq!(get_prefix(text, 5), "Hello");
        assert_eq!(get_prefix(text, 20), "Hello, Rust!");
        assert_eq!(get_prefix(text, 0), "");
    }
    
    #[test]
    fn test_text_processor() {
        let text = "Rust is awesome and powerful";
        let processor = TextProcessor::new(text);
        
        let words = processor.get_words();
        assert_eq!(words.len(), 5);
        assert_eq!(words[0], "Rust");
        
        let first_word = processor.get_first_word();
        assert_eq!(first_word, Some("Rust"));
    }
    
    #[test]
    fn test_lifetime_constraints() {
        let long_string = String::from("This is a long string");
        let result;
        
        {
            let short_string = String::from("Short");
            result = longest_with_announcement(&long_string, &short_string, "Comparing!");
            assert_eq!(result, "This is a long string");
        } // short_string is dropped here
        
    }
}
