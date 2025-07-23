//! Benchmark tests for v8_lexer
//! 
//! Tests for lexer performance and benchmarking.

use v8_lexer::tokenize;
use std::time::Instant;

#[test]
fn test_lexer_performance_simple() {
    let source = "let x = 42;";
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _tokens = tokenize(source).unwrap();
    }
    
    let duration = start.elapsed();
    println!("Simple lexing took: {:?} for 1000 iterations", duration);
    
    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_performance_complex() {
    let source = r#"
        function fibonacci(n) {
            if (n <= 1) {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        let result = fibonacci(10);
        console.log(`Fibonacci of 10 is ${result}`);
    "#;
    
    let start = Instant::now();
    
    for _ in 0..100 {
        let _tokens = tokenize(source).unwrap();
    }
    
    let duration = start.elapsed();
    println!("Complex lexing took: {:?} for 100 iterations", duration);
    
    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_performance_large_source() {
    // Generate a large source with many tokens
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("let var{} = {};", i, i));
    }
    
    let start = Instant::now();
    let tokens = tokenize(&source).unwrap();
    let duration = start.elapsed();
    
    println!("Large source lexing took: {:?} for {} tokens", duration, tokens.len());
    
    // Should have many tokens
    assert!(tokens.len() > 3000); // At least 3 tokens per line (let, identifier, number, semicolon)
    
    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_memory_usage() {
    let source = "let x = 42;";
    let tokens = tokenize(source).unwrap();
    
    // Check that tokens don't consume excessive memory
    let token_count = tokens.len();
    let estimated_memory = token_count * std::mem::size_of::<v8_lexer::Token>();
    
    println!("Estimated memory usage: {} bytes for {} tokens", estimated_memory, token_count);
    
    // Should be reasonable memory usage (less than 1MB for small source)
    assert!(estimated_memory < 1_000_000);
}

#[test]
fn test_lexer_concurrent_performance() {
    use std::thread;
    use std::sync::Arc;
    
    let source = Arc::new("let x = 42; let y = 100; let z = x + y;".to_string());
    let mut handles = vec![];
    
    let start = Instant::now();
    
    for _ in 0..10 {
        let source_clone = Arc::clone(&source);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let _tokens = tokenize(&source_clone).unwrap();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("Concurrent lexing took: {:?} for 10 threads, 100 iterations each", duration);
    
    // Should complete in reasonable time (less than 2 seconds)
    assert!(duration.as_millis() < 2000);
}

#[test]
fn test_lexer_error_performance() {
    let source = "\"unterminated string";
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _result = tokenize(source);
    }
    
    let duration = start.elapsed();
    println!("Error handling took: {:?} for 1000 iterations", duration);
    
    // Should complete quickly even with errors
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_unicode_performance() {
    let source = "let π = 3.14159; let 你好 = 'world'; let 🚀 = 'rocket';";
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _tokens = tokenize(source).unwrap();
    }
    
    let duration = start.elapsed();
    println!("Unicode lexing took: {:?} for 1000 iterations", duration);
    
    // Should handle Unicode efficiently
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_whitespace_performance() {
    let source = "   \t\n   let   x   =   42   ;   \n   ";
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _tokens = tokenize(source).unwrap();
    }
    
    let duration = start.elapsed();
    println!("Whitespace-heavy lexing took: {:?} for 1000 iterations", duration);
    
    // Should handle whitespace efficiently
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_comment_performance() {
    let source = r#"
        // This is a comment
        let x = 42; /* This is another comment */
        /*
         * Multi-line comment
         * with lots of content
         */
        let y = 100;
    "#;
    
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _tokens = tokenize(source).unwrap();
    }
    
    let duration = start.elapsed();
    println!("Comment-heavy lexing took: {:?} for 1000 iterations", duration);
    
    // Should handle comments efficiently
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_string_performance() {
    let source = r#"
        let str1 = "Hello, world!";
        let str2 = "This is a \"quoted\" string";
        let str3 = "String with \n newline";
        let str4 = "String with \t tab";
        let str5 = "String with \\ backslash";
    "#;
    
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _tokens = tokenize(source).unwrap();
    }
    
    let duration = start.elapsed();
    println!("String-heavy lexing took: {:?} for 1000 iterations", duration);
    
    // Should handle strings efficiently
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_number_performance() {
    let source = r#"
        let int1 = 42;
        let float1 = 3.14159;
        let hex1 = 0xFF;
        let binary1 = 0b1010;
        let octal1 = 0o755;
        let bigint1 = 42n;
        let exp1 = 1.23e-4;
        let exp2 = 1.23E+4;
    "#;
    
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _tokens = tokenize(source).unwrap();
    }
    
    let duration = start.elapsed();
    println!("Number-heavy lexing took: {:?} for 1000 iterations", duration);
    
    // Should handle numbers efficiently
    assert!(duration.as_millis() < 1000);
} 