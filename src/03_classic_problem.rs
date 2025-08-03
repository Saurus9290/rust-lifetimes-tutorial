//
// 

fn main() {
    println!("🦀 The Classic Lifetime Problem");
    println!();
    
    demonstrate_problem();
    
    demonstrate_solutions();
}

fn demonstrate_problem() {
    println!("❌ The Problem:");
    println!("   fn longest(x: &str, y: &str) -> &str {{");
    println!("       if x.len() > y.len() {{ x }} else {{ y }}");
    println!("   }}");
    println!();
    println!("   Error: missing lifetime specifier");
    println!("   Rust doesn't know if the return value should live as long as x or y!");
    println!();
}

fn demonstrate_solutions() {
    println!("✅ Solution 1: Explicit Lifetimes");
    
    let string1 = String::from("Hello");
    let string2 = String::from("World!");
    
    let result = longest_fixed(&string1, &string2);
    println!("   Longest: {}", result);
    println!();
    
    println!("✅ Solution 2: Return Owned Data");
    let owned_result = longest_owned(&string1, &string2);
    println!("   Longest (owned): {}", owned_result);
    println!();
    
    println!("✅ Solution 3: Use Static Strings");
    let static_result = longest_static("Hello", "World!");
    println!("   Longest (static): {}", static_result);
    println!();
}

fn longest_fixed<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_owned(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string()  // Convert to owned String
    } else {
        y.to_string()
    }
}

fn longest_static(x: &'static str, y: &'static str) -> &'static str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn explain_the_error() {
    println!("🔍 Why does this error happen?");
    println!();
    println!("When Rust sees:");
    println!("   fn longest(x: &str, y: &str) -> &str");
    println!();
    println!("It asks: 'Which lifetime should the return value have?'");
    println!("- Should it live as long as x?");
    println!("- Should it live as long as y?");
    println!("- What if x and y have different lifetimes?");
    println!();
    println!("Without explicit annotations, Rust can't decide!");
    println!("So it asks you to be explicit with lifetime parameters.");
}

fn dangerous_function() -> &'static str {
    
    "I'm a string literal!"
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]  // Return the whole string if no space found
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longest_fixed() {
        let s1 = "Hello";
        let s2 = "World!";
        
        assert_eq!(longest_fixed(s1, s2), "World!");
        assert_eq!(longest_fixed("Rust", "Go"), "Rust");
    }
    
    #[test]
    fn test_longest_owned() {
        let s1 = String::from("Short");
        let s2 = String::from("Much longer string");
        
        let result = longest_owned(&s1, &s2);
        assert_eq!(result, "Much longer string");
        
        drop(s1);
        drop(s2);
        assert_eq!(result, "Much longer string");
    }
    
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("Hello world"), "Hello");
        assert_eq!(first_word("Rust"), "Rust");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("One Two Three"), "One");
    }
    
    #[test]
    fn test_lifetime_constraints() {
        let string1 = String::from("long string is long");
        let result;
        
        {
            let string2 = String::from("xyz");
            result = longest_fixed(&string1, &string2);
            println!("Result in inner scope: {}", result);
        } // string2 is dropped here
        
    }
}
