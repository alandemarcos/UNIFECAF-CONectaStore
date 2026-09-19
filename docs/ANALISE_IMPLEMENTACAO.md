# CONectaStore — Análise da implementação

Documento gerado a partir do prompt em `prompt/prompt inicial` e do código do repositório.

---

## 1. Análise do prompt — o que foi entendido dos requisitos

O prompt define um **trabalho acadêmico** (não um produto comercial): demonstrar **estruturas de dados + grafos + algoritmos + recomendação + testes + medição de desempenho**, em **Rust puro** (Cargo, std, sem crates desnecessárias).

### Contexto e objetivo

- Cenário fictício **MegaStore**: recomendações não podem limitar-se a “mais vendidos” ou mesma categoria.
- **CONectaStore** modela relações entre clientes, produtos, categorias, compras, interesses, similaridades e avaliações como **grafo**, usando **percurso** (preferencialmente **BFS**) para sugerir produtos.

### Requisitos funcionais obrigatórios (mapeamento)

| Requisito do prompt | Interpretação |
|---------------------|---------------|
| CRUD de produtos/clientes + consultas | `Store` com `register_*`, `get_*`, `list_*` |
| Grafo explícito, vértices e arestas ponderadas | `Graph` + `VertexKind` + `Edge` / `EdgeType` |
| Lista de adjacência (não matriz) | `HashMap<VertexId, Vec<Edge>>` |
| HashMap para acesso por ID | Repositório separado do grafo |
| Recomendação por cliente e por produto | `RecommendationEngine` |
| BFS (DFS opcional) | `graph::bfs` (+ `graph::dfs`) |
| Sem duplicatas, sem auto-recomendação, limite de profundidade | `HashSet` + filtros no engine |
| Pontuação explicável (sem ML) | `score_recommendation` em `scoring.rs` |
| CLI com menu 1–11 | `src/main.rs` |
| Dados demo coerentes (ex.: Notebook A → B → Mouse X) | `utils/demo_data.rs` |
| Testes unitários + integração | `#[test]` nos módulos + `tests/` |
| Benchmark 3 volumes com `Instant`, resultados reais | `benchmark/` + binário `benchmark` |
| README completo (sem benchmark inventado, placeholder de vídeo) | `README.md` |

### Processo pedido (seções 21–22)

Implementação **incremental** (modelos → grafo → repositório → BFS → recomendação → CLI → demo → testes → benchmark → README), validação com `cargo check`, `build`, `test`, `run`.

### Regra fundamental

Foco em **justificativa técnica** (por que HashMap, adjacência, BFS, complexidade, anti-duplicidade, escalabilidade) — refletida no código e na documentação, não apenas “funcionar”.

---

## 2. O que foi implementado

### Visão geral

Crate **`conectastore`**: biblioteca (`src/lib.rs`) + binário CLI (`src/main.rs`) + binário **`benchmark`** (`src/bin/benchmark.rs`). Projeto na raiz do repositório (equivalente modular ao `megastore/` sugerido no prompt).

### Módulos

| Módulo | Responsabilidade |
|--------|------------------|
| **`models/`** | `Product`, `Customer`, `Category`; IDs tipados (`ProductId`, `CustomerId`, `CategoryId`, `VertexId`). |
| **`graph/`** | Grafo com **lista de adjacência**; `EdgeType` (Purchased, Interested, Similar, BelongsTo, Rated); **BFS** e **DFS**; arestas com peso. |
| **`repository/`** | **`Store`**: HashMaps de entidades, mapas entidade→vértice, criação automática de vértice ao cadastrar, APIs de ligação (`link_purchase`, `link_similar_products`, etc.). |
| **`recommendation/`** | **`RecommendationEngine`**: BFS a partir de cliente ou produto; exclusões; ordenação por score. |
| **`benchmark/`** | Grafo sintético em cadeia; `run_benchmark` com `std::time::Instant`; tabela no stdout. |
| **`utils/`** | `load_demo_store()`: categorias, produtos, clientes e relações para demo/vídeo. |
| **`main.rs`** | CLI interativa (opções 1–11). |

### Estruturas de dados

- **HashMap**: produtos, clientes, categorias; adjacência; mapeamento ID de negócio → `VertexId`.
- **Lista de adjacência**: vizinhos como `Vec<Edge>` por vértice (grafo esparso).
- **VecDeque + HashSet**: fila e visitados no BFS (evita ciclos e revisitas).
- **HashSet&lt;ProductId&gt;**: uma entrada por produto na lista final de recomendações.

### Algoritmos

- **BFS** com `max_depth`: base da recomendação (proximidade em camadas).
- **DFS** iterativo: complementar, não usado no fluxo principal de recomendação.
- **Scoring**: `(peso × multiplicador do tipo de aresta) / profundidade`, com multiplicadores por `EdgeType`.

### Testes

- **Unitários** (13): grafo (vértices/arestas, BFS/DFS), repositório, scoring, engine (duplicatas, exclusão do produto origem, cliente isolado), benchmark em volumes pequenos.
- **Integração** (3 em `tests/integration_recommendations.rs`): fluxo criar entidades → conexões → recomendar; sem duplicatas; produto consultado não aparece na lista.

### Benchmark

- Volumes padrão **100, 1.000, 10.000** produtos (sintéticos).
- Execução: `cargo run --release --bin benchmark` ou opção **10** na CLI.
- Tempos medidos localmente (ex.: 100/1000 → sub-ms; 10.000 → ~3 ms em release) — documentados no README, não inventados.

### CLI e demo

- Menu conforme prompt; dados pré-carregados ao iniciar.
- Opção **9**: demo Ana (cliente 1) / Notebook A e cadeia de similares.

---

## 3. Resumo solicitado (seção 21 do prompt)

### 1. Arquitetura implementada

Camadas em sequência: **domínio (`models`)** → **topologia e algoritmos (`graph`)** → **persistência em memória e sincronização com o grafo (`repository/Store`)** → **regras de recomendação (`recommendation`)** → **entrada do usuário (`main`)** / **dados fictícios (`utils`)** / **desempenho (`benchmark`)**.

Separação explícita: **objetos** (HashMap), **conexões** (grafo), **lógica de sugestão** (BFS + score).

### 2. Estruturas de dados utilizadas

- HashMap (entidades e adjacência)
- Vec (listas de adjacência)
- VecDeque, HashSet (BFS)
- HashSet (anti-duplicata em recomendações)
- Enums (`VertexKind`, `EdgeType`, erros tipados)

### 3. Algoritmos utilizados

- **BFS** (recomendação)
- **DFS** (demonstração adicional)
- Ordenação por score (sort por `f64` decrescente)

### 4. Estrutura dos arquivos criados

```
Cargo.toml, README.md, .gitignore
src/lib.rs, main.rs, bin/benchmark.rs
src/models/          (category, customer, product, ids)
src/graph/           (adjacency, algorithms, edge, vertex)
src/repository/      (store)
src/recommendation/  (engine, scoring)
src/benchmark/       (runner, synthetic)
src/utils/           (demo_data)
tests/integration_recommendations.rs
prompt/prompt inicial
```

### 5. Comandos para executar

```bash
cd C:\dev\UNIFECAF-CONectaStore
cargo build
cargo run              # CLI interativa
cargo run --release    # CLI mais rápida
```

### 6. Comandos para testar

```bash
cargo test
```

### 7. Comando para executar o benchmark

```bash
cargo run --release --bin benchmark
```

Alternativa: `cargo run` → opção **10**.

### 8. Pontos que podem exigir intervenção manual

- **Toolchain Rust + linker MSVC** no Windows (Build Tools / VC++), se `link.exe` não for encontrado.
- **Cadastro de cliente pela CLI**: exposto via `Store::register_customer`, mas o menu do prompt não inclui “cadastrar cliente” — hoje clientes vêm do demo ou da API interna/testes.
- **Link do vídeo pitch**: placeholder no README (`[Link do vídeo pitch]`).
- **Reexecutar benchmark** na sua máquina para atualizar a tabela de tempos se quiser precisão abaixo de 1 ms.
- **Commit/push** no Git: não feito automaticamente pelo fluxo do prompt.

---

## 4. Manutenibilidade

### Organização e modularidade

A divisão **models / graph / repository / recommendation** permite explicar e alterar cada camada no pitch acadêmico sem misturar “cadastro de produto” com “percurso em grafo”. O grafo não conhece preço ou nome de cliente; o engine não implementa adjacência — apenas consome `Store` + `bfs`.

### Facilidade de evolução

- **Novos tipos de relação**: estender `EdgeType` e multiplicador em `scoring`; ligar no `Store`.
- **Outra estratégia de busca**: trocar ou combinar BFS/DFS no `RecommendationEngine` sem mudar HashMaps.
- **Escala**: README já aponta particionamento e pré-computação; lista de adjacência mantém memória O(V+E).
- **Automação/CI**: binário `benchmark` evita depender de stdin da CLI.

### Riscos / melhorias naturais (sem refatoração grande)

- IDs e vértices duplicados na lógica de aresta undirected exigem cuidado ao evoluir para arestas direcionadas.
- CLI monolítica em `main.rs` ainda é aceitável para o escopo; crescer o menu pode justificar módulo `cli/`.
- Tempos 0 ms no benchmark refletem granularidade de `as_millis()` — evolução simples: microssegundos ou repetições com média.

### Avaliação geral

O projeto está **alinhado ao escopo acadêmico**: legível, testável, documentado no README, sem dependências externas. A manutenibilidade vem sobretudo da **separação armazenamento × topologia × algoritmo de recomendação**, que é exatamente o que o prompt pede para sustentar perguntas de complexidade e escalabilidade no vídeo.
