//! Criterion benchmarks for v8_lexer
//! 
//! This module contains performance benchmarks for the lexer using Criterion.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use v8_lexer::tokenize;

fn bench_simple_tokenization(c: &mut Criterion) {
    let source = "let x = 42;";
    
    c.bench_function("simple_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_complex_tokenization(c: &mut Criterion) {
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
    
    c.bench_function("complex_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_large_source_tokenization(c: &mut Criterion) {
    // Generate a large source with many tokens
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("let var{} = {};", i, i));
    }
    
    c.bench_function("large_source_tokenization", |b| {
        b.iter(|| tokenize(black_box(&source)))
    });
}

fn bench_unicode_tokenization(c: &mut Criterion) {
    let source = "let π = 3.14159; let 你好 = 'world'; let 🚀 = 'rocket';";
    
    c.bench_function("unicode_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_whitespace_heavy_tokenization(c: &mut Criterion) {
    let source = "   \t\n   let   x   =   42   ;   \n   ";
    
    c.bench_function("whitespace_heavy_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_comment_heavy_tokenization(c: &mut Criterion) {
    let source = r#"
        // This is a comment
        let x = 42; /* This is another comment */
        /*
         * Multi-line comment
         * with lots of content
         */
        let y = 100;
    "#;
    
    c.bench_function("comment_heavy_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_string_heavy_tokenization(c: &mut Criterion) {
    let source = r#"
        let str1 = "Hello, world!";
        let str2 = "This is a \"quoted\" string";
        let str3 = "String with \n newline";
        let str4 = "String with \t tab";
        let str5 = "String with \\ backslash";
    "#;
    
    c.bench_function("string_heavy_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_number_heavy_tokenization(c: &mut Criterion) {
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
    
    c.bench_function("number_heavy_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_error_handling(c: &mut Criterion) {
    let source = "\"unterminated string";
    
    c.bench_function("error_handling", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_template_literal_tokenization(c: &mut Criterion) {
    let source = "`Hello, ${name}! How are you ${user.name}?`";
    
    c.bench_function("template_literal_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_regex_literal_tokenization(c: &mut Criterion) {
    let source = "/[a-zA-Z]+/g";
    
    c.bench_function("regex_literal_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_arrow_function_tokenization(c: &mut Criterion) {
    let source = "(a, b) => a + b";
    
    c.bench_function("arrow_function_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

fn bench_class_declaration_tokenization(c: &mut Criterion) {
    let source = r#"
        class Person {
            constructor(name) {
                this.name = name;
            }
            
            sayHello() {
                return `Hello, ${this.name}!`;
            }
        }
    "#;
    
    c.bench_function("class_declaration_tokenization", |b| {
        b.iter(|| tokenize(black_box(source)))
    });
}

criterion_group!(
    benches,
    bench_simple_tokenization,
    bench_complex_tokenization,
    bench_large_source_tokenization,
    bench_unicode_tokenization,
    bench_whitespace_heavy_tokenization,
    bench_comment_heavy_tokenization,
    bench_string_heavy_tokenization,
    bench_number_heavy_tokenization,
    bench_error_handling,
    bench_template_literal_tokenization,
    bench_regex_literal_tokenization,
    bench_arrow_function_tokenization,
    bench_class_declaration_tokenization,
);

criterion_main!(benches); 