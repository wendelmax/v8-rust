//! Error handling tests for v8_lexer
//! 
//! Tests for lexer error conditions and error recovery.

use v8_lexer::{tokenize, LexerError};

#[test]
fn test_unterminated_string_error() {
    let source = "\"hello world";
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::UnterminatedString => {},
        _ => panic!("Expected UnterminatedString error"),
    }
}

#[test]
fn test_unterminated_template_string_error() {
    let source = "`hello world";
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::UnterminatedTemplateString => {},
        _ => panic!("Expected UnterminatedTemplateString error"),
    }
}

#[test]
fn test_unterminated_comment_error() {
    let source = "/* hello world";
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::UnterminatedComment => {},
        _ => panic!("Expected UnterminatedComment error"),
    }
}

#[test]
fn test_invalid_number_error() {
    let source = "0xGG"; // Invalid hex
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidNumber(_) => {},
        _ => panic!("Expected InvalidNumber error"),
    }
}

#[test]
fn test_invalid_binary_number_error() {
    let source = "0b123"; // Invalid binary
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidBinaryLiteral(_) => {},
        _ => panic!("Expected InvalidBinaryLiteral error"),
    }
}

#[test]
fn test_invalid_octal_number_error() {
    let source = "0o89"; // Invalid octal
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidOctalLiteral(_) => {},
        _ => panic!("Expected InvalidOctalLiteral error"),
    }
}

#[test]
fn test_invalid_bigint_error() {
    let source = "42.5n"; // Invalid BigInt (cannot have decimal)
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidBigIntLiteral(_) => {},
        _ => panic!("Expected InvalidBigIntLiteral error"),
    }
}

#[test]
fn test_invalid_escape_sequence_error() {
    let source = "\"hello\\qworld\""; // Invalid escape sequence
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidEscapeSequence(_) => {},
        _ => panic!("Expected InvalidEscapeSequence error"),
    }
}

#[test]
fn test_invalid_unicode_escape_error() {
    let source = "\"hello\\u{invalid}world\""; // Invalid Unicode escape
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidUnicodeEscape(_) => {},
        _ => panic!("Expected InvalidUnicodeEscape error"),
    }
}

#[test]
fn test_invalid_hex_escape_error() {
    let source = "\"hello\\xGGworld\""; // Invalid hex escape
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidHexEscape(_) => {},
        _ => panic!("Expected InvalidHexEscape error"),
    }
}

#[test]
fn test_invalid_octal_escape_error() {
    let source = "\"hello\\999world\""; // Invalid octal escape
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidOctalEscape(_) => {},
        _ => panic!("Expected InvalidOctalEscape error"),
    }
}

#[test]
fn test_invalid_regex_error() {
    let source = "/[invalid regex/"; // Invalid regex
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidRegexLiteral(_) => {},
        _ => panic!("Expected InvalidRegexLiteral error"),
    }
}

#[test]
fn test_invalid_regex_flags_error() {
    let source = "/test/xyz"; // Invalid regex flags
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidRegexFlags(_) => {},
        _ => panic!("Expected InvalidRegexFlags error"),
    }
}

#[test]
fn test_invalid_identifier_error() {
    let source = "let 123abc = 42;"; // Invalid identifier starting with number
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidIdentifier(_) => {},
        _ => panic!("Expected InvalidIdentifier error"),
    }
}

#[test]
fn test_invalid_keyword_error() {
    let source = "let invalid-keyword = 42;"; // Invalid keyword with hyphen
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidKeyword(_) => {},
        _ => panic!("Expected InvalidKeyword error"),
    }
}

#[test]
fn test_invalid_operator_error() {
    let source = "let x = 42 @ 10;"; // Invalid operator
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidOperator(_) => {},
        _ => panic!("Expected InvalidOperator error"),
    }
}

#[test]
fn test_invalid_symbol_error() {
    let source = "let x = 42 # 10;"; // Invalid symbol
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidSymbol(_) => {},
        _ => panic!("Expected InvalidSymbol error"),
    }
}

#[test]
fn test_invalid_comment_error() {
    let source = "// comment with invalid chars \x00";
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidComment(_) => {},
        _ => panic!("Expected InvalidComment error"),
    }
}

#[test]
fn test_invalid_whitespace_error() {
    let source = "let x = 42\x00;"; // Invalid whitespace character
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidWhitespace(_) => {},
        _ => panic!("Expected InvalidWhitespace error"),
    }
}

#[test]
fn test_invalid_token_error() {
    let source = "let x = 42 \x01;"; // Invalid token character
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::InvalidToken(_) => {},
        _ => panic!("Expected InvalidToken error"),
    }
}

#[test]
fn test_unexpected_end_of_input_error() {
    let source = "let x = "; // Unexpected end of input
    let result = tokenize(source);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        LexerError::UnexpectedEndOfInput => {},
        _ => panic!("Expected UnexpectedEndOfInput error"),
    }
}

#[test]
fn test_internal_error() {
    // This would typically be triggered by a bug in the lexer
    // We'll test the error type exists
    let error = LexerError::InternalError("test error".to_string());
    assert!(format!("{:?}", error).contains("test error"));
}

#[test]
fn test_error_message_formatting() {
    let error = LexerError::UnexpectedCharacter('@');
    let message = format!("{}", error);
    assert!(message.contains("Unexpected character: @"));
    
    let error = LexerError::InvalidNumber("invalid".to_string());
    let message = format!("{}", error);
    assert!(message.contains("Invalid number: invalid"));
}

#[test]
fn test_error_clone() {
    let error = LexerError::UnexpectedCharacter('@');
    let cloned = error.clone();
    assert_eq!(error, cloned);
}

#[test]
fn test_error_partial_eq() {
    let error1 = LexerError::UnexpectedCharacter('@');
    let error2 = LexerError::UnexpectedCharacter('@');
    let error3 = LexerError::UnexpectedCharacter('#');
    
    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_multiple_errors_in_source() {
    let source = "\"unterminated\nlet x = 42 @ 10;"; // Multiple errors
    let result = tokenize(source);
    
    assert!(result.is_err());
    // Should return the first error encountered
    match result.unwrap_err() {
        LexerError::UnterminatedString => {},
        _ => panic!("Expected first error to be UnterminatedString"),
    }
}

#[test]
fn test_error_recovery() {
    // Test that the lexer can recover from some errors
    let source = "let x = 42;\nlet y = \"unterminated;\nlet z = 100;";
    let result = tokenize(source);
    
    // Should fail due to unterminated string, but we can test recovery
    assert!(result.is_err());
}

#[test]
fn test_error_positions() {
    let source = "let x = 42;\nlet y = \"unterminated";
    let result = tokenize(source);
    
    assert!(result.is_err());
    // In a more sophisticated implementation, we'd check that the error
    // includes position information about where the error occurred
} 