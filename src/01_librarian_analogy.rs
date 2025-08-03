// 
//

fn main() {
    println!("🦀 Welcome to Rust Lifetimes: The Librarian Analogy!");
    println!();
    
    let book_title = String::from("The Rust Programming Language");
    
    let borrowed_book = &book_title;
    
    println!("📖 I borrowed: {}", borrowed_book);
    
    
    println!();
    println!("🔍 Key Insight:");
    println!("   - book_title is the 'owner' of the data");
    println!("   - borrowed_book is a 'reference' (borrow) to that data");
    println!("   - The borrow must not outlive the owner!");
    
    
} // Both book_title and borrowed_book go out of scope here

// fn problematic_function() -> &str {
//     let temp_string = String::from("This won't work!");
//     &temp_string // ❌ ERROR: temp_string will be dropped when function ends!
// }


#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_borrowing() {
        let data = String::from("Hello, Rust!");
        let reference = &data;
        
        assert_eq!(reference, "Hello, Rust!");
        
        println!("Data: {}, Reference: {}", data, reference);
    }
    
    #[test]
    fn test_multiple_borrows() {
        let book = String::from("Rust in Action");
        
        let reader1 = &book;
        let reader2 = &book;
        let reader3 = &book;
        
        println!("Reader 1: {}", reader1);
        println!("Reader 2: {}", reader2);
        println!("Reader 3: {}", reader3);
        
        assert_eq!(reader1, reader2);
        assert_eq!(reader2, reader3);
    }
}
