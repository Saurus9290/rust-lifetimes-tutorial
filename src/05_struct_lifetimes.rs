//

fn main() {
    println!("🦀 Struct Lifetimes");
    println!();
    
    demonstrate_basic_struct_lifetimes();
    demonstrate_multiple_references();
    demonstrate_methods_with_lifetimes();
}

fn demonstrate_basic_struct_lifetimes() {
    println!("📚 Basic Struct with Lifetime");
    println!();
    
    let title = String::from("The Rust Programming Language");
    let book = Book::new(&title);
    
    println!("Book title: {}", book.get_title());
    println!("Title length: {}", book.title_length());
    println!();
}

struct Book<'a> {
    title: &'a str,
}

impl<'a> Book<'a> {
    fn new(title: &'a str) -> Self {
        Book { title }
    }
    
    fn get_title(&self) -> &str {
        self.title
    }
    
    fn title_length(&self) -> usize {
        self.title.len()
    }
}

fn demonstrate_multiple_references() {
    println!("🔗 Struct with Multiple References");
    println!();
    
    let author_name = String::from("Steve Klabnik");
    let book_title = String::from("The Rust Programming Language");
    
    let article = Article::new(&book_title, &author_name);
    article.print_info();
    
    let content = String::from("Rust is a systems programming language...");
    let detailed_article = DetailedArticle::new(&book_title, &author_name, &content);
    detailed_article.print_summary();
    println!();
}

struct Article<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Article<'a> {
    fn new(title: &'a str, author: &'a str) -> Self {
        Article { title, author }
    }
    
    fn print_info(&self) {
        println!("Article: '{}' by {}", self.title, self.author);
    }
}

struct DetailedArticle<'title, 'author, 'content> {
    title: &'title str,
    author: &'author str,
    content: &'content str,
}

impl<'title, 'author, 'content> DetailedArticle<'title, 'author, 'content> {
    fn new(
        title: &'title str,
        author: &'author str,
        content: &'content str,
    ) -> Self {
        DetailedArticle {
            title,
            author,
            content,
        }
    }
    
    fn print_summary(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Content preview: {}...", &self.content[..50.min(self.content.len())]);
    }
}

fn demonstrate_methods_with_lifetimes() {
    println!("🔧 Methods with Lifetime Parameters");
    println!();
    
    let text = "Hello, Rust! Welcome to lifetimes.";
    let parser = TextParser::new(text);
    
    let words = parser.get_words();
    println!("Words: {:?}", words);
    
    let first_sentence = parser.get_first_sentence();
    println!("First sentence: {}", first_sentence);
    
    let word_at_index = parser.get_word_at(2);
    match word_at_index {
        Some(word) => println!("Word at index 2: {}", word),
        None => println!("No word at index 2"),
    }
    println!();
}

struct TextParser<'a> {
    text: &'a str,
}

impl<'a> TextParser<'a> {
    fn new(text: &'a str) -> Self {
        TextParser { text }
    }
    
    fn get_words(&self) -> Vec<&'a str> {
        self.text.split_whitespace().collect()
    }
    
    fn find_substring(&self, pattern: &str) -> Option<&'a str> {
        if let Some(start) = self.text.find(pattern) {
            Some(&self.text[start..start + pattern.len()])
        } else {
            None
        }
    }
    
    fn get_first_sentence(&self) -> &'a str {
        if let Some(end) = self.text.find('.') {
            &self.text[..=end]
        } else {
            self.text
        }
    }
    
    fn get_word_at(&self, index: usize) -> Option<&'a str> {
        self.text.split_whitespace().nth(index)
    }
}

struct MixedData<'a> {
    owned_title: String,
    borrowed_content: &'a str,
    id: u32,
}

impl<'a> MixedData<'a> {
    fn new(title: String, content: &'a str, id: u32) -> Self {
        MixedData {
            owned_title: title,
            borrowed_content: content,
            id,
        }
    }
    
    fn get_title(&self) -> &str {
        &self.owned_title  // Returns reference to owned data
    }
    
    fn get_content(&self) -> &'a str {
        self.borrowed_content  // Returns borrowed reference
    }
    
    fn get_summary(&self) -> String {
        format!("#{}: {} - {}", self.id, self.owned_title, 
                &self.borrowed_content[..20.min(self.borrowed_content.len())])
    }
}

fn create_invalid_book() -> Book<'static> {
    
    Book::new("Static title")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_book_struct() {
        let title = String::from("Rust in Action");
        let book = Book::new(&title);
        
        assert_eq!(book.get_title(), "Rust in Action");
        assert_eq!(book.title_length(), 14);
    }
    
    #[test]
    fn test_article_struct() {
        let title = "Understanding Lifetimes";
        let author = "Rust Developer";
        
        let article = Article::new(title, author);
        assert_eq!(article.title, "Understanding Lifetimes");
        assert_eq!(article.author, "Rust Developer");
    }
    
    #[test]
    fn test_text_parser() {
        let text = "Hello world! This is Rust. Amazing language.";
        let parser = TextParser::new(text);
        
        let words = parser.get_words();
        assert_eq!(words.len(), 7);
        assert_eq!(words[0], "Hello");
        
        let first_sentence = parser.get_first_sentence();
        assert_eq!(first_sentence, "Hello world!");
        
        let word_at_2 = parser.get_word_at(2);
        assert_eq!(word_at_2, Some("This"));
        
        let substring = parser.find_substring("Rust");
        assert_eq!(substring, Some("Rust"));
    }
    
    #[test]
    fn test_mixed_data() {
        let content = "This is some borrowed content that lives long enough";
        let mixed = MixedData::new("Owned Title".to_string(), content, 42);
        
        assert_eq!(mixed.get_title(), "Owned Title");
        assert_eq!(mixed.get_content(), content);
        assert!(mixed.get_summary().contains("Owned Title"));
        assert!(mixed.get_summary().contains("#42"));
    }
    
    #[test]
    fn test_lifetime_constraints() {
        let title = String::from("Test Book");
        let book;
        
        {
            book = Book::new(&title);
            assert_eq!(book.get_title(), "Test Book");
        } // book goes out of scope here, but title is still alive
        
        assert_eq!(title, "Test Book");
    }
}
