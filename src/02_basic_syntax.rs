//

fn main() {
    println!("🦀 Basic Lifetime Syntax");
    println!();
    
    let string1 = String::from("Hello");
    let string2 = String::from("World!");
    
    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
    
    demonstrate_lifetime_syntax();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn demonstrate_lifetime_syntax() {
    println!("📝 Lifetime Syntax Breakdown:");
    println!("   <'a>        - Declares lifetime parameter 'a");
    println!("   &'a str     - Reference with lifetime 'a");
    println!("   -> &'a str  - Return type with lifetime 'a");
    println!();
    
    let text1 = "Short";
    let text2 = "This is a longer string";
    
    let result = longest_verbose(&text1, &text2);
    println!("Longest (verbose): {}", result);
}

fn longest_verbose<'text_lifetime>(
    x: &'text_lifetime str, 
    y: &'text_lifetime str
) -> &'text_lifetime str {
    if x.len() > y.len() { x } else { y }
}

fn first_word_or_default<'a, 'b>(text: &'a str, _default: &'b str) -> &'a str {
    if text.is_empty() {
        text // Empty string with correct lifetime
    } else {
        text.split_whitespace().next().unwrap_or(text)
    }
}

fn first_word_or_default_fixed<'a>(text: &'a str, default: &'a str) -> &'a str {
    if text.is_empty() {
        default
    } else {
        text.split_whitespace().next().unwrap_or(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longest_function() {
        let short = "Hi";
        let long = "Hello, World!";
        
        assert_eq!(longest(short, long), "Hello, World!");
        assert_eq!(longest(long, short), "Hello, World!");
    }
    
    #[test]
    fn test_lifetime_relationships() {
        let string1 = String::from("abcd");
        let result;
        
        {
            let string2 = String::from("xyz");
            result = longest(&string1, &string2);
            assert!(result.len() >= 3);
        } // string2 goes out of scope here
        
    }
    
    #[test]
    fn test_first_word_function() {
        let text = "Hello world from Rust";
        let default = "default";
        
        let result = first_word_or_default_fixed(text, default);
        assert_eq!(result, "Hello");
        
        let empty_text = "";
        let result2 = first_word_or_default_fixed(empty_text, default);
        assert_eq!(result2, "default");
    }
}
