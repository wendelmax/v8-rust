# VM-Checklist.md

## Checklist Detalhado: Máquina Virtual para Execução de Bytecode

> **Este arquivo detalha o progresso, plano incremental e decisões técnicas para a implementação da Máquina Virtual (VM) do projeto v8-rust.**
> Consulte o [V8-Rust-Checklist.md](./V8-Rust-Checklist.md) para o status macro do projeto.

---

## Fase 1: Núcleo da Execução
- [x] Ciclo fetch-decode-execute **(100%)**
- [x] Instruções básicas: PushConst, Add, Sub, Mul, Div, Pop, Dup **(100%)**
- [x] Stack de execução e frames de chamada **(100%)**
- [x] Variáveis locais e passagem de argumentos simples **(100%)**
- [x] Testes unitários para instruções básicas **(100%)**

## Fase 2: Controle de Fluxo e Variáveis
- [ ] Jump, JumpIfTrue, JumpIfFalse, Return **(80%)**
- [ ] If, while, for, break, continue **(0%)**
- [ ] LoadLocal, StoreLocal, LoadGlobal, StoreGlobal **(50%)**
- [ ] Testes para escopos e controle de fluxo **(50%)**

## Fase 3: Heap e Tipos Dinâmicos
- [ ] Estrutura de heap para objetos, arrays, funções, strings **(0%)**
- [ ] Tipo genérico Value (Number, String, Boolean, Object, Array, Function, etc.) **(0%)**
- [ ] Integração stack/heap via handles **(0%)**
- [ ] Testes de heap e Value **(0%)**

## Fase 4: Funções, Closures e Contextos
- [ ] Call, criação de frames, passagem de argumentos, retorno **(0%)**
- [ ] Suporte a closures e escopos léxicos **(0%)**
- [ ] Testes de funções, recursão, closures, this **(0%)**

## Fase 5: Objetos, Arrays e Propriedades
- [ ] NewObject, NewArray, SetProperty, GetProperty **(0%)**
- [ ] Propriedades dinâmicas, protótipos **(0%)**
- [ ] Testes de objetos/arrays **(0%)**

## Fase 6: Exceptions, Async/Await, Operadores Modernos
- [ ] Throw, Try, Catch, Finally **(0%)**
- [ ] Await, Yield, suporte básico a corrotinas **(0%)**
- [ ] Spread, Destructure, OptionalChain, NullishCoalesce **(0%)**
- [ ] Testes para exceptions e operadores modernos **(0%)**

## Fase 7: Integração, Otimizações e Documentação
- [ ] Integração com pipeline de bytecode **(0%)**
- [ ] Otimizações e profiling **(0%)**
- [ ] Documentação, exemplos e atualização do checklist principal **(0%)**

---

## Observações
- Heap e Value são pré-requisitos para objetos, arrays, funções, closures, exceptions, etc.
- Stack e frames são pré-requisitos para controle de fluxo, funções e variáveis locais.
- Testes devem ser incrementais, acompanhando cada etapa.
- Garbage Collection pode ser implementado após o heap básico.

---

> Atualize este arquivo a cada avanço ou decisão relevante na implementação da VM. 