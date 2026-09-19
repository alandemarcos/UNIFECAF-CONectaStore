# ConectaStore

Sistema de recomendação de produtos baseado em grafos para a disciplina **Data Structure Strategy and Implementation** (MegaStore / UNIFECAF).

## Objetivo

Demonstrar, de forma prática e tecnicamente justificável, o uso de **grafos**, **HashMap** e **algoritmos de percurso (BFS)** para gerar recomendações de produtos a partir de relações entre clientes, produtos e categorias.

## Problema

A MegaStore possui um catálogo amplo, mas recomendações baseadas apenas em “mais vendidos” ou mesma categoria ignoram relações ricas: compras, interesses, similaridade, avaliações e pertencimento a categorias. O ConectaStore modela essas relações explicitamente.

## Solução

1. Entidades (`Product`, `Customer`, `Category`) são armazenadas em **HashMap** para consulta O(1) média por ID.
2. Relações são **vértices** (cliente, produto, categoria) e **arestas ponderadas** (compra, similar, interesse, etc.) em um **grafo com lista de adjacência**.
3. **BFS** explora o grafo a partir de um cliente ou produto, com limite de profundidade.
4. Produtos alcançados recebem **pontuação** derivada do peso da aresta, tipo de relação e distância no grafo.
5. **HashSet** evita duplicatas e revisitas desnecessárias durante o percurso.

## Modelagem do Grafo

| Elemento | Representação |
|----------|----------------|
| Vértices | `VertexKind`: Customer, Product, Category |
| Arestas | `Edge`: destino, `EdgeType`, `weight` |
| Pesos | Compra/similaridade/interesse refletem força da relação |
| Lista de adjacência | `HashMap<VertexId, Vec<Edge>>` — adequada a grafos esparsos (catálogo grande, poucas relações por item) |
| HashMap de entidades | `HashMap<ProductId, Product>`, etc. — separado da topologia do grafo |

**Por que lista de adjacência (e não matriz)?** O número de produtos é muito maior que o grau médio de cada vértice; matriz seria O(V²) em memória. Lista de adjacência escala com V + E.

**Por que HashMap?** Cadastro e consulta por ID devem evitar busca linear em catálogos grandes.

## Algoritmo de Recomendação

1. Obtém o `VertexId` do cliente ou produto inicial (HashMap interno no `Store`).
2. Executa **BFS** com `VecDeque` (fila) e `HashSet` (visitados), até `max_depth`.
3. Para cada vértice do tipo **Product** alcançado:
   - Ignora produtos já comprados (recomendação por cliente) ou o produto origem.
   - Usa `HashSet<ProductId>` para **não repetir** o mesmo produto na lista final.
4. Calcula `score = (weight × multiplicador_do_tipo) / profundidade` e ordena decrescente.

**Por que BFS?** Recomendações por proximidade no grafo correspondem naturalmente a “expansão em camadas”: primeiro vizinhos diretos (compras/similares), depois vizinhos de vizinhos. BFS garante a profundidade mínima até cada nó, o que alinha score e interpretabilidade no pitch.

DFS também está implementado (`graph::dfs`) como recurso complementar de percurso, mas a recomendação principal usa BFS.

## Estrutura do Projeto

```
src/
├── main.rs              # CLI interativa
├── lib.rs               # exports da biblioteca
├── models/              # Product, Customer, Category, IDs
├── graph/               # lista de adjacência, BFS, DFS, EdgeType
├── repository/          # Store + HashMaps + sincronização com o grafo
├── recommendation/      # motor de recomendação e pontuação
├── benchmark/           # dados sintéticos e medição com Instant
└── utils/               # dados de demonstração (load_demo_store)
tests/
└── integration_recommendations.rs
```

## Tecnologias

- Rust (edition 2021)
- Cargo
- Biblioteca padrão (`HashMap`, `HashSet`, `VecDeque`, `Instant`)

Sem dependências externas.

## Como Compilar

```bash
cargo build
```

## Como Executar

```bash
cargo run
```

Menu: listar/cadastrar/consultar, recomendações, info do grafo, demo e benchmark.

## Como Executar os Testes

```bash
cargo test
```

Inclui testes unitários nos módulos (`graph`, `repository`, `recommendation`, `benchmark`) e testes de integração em `tests/`.

## Teste de Desempenho

## Comando para executar o benchmark

```bash
cargo run --release --bin benchmark
```

Ou na CLI: `cargo run` → opção `10`.

```rust
use conectastore::benchmark::{run_benchmark, print_benchmark_table};
let rows = run_benchmark(&[100, 1_000, 10_000]);
print_benchmark_table(&rows);
```

Volumes padrão: 100, 1.000 e 10.000 produtos (grafo sintético em cadeia de similaridade + compra inicial).

### Resultados

Execute o benchmark localmente e preencha com os tempos reais:

| Volume | Tempo (ms) |
|--------|------------|
| 100    | 0 (debug/release, sub-ms) |
| 1.000  | 0 |
| 10.000 | 3 |

Medição obtida com:

```bash
cargo run --release --bin benchmark
```

Na CLI interativa (`cargo run`), use a opção **10**.

## Exemplos de Uso

Após `cargo run`:

1. **Listar produtos** — opção `1`
2. **Recomendar para cliente** — opção `6`, ID `1` (Ana Silva nos dados demo)
3. **Demonstração** — opção `9` (fluxo Notebook A → similares)
4. **Benchmark** — opção `10`

## Complexidade

| Operação | Complexidade |
|----------|----------------|
| Cadastro produto/cliente (HashMap insert) | O(1) médio |
| Consulta por ID (HashMap get) | O(1) médio |
| Inserção de vértice | O(1) médio |
| Inserção de aresta (lista de adjacência) | O(1) amortizado |
| BFS | O(V + E) no subgrafo visitado até a profundidade limite |
| Recomendação | O(V + E) + O(R log R) para ordenar R candidatos |
| Espaço do grafo | O(V + E) |

Valores assumem hash eficiente; pior caso teórico de HashMap é O(n), raro com boa função de hash.

## Escalabilidade

Para milhões de produtos:

- Manter **lista de adjacência** e grafos esparsos (só relações reais).
- Particionar recomendação (por categoria ou por comunidade).
- Pré-computar vizinhanças para produtos “hot”.
- Persistência externa (não escopo deste trabalho) com IDs estáveis.

O desenho atual separa **armazenamento** (HashMap), **topologia** (grafo) e **algoritmo** (BFS + score), facilitando evolução incremental.

## Arquitetura

- **models**: tipos de domínio puros.
- **graph**: estruturas e algoritmos de grafos, sem regra de negócio de e-commerce.
- **repository**: orquestra cadastros e liga entidades aos vértices.
- **recommendation**: regras de exclusão, score e ordenação.
- **main**: apenas CLI.

## Vídeo Pitch

[Link do vídeo pitch]
