# ConectaStore

Sistema de recomendação de produtos baseado em grafos para a disciplina **Data Structure Strategy and Implementation** (MegaStore / UNIFECAF).

## Objetivo

Demonstrar o uso de **grafos**, **HashMap** e **BFS** para gerar recomendações a partir de relações entre clientes, produtos e categorias.

## Problema

Recomendações baseadas só em “mais vendidos” ou mesma categoria ignoram compras, interesses, similaridade, avaliações e categorias. O ConectaStore modela essas relações explicitamente.

## Solução

1. Entidades em **HashMap** (`Product`, `Customer`, `Category`).
2. Relações como **vértices** e **arestas ponderadas** em **lista de adjacência**.
3. **BFS** com limite de profundidade.
4. **Pontuação** determinística e ordenação.
5. **HashSet** para evitar duplicatas e revisitas no BFS.

## Arquitetura

Fluxo de dependência:

```
models → graph → repository → recommendation
```

Módulos auxiliares: `utils` (dados demo), `benchmark` (desempenho), `main` (CLI).

| Módulo | Papel |
|--------|--------|
| `models/` | Tipos de domínio e IDs tipados |
| `graph/` | Lista de adjacência, BFS, DFS, `EdgeType` |
| `repository/` | `Store`: HashMaps + sincronização com vértices/arestas |
| `recommendation/` | `RecommendationEngine` + `score_recommendation` |
| `benchmark/` | Grafo sintético e medição com `Instant` |
| `utils/` | `load_demo_store()` |

## Estrutura do projeto

```
src/
├── main.rs                 # CLI (menu 1–12)
├── bin/benchmark.rs        # benchmark standalone
├── lib.rs
├── models/
├── graph/
├── repository/
├── recommendation/
├── benchmark/
└── utils/
tests/
└── integration_recommendations.rs
docs/
└── ANALISE_IMPLEMENTACAO.md
```

## Estruturas de dados

### Entidades (repositório)

Separadas do grafo, acesso por ID:

- `HashMap<ProductId, Product>`
- `HashMap<CustomerId, Customer>`
- `HashMap<CategoryId, Category>`
- `HashMap<ProductId, VertexId>` (e equivalentes para cliente/categoria)

### Grafo (lista de adjacência)

- `HashMap<VertexId, Vec<Edge>>` — **não** usa matriz de adjacência.
- Cada `Edge` contém vértice destino, `EdgeType` e `weight`.
- Grafo modelado como não direcionado nas ligações de negócio (`add_undirected_edge`).

### Recomendação e BFS

- BFS: `VecDeque` + `HashSet<VertexId>` (visitados).
- Resultado final: `HashSet<ProductId>` para uma ocorrência por produto.

## Algoritmos

### BFS (`graph::bfs`)

- Fila `VecDeque`; marca visitados antes de enfileirar vizinhos.
- Para em `max_depth`; não revisita vértices → sem loop infinito em ciclos.
- `RecommendationEngine` chama `bfs` em `collect_product_recommendations`.

### DFS (`graph::dfs`)

- Percurso iterativo complementar; **não** usado na recomendação principal.

### Pontuação (`score_recommendation`)

```text
score = (weight × EdgeType::score_multiplier()) / max(depth, 1)
```

Multiplicadores em `EdgeType::score_multiplier()` (ex.: Purchased 1.0, Similar 0.9). Resultados ordenados por `score` decrescente.

## Tecnologias

- Rust 2021, Cargo, biblioteca padrão apenas (sem dependências externas).

## Como compilar

```bash
cargo build
```

Release (recomendado para benchmark):

```bash
cargo build --release
```

## Como executar

CLI interativa (carrega dados de demonstração ao iniciar):

```bash
cargo run
```

Menu:

| Opção | Ação |
|-------|------|
| 1 | Listar produtos |
| 2 | Consultar produto |
| 3 | Cadastrar produto |
| 4 | Listar clientes |
| 5 | Consultar cliente |
| 6 | Cadastrar cliente (ID automático ou informado) |
| 7 | Recomendar para cliente |
| 8 | Recomendar para produto |
| 9 | Informações do grafo |
| 10 | Demonstração (cliente 1 / Ana) |
| 11 | Benchmark |
| 12 | Sair |

## Como testar

```bash
cargo test
```

- **17** testes unitários na biblioteca.
- **3** testes de integração em `tests/integration_recommendations.rs`.

Formatação:

```bash
cargo fmt -- --check
```

## Como executar o benchmark

```bash
cargo run --release --bin benchmark
```

Ou na CLI: opção **11**.

Parâmetros (constantes em `benchmark/runner.rs`):

| Parâmetro | Valor |
|-----------|--------|
| Volumes | 100, 1.000, 10.000 produtos |
| Grafo | Cadeia de `Similar` + compra no produto 0 |
| Operação | `RecommendationEngine::for_customer` (BFS) |
| Profundidade | 6 |
| Limite de recomendações | 20 |
| Repetições por volume | 10 |
| Unidade | microssegundos (média, mín, máx) |

### Desempenho (execução real — release)

Medição obtida neste ambiente com `cargo run --release --bin benchmark`:

| Produtos | Média (µs) | Mín (µs) | Máx (µs) |
|----------|------------|----------|----------|
| 100 | 34 | 19 | 69 |
| 1.000 | 247 | 227 | 285 |
| 10.000 | 3871 | 2994 | 4926 |

Reexecute o comando acima na sua máquina se precisar atualizar os números.

## Exemplos de uso

1. `cargo run` → **10** — demo Ana (Notebook A → similares).
2. **7** → cliente **1** — recomendações para Ana.
3. **6** → Enter no ID → nome **João Teste** — novo cliente com vértice no grafo.
4. **3** — cadastrar produto (informe ID de categoria existente, ex.: **2** = Informática).

## Complexidade

Análise alinhada ao código atual:

### HashMap

- `get` / `insert` em entidades e adjacência: **O(1) amortizado** (média); pior caso teórico O(n).

### Lista de adjacência

- Armazenamento: **O(V + E)**.
- Iterar vizinhos de um vértice: **O(grau(v))**.

### BFS

- No subgrafo alcançado até `max_depth`: **O(V' + E')** com V'/E' visitados; cada vértice entra no `HashSet` uma vez.

### Recomendação

1. BFS — O(V' + E').
2. Filtragem + `HashSet` de produtos — O(k) sobre passos do BFS.
3. Ordenação de candidatos — O(R log R), R = tamanho da lista antes do `truncate(limit)`.

### Cadastro / consulta

- Produto, cliente, categoria por ID: **O(1) amortizado** via HashMap.
- Inserção de aresta: **O(1) amortizado** (push na `Vec` de adjacência).

## Escalabilidade

- Manter grafo esparso e lista de adjacência.
- Particionar buscas por categoria/comunidade em catálogos enormes.
- Pré-computar vizinhos para itens frequentes.
- Persistência externa fora do escopo deste trabalho.

## Vídeo Pitch

[Link do vídeo pitch]
