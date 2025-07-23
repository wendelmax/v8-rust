# V8-Rust

Uma engine JavaScript moderna, inspirada no [V8 Engine](https://github.com/v8/v8) do Google, porém totalmente escrita em Rust.

## Objetivo
O objetivo do projeto é criar uma engine alternativa ao V8, replicando sua arquitetura, performance e compatibilidade ECMAScript, mas aproveitando a segurança, performance e modernidade do ecossistema Rust.

## Inspiração
Este projeto é fortemente inspirado no [V8 Engine](https://github.com/v8/v8) e no [Boa Engine](https://github.com/boa-dev/boa), utilizando as melhores práticas de ambos para criar uma engine JavaScript robusta e modular.

## Arquitetura Modular

O projeto segue uma arquitetura modular inspirada no Boa Engine, com crates separados para cada componente principal:

```
v8-rust/
├── crates/
│   ├── v8_lexer/          # Lexer independente com posicionamento preciso
│   ├── v8_ast/            # AST completo com visitor pattern
│   ├── v8_parser/         # Parser que usa lexer + ast
│   ├── v8_runtime/        # Runtime e contexto de execução
│   ├── v8_vm/             # Máquina virtual e bytecode
│   ├── v8_gc/             # Garbage Collector
│   └── v8_api/            # API pública e CLI
```

### Componentes

#### **v8_lexer**
- Lexer robusto com posicionamento preciso (linha/coluna)
- Suporte completo a Unicode
- Sistema de erros detalhado
- Tokens com spans para melhor debugging

#### **v8_ast** 
- AST completo com todos os nós ECMAScript
- Serialização/deserialização com Serde
- Visitor pattern para traversão
- Suporte a source maps

#### **v8_parser**
- Parser recursivo descendente
- Análise semântica integrada
- Suporte a módulos ES6+
- Tratamento robusto de erros

#### **v8_runtime**
- Contextos de execução isolados
- Objetos builtin do ECMAScript
- Sistema de escopos léxicos
- Gerenciamento de memória

#### **v8_vm**
- Máquina virtual com bytecode otimizado
- Sistema de frames e stack
- Compilador JIT planejado
- Otimizações avançadas

#### **v8_gc**
- Garbage collector próprio
- Coleta por marcação e varredura
- Otimizado para performance
- Integração com runtime

#### **v8_api**
- API pública unificada
- CLI e REPL interativo
- Interface para embedding
- Integração de todos os componentes

## Vantagens da Arquitetura Modular

### **Reutilização**
- Outros projetos podem usar apenas o lexer ou AST
- Cada crate pode ser usado independentemente
- Facilita integração com outros projetos Rust

### **Testabilidade**
- Cada componente pode ser testado isoladamente
- Testes unitários mais focados
- Benchmarks específicos por componente

### **Performance**
- Compilação paralela dos crates
- Otimizações específicas por componente
- Menor overhead de dependências

### **Manutenibilidade**
- Mudanças em um componente não afetam outros
- Equipes podem trabalhar em componentes diferentes
- Código mais organizado e legível

### **Compatibilidade**
- Estrutura similar ao Boa Engine
- Facilita contribuições da comunidade
- Padrões estabelecidos e reconhecidos

## Melhorias Implementadas

### **Lexer Melhorado**
- ✅ Posicionamento preciso (linha/coluna)
- ✅ Sistema de erros robusto
- ✅ Tokens com spans
- ✅ Suporte a Unicode
- ✅ String interning planejado

### **AST Completo**
- ✅ Todos os nós ECMAScript
- ✅ Serialização com Serde
- ✅ Visitor pattern
- ✅ Source location tracking

### **Parser Robusto**
- ✅ Análise semântica
- ✅ Tratamento de erros
- ✅ Suporte a módulos
- ✅ Otimizações de parsing

## Próximos Passos

1. **Implementar v8_parser** - Parser que usa o novo lexer e AST
2. **Desenvolver v8_runtime** - Runtime básico com objetos builtin
3. **Criar v8_vm** - Máquina virtual com bytecode
4. **Implementar v8_gc** - Garbage collector funcional
5. **Finalizar v8_engine** - Engine principal unificada

## Checklist de Implementação
Consulte o arquivo [V8-Rust-Checklist.md](./V8-Rust-Checklist.md) para acompanhar o progresso e as tarefas necessárias.

## Referências
- [V8 Engine (GitHub)](https://github.com/v8/v8)
- [Boa Engine (GitHub)](https://github.com/boa-dev/boa)
- [V8 Docs](https://v8.dev/docs)

---

> Projeto open source, inspirado e motivado pelo poder do V8, pela robustez do Rust e pelas melhores práticas do Boa Engine. 