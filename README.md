# v8-rust

Uma engine JavaScript moderna, inspirada no [V8 Engine](https://github.com/v8/v8) do Google, porém totalmente escrita em Rust.

## Objetivo
O objetivo do projeto é criar uma engine alternativa ao V8, replicando sua arquitetura, performance e compatibilidade ECMAScript, mas aproveitando a segurança, performance e modernidade do ecossistema Rust.

## Inspiração
Este projeto é fortemente inspirado no [V8 Engine](https://github.com/v8/v8), utilizado no Google Chrome, Node.js e outros ambientes. O V8 é referência mundial em performance e inovação para execução de JavaScript e WebAssembly.

## Componentes principais
- Parser de JavaScript (ECMAScript)
- Geração de AST
- Máquina Virtual (VM) para execução de bytecode
- Compilador JIT
- Garbage Collector
- Compatibilidade ECMAScript
- Otimizações avançadas (inline caching, hidden classes, etc)

## Estrutura do Projeto
```
crates/
  parser/    # Parser e AST
  runtime/   # Execução, contexto, GC
  vm/        # Máquina virtual, bytecode, JIT
  api/       # API pública e bindings
```

## Checklist de Implementação
Consulte o arquivo [V8-Rust-Checklist.md](./V8-Rust-Checklist.md) para acompanhar o progresso e as tarefas necessárias.

## Referências
- [V8 Engine (GitHub)](https://github.com/v8/v8)
- [V8 Docs](https://v8.dev/docs)

---

> Projeto open source, inspirado e motivado pelo poder do V8 e pela robustez do Rust. 