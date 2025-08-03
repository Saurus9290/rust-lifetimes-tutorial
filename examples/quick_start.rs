
use rust_lifetimes_tutorial::example_longest;

fn main() {
    println!("🦀 Rust Lifetimes Tutorial - Quick Start");
    println!();
    
    let string1 = "Hello";
    let string2 = "World!";
    let result = example_longest(string1, string2);
    println!("Longest string: '{}'", result);
    
    let short = "Hi";
    let long = "This is a much longer string";
    let longest = example_longest(short, long);
    println!("Between '{}' and '{}', the longest is: '{}'", short, long, longest);
    
    println!();
    println!("🎯 Key Points:");
    println!("   - The function signature tells us about lifetime relationships");
    println!("   - Both parameters and return value share the same lifetime 'a");
    println!("   - This ensures the returned reference is valid as long as both inputs are valid");
    println!();
    println!("📚 Continue with the full tutorial modules:");
    println!("   cargo run --bin librarian_analogy");
    println!("   cargo run --bin basic_syntax");
    println!("   cargo run --bin classic_problem");
    println!("   ... and more!");
}
