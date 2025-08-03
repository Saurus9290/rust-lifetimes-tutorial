//

fn main() {
    println!("🦀 Common Lifetime Patterns");
    println!();
    
    demonstrate_iterator_patterns();
    demonstrate_string_processing();
    demonstrate_configuration_patterns();
    demonstrate_cache_patterns();
}

fn demonstrate_iterator_patterns() {
    println!("🔄 Iterator Patterns");
    println!();
    
    let data = vec!["apple", "banana", "cherry", "date"];
    
    let long_fruits = filter_long_items(&data, 5);
    println!("Long fruits: {:?}", long_fruits);
    
    let first_with_a = find_first_containing(&data, "a");
    match first_with_a {
        Some(fruit) => println!("First fruit with 'a': {}", fruit),
        None => println!("No fruit contains 'a'"),
    }
    
    let processed = process_items(&data);
    println!("Processed items: {:?}", processed);
    println!();
}

fn filter_long_items<'a>(items: &'a [&str], min_length: usize) -> Vec<&'a str> {
    items
        .iter()
        .filter(|item| item.len() >= min_length)
        .copied()
        .collect()
}

fn find_first_containing<'a>(items: &'a [&str], pattern: &str) -> Option<&'a str> {
    items
        .iter()
        .find(|item| item.contains(pattern))
        .copied()
}

fn process_items<'a>(items: &'a [&str]) -> Vec<&'a str> {
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)  // Take every other item
        .map(|(_, item)| *item)
        .collect()
}

fn demonstrate_string_processing() {
    println!("📝 String Processing Patterns");
    println!();
    
    let text = "Hello, world! Welcome to Rust programming. It's amazing!";
    
    let sentences = split_into_sentences(text);
    println!("Sentences: {:?}", sentences);
    
    let parser = SimpleParser::new(text);
    let words = parser.extract_words();
    println!("Words: {:?}", words);
    
    let trimmed = trim_and_extract(text, 0, 20);
    println!("Trimmed: '{}'", trimmed);
    println!();
}

fn split_into_sentences<'a>(text: &'a str) -> Vec<&'a str> {
    text.split('.')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect()
}

struct SimpleParser<'a> {
    text: &'a str,
}

impl<'a> SimpleParser<'a> {
    fn new(text: &'a str) -> Self {
        SimpleParser { text }
    }
    
    fn extract_words(&self) -> Vec<&'a str> {
        self.text
            .split_whitespace()
            .filter(|word| word.chars().all(|c| c.is_alphabetic()))
            .collect()
    }
    
    fn find_pattern(&self, pattern: &str) -> Vec<&'a str> {
        self.text
            .split_whitespace()
            .filter(|word| word.contains(pattern))
            .collect()
    }
}

fn trim_and_extract<'a>(text: &'a str, start: usize, end: usize) -> &'a str {
    let end = end.min(text.len());
    let start = start.min(end);
    &text[start..end]
}

fn demonstrate_configuration_patterns() {
    println!("⚙️ Configuration Patterns");
    println!();
    
    let config_text = "database_url=localhost:5432\napi_key=secret123\nport=8080";
    let config = Config::parse(config_text);
    
    if let Some(db_url) = config.get("database_url") {
        println!("Database URL: {}", db_url);
    }
    
    if let Some(port) = config.get("port") {
        println!("Port: {}", port);
    }
    println!();
}

struct Config<'a> {
    data: std::collections::HashMap<&'a str, &'a str>,
}

impl<'a> Config<'a> {
    fn parse(text: &'a str) -> Self {
        let mut data = std::collections::HashMap::new();
        
        for line in text.lines() {
            if let Some((key, value)) = line.split_once('=') {
                data.insert(key.trim(), value.trim());
            }
        }
        
        Config { data }
    }
    
    fn get(&self, key: &str) -> Option<&'a str> {
        self.data.get(key).copied()
    }
    
    fn keys(&self) -> Vec<&'a str> {
        self.data.keys().copied().collect()
    }
}

fn demonstrate_cache_patterns() {
    println!("💾 Cache Patterns");
    println!();
    
    let data = vec!["expensive", "computation", "results", "cached"];
    let mut cache = SimpleCache::new();
    
    let result1 = cache.get_or_compute(&data, |items| {
        println!("Computing...");
        items.join(" ")
    });
    println!("Result 1: {}", result1);
    
    let result2 = cache.get_or_compute(&data, |items| {
        println!("This shouldn't print - using cache!");
        items.join(" ")
    });
    println!("Result 2: {}", result2);
    println!();
}

struct SimpleCache {
    cached_result: Option<String>,
}

impl SimpleCache {
    fn new() -> Self {
        SimpleCache {
            cached_result: None,
        }
    }
    
    fn get_or_compute<F>(&mut self, _input: &[&str], compute: F) -> &str
    where
        F: FnOnce(&[&str]) -> String,
    {
        if self.cached_result.is_none() {
            self.cached_result = Some(compute(_input));
        }
        
        self.cached_result.as_ref().unwrap()
    }
    
    fn clear(&mut self) {
        self.cached_result = None;
    }
}

struct QueryBuilder<'a> {
    table: Option<&'a str>,
    conditions: Vec<&'a str>,
    limit: Option<usize>,
}

impl<'a> QueryBuilder<'a> {
    fn new() -> Self {
        QueryBuilder {
            table: None,
            conditions: Vec::new(),
            limit: None,
        }
    }
    
    fn table(mut self, table: &'a str) -> Self {
        self.table = Some(table);
        self
    }
    
    fn where_clause(mut self, condition: &'a str) -> Self {
        self.conditions.push(condition);
        self
    }
    
    fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    fn build(&self) -> String {
        let mut query = String::from("SELECT * FROM ");
        
        if let Some(table) = self.table {
            query.push_str(table);
        } else {
            query.push_str("unknown_table");
        }
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_long_items() {
        let items = vec!["a", "hello", "world", "rust"];
        let long_items = filter_long_items(&items, 4);
        assert_eq!(long_items, vec!["hello", "world", "rust"]);
    }
    
    #[test]
    fn test_simple_parser() {
        let text = "Hello, world! 123 Rust programming.";
        let parser = SimpleParser::new(text);
        
        let words = parser.extract_words();
        assert_eq!(words, vec!["Hello", "world", "Rust", "programming"]);
        
        let rust_words = parser.find_pattern("rust");
        assert_eq!(rust_words, vec!["Rust"]);
    }
    
    #[test]
    fn test_config_parser() {
        let config_text = "host=localhost\nport=8080\nssl=true";
        let config = Config::parse(config_text);
        
        assert_eq!(config.get("host"), Some("localhost"));
        assert_eq!(config.get("port"), Some("8080"));
        assert_eq!(config.get("ssl"), Some("true"));
        assert_eq!(config.get("missing"), None);
    }
    
    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .table("users")
            .where_clause("age > 18")
            .where_clause("active = true")
            .limit(10)
            .build();
        
        assert!(query.contains("SELECT * FROM users"));
        assert!(query.contains("WHERE age > 18 AND active = true"));
        assert!(query.contains("LIMIT 10"));
    }
    
    #[test]
    fn test_simple_cache() {
        let data = vec!["test", "data"];
        let mut cache = SimpleCache::new();
        
        let result1 = cache.get_or_compute(&data, |items| items.join("-"));
        assert_eq!(result1, "test-data");
        
        let result2 = cache.get_or_compute(&data, |_| "different".to_string());
        assert_eq!(result2, "test-data"); // Should return cached value
    }
}
