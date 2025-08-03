//

use std::collections::HashMap;

fn main() {
    println!("🦀 Advanced Lifetime Examples");
    println!();
    
    demonstrate_nested_lifetimes();
    demonstrate_lifetime_bounds();
    demonstrate_self_referential_patterns();
}

fn demonstrate_nested_lifetimes() {
    println!("🔗 Nested Lifetime Relationships");
    println!();
    
    let document = "Chapter 1: Introduction\nChapter 2: Basics\nChapter 3: Advanced";
    let mut book = Book::new("Rust Guide", document);
    
    let chapter = book.get_chapter(0);
    match chapter {
        Some(ch) => {
            println!("First chapter: {}", ch.get_title());
            println!("Content preview: {}", ch.get_preview(20));
        }
        None => println!("No chapters found"),
    }
    
    book.add_bookmark("Important section", 15);
    let bookmarks = book.get_bookmarks();
    println!("Bookmarks: {:?}", bookmarks);
    println!();
}

struct Book<'a> {
    title: &'a str,
    content: &'a str,
    chapters: Vec<Chapter<'a>>,
    bookmarks: Vec<Bookmark<'a>>,
}

struct Chapter<'a> {
    title: &'a str,
    content: &'a str,
    start_pos: usize,
}

#[derive(Debug)]
struct Bookmark<'a> {
    name: &'a str,
    position: usize,
}

impl<'a> Book<'a> {
    fn new(title: &'a str, content: &'a str) -> Self {
        let chapters = Self::parse_chapters(content);
        Book {
            title,
            content,
            chapters,
            bookmarks: Vec::new(),
        }
    }
    
    fn parse_chapters(content: &'a str) -> Vec<Chapter<'a>> {
        let mut chapters = Vec::new();
        let mut start_pos = 0;
        
        for line in content.lines() {
            if line.starts_with("Chapter") {
                if let Some(colon_pos) = line.find(':') {
                    let title = &line[..colon_pos + 1];
                    let chapter_content = &line[colon_pos + 1..].trim();
                    chapters.push(Chapter {
                        title,
                        content: chapter_content,
                        start_pos,
                    });
                }
            }
            start_pos += line.len() + 1; // +1 for newline
        }
        
        chapters
    }
    
    fn get_chapter(&self, index: usize) -> Option<&Chapter<'a>> {
        self.chapters.get(index)
    }
    
    fn add_bookmark(&mut self, name: &'a str, position: usize) {
        self.bookmarks.push(Bookmark { name, position });
    }
    
    fn get_bookmarks(&self) -> &[Bookmark<'a>] {
        &self.bookmarks
    }
}

impl<'a> Chapter<'a> {
    fn get_title(&self) -> &'a str {
        self.title
    }
    
    fn get_preview(&self, length: usize) -> &'a str {
        if self.content.len() <= length {
            self.content
        } else {
            &self.content[..length]
        }
    }
}

fn demonstrate_lifetime_bounds() {
    println!("🎯 Lifetime Bounds with Generics");
    println!();
    
    let data = vec!["apple", "banana", "cherry"];
    let processor = DataProcessor::new(&data);
    
    let result = processor.process_with_filter(|item| item.len() > 5);
    println!("Filtered items: {:?}", result);
    
    let combined = processor.combine_items(" | ");
    println!("Combined: {}", combined);
    println!();
}

struct DataProcessor<'a, T> 
where 
    T: std::fmt::Display + 'a,
{
    data: &'a [T],
}

impl<'a, T> DataProcessor<'a, T> 
where 
    T: std::fmt::Display + 'a,
{
    fn new(data: &'a [T]) -> Self {
        DataProcessor { data }
    }
    
    fn process_with_filter<F>(&self, filter: F) -> Vec<&'a T>
    where
        F: Fn(&T) -> bool,
    {
        self.data.iter().filter(|item| filter(item)).collect()
    }
    
    fn combine_items(&self, separator: &str) -> String {
        self.data
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join(separator)
    }
    
    fn get_first(&self) -> Option<&'a T> {
        self.data.first()
    }
}

fn demonstrate_self_referential_patterns() {
    println!("🔄 Self-Referential Patterns");
    println!();
    
    let mut graph = Graph::new();
    let node1 = graph.add_node("Node 1");
    let node2 = graph.add_node("Node 2");
    let node3 = graph.add_node("Node 3");
    
    graph.add_edge(node1, node2);
    graph.add_edge(node2, node3);
    graph.add_edge(node1, node3);
    
    println!("Graph nodes: {}", graph.node_count());
    println!("Graph edges: {}", graph.edge_count());
    
    if let Some(neighbors) = graph.get_neighbors(node1) {
        println!("Node 1 neighbors: {:?}", neighbors);
    }
    println!();
}

#[derive(Debug)]
struct Graph<'a> {
    nodes: Vec<Node<'a>>,
    edges: Vec<Edge>,
}

#[derive(Debug)]
struct Node<'a> {
    id: usize,
    data: &'a str,
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    fn add_node(&mut self, data: &'a str) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node { id, data });
        id
    }
    
    fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.edges.push(Edge { from, to });
        }
    }
    
    fn get_neighbors(&self, node_id: usize) -> Option<Vec<usize>> {
        if node_id >= self.nodes.len() {
            return None;
        }
        
        let neighbors: Vec<usize> = self.edges
            .iter()
            .filter_map(|edge| {
                if edge.from == node_id {
                    Some(edge.to)
                } else if edge.to == node_id {
                    Some(edge.from)
                } else {
                    None
                }
            })
            .collect();
        
        Some(neighbors)
    }
    
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

trait Processor<'a> {
    fn process(&self, input: &'a str) -> &'a str;
}

struct UppercaseProcessor;
struct TrimProcessor;

impl<'a> Processor<'a> for UppercaseProcessor {
    fn process(&self, input: &'a str) -> &'a str {
        input
    }
}

impl<'a> Processor<'a> for TrimProcessor {
    fn process(&self, input: &'a str) -> &'a str {
        input.trim()
    }
}

fn process_with_trait<'a>(input: &'a str, processor: &dyn Processor<'a>) -> &'a str {
    processor.process(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_book_chapters() {
        let content = "Chapter 1: Introduction\nChapter 2: Basics\nChapter 3: Advanced";
        let book = Book::new("Test Book", content);
        
        assert_eq!(book.chapters.len(), 3);
        
        let first_chapter = book.get_chapter(0).unwrap();
        assert_eq!(first_chapter.get_title(), "Chapter 1:");
    }
    
    #[test]
    fn test_data_processor() {
        let data = vec!["short", "medium", "very long string"];
        let processor = DataProcessor::new(&data);
        
        let long_items = processor.process_with_filter(|item| item.len() > 6);
        assert_eq!(long_items.len(), 1);
        assert_eq!(long_items[0], &"very long string");
        
        let combined = processor.combine_items(" - ");
        assert!(combined.contains("short"));
        assert!(combined.contains("medium"));
        assert!(combined.contains("very long string"));
    }
    
    #[test]
    fn test_graph() {
        let mut graph = Graph::new();
        let node1 = graph.add_node("A");
        let node2 = graph.add_node("B");
        let node3 = graph.add_node("C");
        
        graph.add_edge(node1, node2);
        graph.add_edge(node2, node3);
        
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        
        let neighbors = graph.get_neighbors(node2).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&node1));
        assert!(neighbors.contains(&node3));
    }
    
    #[test]
    fn test_trait_objects() {
        let input = "  Hello, World!  ";
        let trim_processor = TrimProcessor;
        
        let result = process_with_trait(input, &trim_processor);
        assert_eq!(result, "Hello, World!");
    }
}
