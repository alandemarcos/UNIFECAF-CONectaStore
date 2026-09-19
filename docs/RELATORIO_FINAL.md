# Relatório Final — CONectaStore

**Disciplina:** Data Structure Strategy and Implementation  
**Projeto:** CONectaStore — Sistema de Recomendação de Produtos Baseado em Grafos  
**Linguagem:** Rust (Cargo, biblioteca padrão)  
**Data do relatório:** 19/09/2026  

---

## 1. Resumo executivo

O CONectaStore implementa recomendação de produtos modelando clientes, produtos e categorias como **vértices** e relações de negócio (compra, similaridade, interesse, avaliação, categoria) como **arestas ponderadas** em um **grafo com lista de adjacência**. O motor de recomendação utiliza **BFS** com limite de profundidade, **pontuação determinística** e **HashSet** para evitar duplicatas.

Após a implementação inicial, foi executada a etapa de **correção e finalização** (auditoria, cadastro de cliente na CLI, benchmark em microssegundos, README alinhado ao código e validação com `cargo test` / benchmark real).

---

## 2. O que foi corrigido na etapa de finalização

| Item | Descrição |
|------|-----------|
| Cadastro de cliente na CLI | Opção **6** no menu; ID automático (Enter) ou manual; rejeita ID duplicado e nome vazio |
| Menu CLI | Renumerado para **12 opções** (benchmark **11**, sair **12**) |
| Benchmark | **10 repetições** por volume; tempos em **µs** (média, mínimo, máximo); documentação da operação medida |
| Validação de entradas | Nomes vazios, preços inválidos, IDs inexistentes e opções inválidas não encerram o programa |
| Repositório | `register_customer_with_id`, erros `InvalidInput` e `CustomerAlreadyExists` |
| Testes | Ciclos no BFS/DFS, profundidade máxima na recomendação, cadastro de cliente (duplicata/nome vazio) |
| README | Revisado para refletir menu, arquitetura, complexidade e resultados **reais** do benchmark |

---

## 3. Arquivos alterados (finalização)

- `src/main.rs` — menu 1–12, cadastro de cliente, validações
- `src/repository/store.rs` — cadastro com ID explícito e validações
- `src/benchmark/runner.rs` — medição em µs, repetições, saída formatada
- `src/benchmark/mod.rs` — exports de constantes
- `src/bin/benchmark.rs` — volumes padrão centralizados
- `src/graph/algorithms.rs` — testes de ciclo; comentário BFS
- `src/recommendation/engine.rs` — teste de profundidade máxima
- `src/recommendation/scoring.rs` — documentação da fórmula
- `README.md` — documentação completa atualizada

### Arquivos de referência (implementação + análise)

- `docs/ANALISE_IMPLEMENTACAO.md` — análise do prompt e manutenibilidade
- `prompt/prompt inicial` — requisitos originais
- `prompt/CORREÇÃO E FINALIZAÇÃO DA PARTE PRÁTICA — CONectaStore` — requisitos da auditoria final

---

## 4. Arquitetura final

```
models → graph → repository → recommendation
         ↑
    utils (demo)    benchmark (desempenho)    main / CLI
```

| Camada | Responsabilidade |
|--------|------------------|
| **models** | `Product`, `Customer`, `Category`, IDs tipados |
| **graph** | Lista de adjacência, `EdgeType`, BFS, DFS |
| **repository** | `Store`: HashMaps de entidades + sincronização com vértices/arestas |
| **recommendation** | BFS + filtros + `score_recommendation` + ordenação |
| **utils** | `load_demo_store()` para demonstração |
| **benchmark** | Grafo sintético em cadeia + medição com `Instant` |
| **main** | Interface de linha de comando |

---

## 5. Estruturas de dados utilizadas

### Entidades (HashMap)

- `HashMap<ProductId, Product>`
- `HashMap<CustomerId, Customer>`
- `HashMap<CategoryId, Category>`
- Mapas auxiliares `ProductId` / `CustomerId` / `CategoryId` → `VertexId`

### Grafo (lista de adjacência)

- `HashMap<VertexId, Vec<Edge>>`
- Cada aresta: destino, tipo (`EdgeType`), peso (`f64`)

### Algoritmos de percurso e recomendação

- **BFS:** `VecDeque` + `HashSet<VertexId>` (visitados)
- **Recomendação:** `HashSet<ProductId>` (sem duplicatas na lista final)

---

## 6. Algoritmos utilizados

| Algoritmo | Uso |
|-----------|-----|
| **BFS** | Recomendação por cliente e por produto (`RecommendationEngine` → `graph::bfs`) |
| **DFS** | Complementar; demonstração em `graph::dfs` |
| **Pontuação** | `score = (weight × multiplicador_do_tipo) / max(depth, 1)` |
| **Ordenação** | Decrescente por `score` antes de aplicar `limit` |

**Justificativa do BFS:** expansão em camadas a partir do cliente/produto origem, alinhada à noção de proximidade no grafo e ao limite de profundidade exigido pelo trabalho.

---

## 7. Testes executados

Comando:

```bash
cargo test
```

**Resultado:** 20 testes aprovados.

| Tipo | Quantidade | Escopo |
|------|------------|--------|
| Unitários (lib) | 17 | Grafo, BFS/DFS, store, scoring, engine, benchmark |
| Integração | 3 | Fluxo completo, anti-duplicidade, exclusão do produto origem |

Cobertura relevante: vértices/arestas, cadastro/consulta, recomendação, duplicidades, cliente isolado, ciclos, profundidade máxima, scoring, benchmark em volumes reduzidos.

---

## 8. Validação Cargo (execução real)

| Comando | Resultado |
|---------|-----------|
| `cargo fmt -- --check` | Sucesso |
| `cargo check` | Sucesso |
| `cargo build` | Sucesso |
| `cargo build --release` | Sucesso |
| `cargo test` | 20 passed |
| `cargo run --release --bin benchmark` | Sucesso (saída abaixo) |

A CLI interativa (`cargo run`) deve ser exercitada manualmente no terminal para gravação do vídeo pitch (opções 6, 7, 10, 11).

---

## 9. Benchmark

### Como executar

```bash
cargo run --release --bin benchmark
```

Alternativa: `cargo run` → opção **11**.

### Configuração (código)

| Parâmetro | Valor |
|-----------|--------|
| Volumes | 100, 1.000, 10.000 produtos |
| Estrutura do grafo | Cadeia de similaridade + compra do cliente no primeiro produto |
| Operação medida | `RecommendationEngine::for_customer` (BFS + score + ordenação) |
| Profundidade máxima | 6 |
| Limite de recomendações | 20 |
| Repetições por volume | 10 |
| Unidade | Microssegundos (µs) |

### Resultados (release — execução real neste ambiente)

| Produtos | Média (µs) | Mín (µs) | Máx (µs) |
|----------|------------|----------|----------|
| 100 | 34 | 19 | 69 |
| 1.000 | 247 | 227 | 285 |
| 10.000 | 3871 | 2994 | 4926 |

> Reexecute o benchmark na máquina de entrega/apresentação para registrar números do ambiente final, se necessário.

---

## 10. Comandos finais para o aluno

```bash
# Compilar
cargo build --release

# Aplicação (menu interativo + dados demo)
cargo run

# Testes
cargo test

# Benchmark
cargo run --release --bin benchmark

# Formatação
cargo fmt -- --check
```

### Atalhos úteis na CLI (após `cargo run`)

| Opção | Ação |
|-------|------|
| 6 | Cadastrar cliente |
| 7 | Recomendar para cliente (ex.: ID **1** = Ana) |
| 10 | Demonstração guiada |
| 11 | Benchmark no terminal |

---

## 11. Auditoria de requisitos (professor)

| Requisito | Status | Evidência |
|-----------|--------|-----------|
| Cadastro de produtos | Atendido | CLI 3, `Store::register_product` |
| Consulta de produtos | Atendido | CLI 2, testes |
| Cadastro de clientes | Atendido | CLI 6, `register_customer` / `register_customer_with_id` |
| Consulta de clientes | Atendido | CLI 5 |
| Construção do grafo | Atendido | `Graph`, vértices ao cadastrar entidades |
| Gerenciamento de conexões | Atendido | `link_purchase`, `link_similar_products`, etc. |
| Lista de adjacência | Atendido | `HashMap<VertexId, Vec<Edge>>` |
| Vértices e arestas com pesos | Atendido | `VertexKind`, `Edge`, `EdgeType` |
| HashMap | Atendido | `Store` + adjacência |
| BFS | Atendido | `graph::bfs`, usado pelo engine |
| DFS | Atendido | `graph::dfs` |
| Recomendação por cliente | Atendido | `RecommendationEngine::for_customer` |
| Recomendação por produto | Atendido | `RecommendationEngine::for_product` |
| Anti-duplicidade | Atendido | `HashSet` + testes |
| Testes unitários | Atendido | 17 na lib |
| Testes de integração | Atendido | 3 em `tests/` |
| Benchmark / volumes distintos | Atendido | 100, 1k, 10k em µs |
| README | Atendido | `README.md` |
| CLI estável | Atendido | Menu 1–12, tratamento de entradas inválidas |
| Projeto Rust organizado | Atendido | Módulos conforme arquitetura |
| Sem dependências externas | Atendido | Apenas std no `Cargo.toml` |

---

## 12. Checklist de conclusão

- [x] Cadastro de cliente disponível na CLI  
- [x] Benchmark com medição em microssegundos e repetições  
- [x] README atualizado e coerente com o código  
- [x] Complexidades documentadas conforme implementação  
- [x] BFS validado (fila, visitados, profundidade, uso real no engine)  
- [x] Recomendação validada (cliente/produto, score, ordenação)  
- [x] Anti-duplicidade validada  
- [x] `cargo fmt`, `check`, `build`, `test` executados  
- [x] Benchmark executado com resultados reais registrados  
- [x] Nenhum resultado de desempenho inventado  
- [ ] Link do vídeo pitch (placeholder no README — preenchimento manual)  
- [ ] Commit/push (não automatizado por solicitação do prompt de correção)  

---

## 13. Pendências e observações

1. **Vídeo pitch:** substituir `[Link do vídeo pitch]` no `README.md` quando o vídeo estiver publicado.  
2. **Benchmark:** opcional reexecutar em release na máquina de apresentação e copiar tabela para o README.  
3. **Ambiente Windows:** compilação requer toolchain Rust + linker MSVC (Visual Studio Build Tools com C++).  
4. **Git:** versionamento e entrega ao repositório institucional ficam a cargo do aluno.  

---

## 14. Documentação complementar

- **README.md** — manual de uso, complexidade, desempenho e arquitetura  
- **docs/ANALISE_IMPLEMENTACAO.md** — análise do prompt inicial e manutenibilidade  

---

*Relatório gerado após implementação e etapa de correção/finalização do CONectaStore. Validações de compilação, testes e benchmark foram executadas de fato neste ambiente; a CLI interativa requer validação manual no terminal para demonstração em vídeo.*
