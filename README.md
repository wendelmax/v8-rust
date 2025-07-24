# V8-Rust: JavaScript Engine in Rust

## Visão Geral
V8-Rust é uma engine JavaScript inspirada na arquitetura do V8 do Google, implementada em Rust. O projeto cobre todas as etapas do pipeline de execução de JavaScript moderno: Lexer, Parser, AST, Análise Semântica, Geração de Bytecode e (futuramente) Máquina Virtual e JIT.

## Componentes
- **Lexer**: Tokenização completa ECMAScript
- **AST**: Árvore sintática completa, compatível com ES2015+
- **Parser**: Parsing robusto, com recuperação de erros
- **Análise Semântica**: Sistema de tipos, escopos e validação ECMAScript
- **Bytecode**: Geração de bytecode 100% compatível com o AST real, cobrindo todas as features ECMAScript
- **Testes**: Cobertura total para todos os módulos

## v8_bytecode
O módulo `v8_bytecode` gera bytecode a partir do AST real, cobrindo 100% das variantes do enum Node. Todos os tipos de nó ECMAScript são suportados, e há testes para cada caso.

### Exemplo de uso
```rust
use v8_ast::Node;
use v8_bytecode::generator::BytecodeGenerator;

let ast = Node::Number(42.0);
let mut gen = BytecodeGenerator::new();
gen.generate(&ast);
assert_eq!(gen.instructions.len(), 1);
```

### Arquitetura
- Enum de instruções inspirado no Ignition (V8)
- Pool de constantes
- Suporte a controle de fluxo, funções, objetos, arrays, operadores modernos, etc.
- Pronto para integração com VM e JIT

## Status Atual
- Lexer, AST, Parser, Análise Semântica: 100% completos
- **Bytecode: 100% completo, com cobertura total do AST real e testes para todos os tipos de nó**
- Pronto para iniciar a implementação da Máquina Virtual (VM)

## Próximos Passos
- Implementação da VM para execução do bytecode
- Otimizações e profiling
- Integração com JIT e garbage collection

## Como rodar os testes
```bash
cargo test --all
``` 