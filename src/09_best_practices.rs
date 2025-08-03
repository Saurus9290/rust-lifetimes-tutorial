//

fn main() {
    println!("🦀 Lifetime Best Practices");
    println!();
    
    demonstrate_prefer_owned_types();
    demonstrate_lifetime_naming();
    demonstrate_avoiding_complex_lifetimes();
    demonstrate_common_patterns();
}

fn demonstrate_prefer_owned_types() {
    println!("✅ Prefer Owned Types When Possible");
    println!();
    
    
    let input1 = "Hello".to_string();
    let input2 = "World".to_string();
    let result = simple_string_processing(input1, input2);
    println!("Result: {}", result);
    
    let data = vec!["apple", "banana", "cherry"];
    let processed = process_and_own(&data);
    println!("Processed: {:?}", processed);
    println!();
}

fn simple_string_processing(a: String, b: String) -> String {
    format!("{} {}!", a, b)
}

fn process_and_own(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .map(|item| format!("processed_{}", item))
        .collect()
}

fn demonstrate_lifetime_naming() {
    println!("📝 Lifetime Naming Conventions");
    println!();
    
    let text = "Sample text for demonstration";
    let config = "key=value\nother=setting";
    
    let parser = DocumentParser::new(text, config);
    let summary = parser.get_summary();
    println!("Summary: {}", summary);
    
    let result = simple_combine(text, config);
    println!("Combined: {}", result);
    println!();
}

struct DocumentParser<'document, 'config> {
    document: &'document str,
    config: &'config str,
}

impl<'document, 'config> DocumentParser<'document, 'config> {
    fn new(document: &'document str, config: &'config str) -> Self {
        DocumentParser { document, config }
    }
    
    fn get_summary(&self) -> String {
        format!("Document: {} chars, Config: {} lines", 
                self.document.len(), 
                self.config.lines().count())
    }
    
    fn get_document(&self) -> &'document str {
        self.document
    }
    
    fn get_config(&self) -> &'config str {
        self.config
    }
}

fn simple_combine<'a, 'b>(a: &'a str, b: &'b str) -> String {
    format!("{}\n{}", a, b)
}

fn demonstrate_avoiding_complex_lifetimes() {
    println!("🚫 Avoiding Complex Lifetimes");
    println!();
    
    let query = QueryBuilder::new()
        .select("name, age")
        .from("users")
        .where_clause("age > 18")
        .build();
    println!("Query: {}", query);
    
    let mut tree = SimpleTree::new();
    let root = tree.add_node("root");
    let child1 = tree.add_node("child1");
    let child2 = tree.add_node("child2");
    
    tree.add_child(root, child1);
    tree.add_child(root, child2);
    
    println!("Tree has {} nodes", tree.node_count());
    println!();
}

struct QueryBuilder {
    select: Option<String>,
    from: Option<String>,
    where_clause: Option<String>,
}

impl QueryBuilder {
    fn new() -> Self {
        QueryBuilder {
            select: None,
            from: None,
            where_clause: None,
        }
    }
    
    fn select(mut self, fields: &str) -> Self {
        self.select = Some(fields.to_string());
        self
    }
    
    fn from(mut self, table: &str) -> Self {
        self.from = Some(table.to_string());
        self
    }
    
    fn where_clause(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }
    
    fn build(self) -> String {
        let mut query = String::from("SELECT ");
        query.push_str(&self.select.unwrap_or_else(|| "*".to_string()));
        query.push_str(" FROM ");
        query.push_str(&self.from.unwrap_or_else(|| "table".to_string()));
        
        if let Some(where_clause) = self.where_clause {
            query.push_str(" WHERE ");
            query.push_str(&where_clause);
        }
        
        query
    }
}

struct SimpleTree {
    nodes: Vec<TreeNode>,
}

struct TreeNode {
    data: String,
    children: Vec<usize>,
}

impl SimpleTree {
    fn new() -> Self {
        SimpleTree { nodes: Vec::new() }
    }
    
    fn add_node(&mut self, data: &str) -> usize {
        let index = self.nodes.len();
        self.nodes.push(TreeNode {
            data: data.to_string(),
            children: Vec::new(),
        });
        index
    }
    
    fn add_child(&mut self, parent: usize, child: usize) {
        if parent < self.nodes.len() && child < self.nodes.len() {
            self.nodes[parent].children.push(child);
        }
    }
    
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

fn demonstrate_common_patterns() {
    println!("🔄 Common Lifetime Patterns");
    println!();
    
    let config_text = "debug=true\nport=8080\nhost=localhost";
    let config = AppConfig::new(config_text);
    println!("Debug mode: {}", config.is_debug());
    
    let text = "Hello, world! This is a test.";
    let processor = TextProcessor::new();
    let words = processor.extract_words(text);
    println!("Words: {:?}", words);
    
    let mut cache = StringCache::new();
    let result1 = cache.get_or_compute("key1", || "computed value".to_string());
    println!("Cached result: {}", result1);
    
    let result2 = cache.get_or_compute("key1", || "this won't be computed".to_string());
    println!("From cache: {}", result2);
    println!();
}

struct AppConfig<'a> {
    raw_config: &'a str,
    parsed: std::collections::HashMap<&'a str, &'a str>,
}

impl<'a> AppConfig<'a> {
    fn new(config_text: &'a str) -> Self {
        let mut parsed = std::collections::HashMap::new();
        
        for line in config_text.lines() {
            if let Some((key, value)) = line.split_once('=') {
                parsed.insert(key.trim(), value.trim());
            }
        }
        
        AppConfig {
            raw_config: config_text,
            parsed,
        }
    }
    
    fn get(&self, key: &str) -> Option<&'a str> {
        self.parsed.get(key).copied()
    }
    
    fn is_debug(&self) -> bool {
        self.get("debug").unwrap_or("false") == "true"
    }
}

struct TextProcessor;

impl TextProcessor {
    fn new() -> Self {
        TextProcessor
    }
    
    fn extract_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .filter(|word| word.chars().all(|c| c.is_alphabetic()))
            .map(|word| word.to_lowercase())
            .collect()
    }
    
    fn count_characters(&self, text: &str) -> std::collections::HashMap<char, usize> {
        let mut counts = std::collections::HashMap::new();
        for ch in text.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        counts
    }
}

struct StringCache {
    cache: std::collections::HashMap<String, String>,
}

impl StringCache {
    fn new() -> Self {
        StringCache {
            cache: std::collections::HashMap::new(),
        }
    }
    
    fn get_or_compute<F>(&mut self, key: &str, compute: F) -> &str
    where
        F: FnOnce() -> String,
    {
        if !self.cache.contains_key(key) {
            let value = compute();
            self.cache.insert(key.to_string(), value);
        }
        
        self.cache.get(key).unwrap()
    }
    
    fn clear(&mut self) {
        self.cache.clear();
    }
}

fn print_best_practices() {
    println!("📋 Lifetime Best Practices Summary:");
    println!();
    println!("1. ✅ Prefer owned types (String, Vec<T>) over references when possible");
    println!("2. ✅ Use lifetime elision - let Rust infer lifetimes when it can");
    println!("3. ✅ Use descriptive lifetime names for complex relationships");
    println!("4. ✅ Keep functions simple - avoid multiple lifetime parameters");
    println!("5. ✅ Use builder patterns to avoid complex lifetime relationships");
    println!("6. ✅ Use indices instead of references for self-referential data");
    println!("7. ✅ Return owned data from functions when processing is involved");
    println!("8. ✅ Use 'static lifetime sparingly - only for truly static data");
    println!("9. ✅ Trust the borrow checker - it's preventing real bugs");
    println!("10. ✅ When in doubt, clone() - performance can be optimized later");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_owned_types() {
        let result = simple_string_processing("Hello".to_string(), "World".to_string());
        assert_eq!(result, "Hello World!");
        
        let items = vec!["a", "b", "c"];
        let processed = process_and_own(&items);
        assert_eq!(processed, vec!["processed_a", "processed_b", "processed_c"]);
    }
    
    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .select("id, name")
            .from("users")
            .where_clause("active = true")
            .build();
        
        assert!(query.contains("SELECT id, name"));
        assert!(query.contains("FROM users"));
        assert!(query.contains("WHERE active = true"));
    }
    
    #[test]
    fn test_simple_tree() {
        let mut tree = SimpleTree::new();
        let root = tree.add_node("root");
        let child = tree.add_node("child");
        
        tree.add_child(root, child);
        assert_eq!(tree.node_count(), 2);
    }
    
    #[test]
    fn test_app_config() {
        let config_text = "debug=true\nport=8080\nhost=localhost";
        let config = AppConfig::new(config_text);
        
        assert_eq!(config.get("debug"), Some("true"));
        assert_eq!(config.get("port"), Some("8080"));
        assert_eq!(config.get("host"), Some("localhost"));
        assert!(config.is_debug());
    }
    
    #[test]
    fn test_text_processor() {
        let processor = TextProcessor::new();
        let text = "Hello, World! 123 Test.";
        
        let words = processor.extract_words(text);
        assert_eq!(words, vec!["hello", "world", "test"]);
        
        let char_counts = processor.count_characters("hello");
        assert_eq!(char_counts.get(&'l'), Some(&2));
        assert_eq!(char_counts.get(&'h'), Some(&1));
    }
    
    #[test]
    fn test_string_cache() {
        let mut cache = StringCache::new();
        
        let result1 = cache.get_or_compute("test", || "computed".to_string());
        assert_eq!(result1, "computed");
        
        let result2 = cache.get_or_compute("test", || "different".to_string());
        assert_eq!(result2, "computed"); // Should return cached value
    }
}
