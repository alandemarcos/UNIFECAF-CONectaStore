use std::io::{self, Write};

use conectastore::benchmark::{print_benchmark_table, run_benchmark, BENCHMARK_VOLUMES};
use conectastore::models::{CategoryId, CustomerId, ProductId};
use conectastore::recommendation::{RecommendationEngine, RecommendationError};
use conectastore::repository::{Store, StoreError};
use conectastore::utils::load_demo_store;

fn main() {
    let mut store = load_demo_store();
    println!("CONectaStore — Recomendação baseada em grafos\n");

    loop {
        print_menu();
        let choice = read_line("Escolha uma opção: ");
        match choice.trim() {
            "1" => list_products(&store),
            "2" => consult_product(&store),
            "3" => register_product(&mut store),
            "4" => list_customers(&store),
            "5" => consult_customer(&store),
            "6" => register_customer(&mut store),
            "7" => recommend_for_customer(&store),
            "8" => recommend_for_product(&store),
            "9" => show_graph_info(&store),
            "10" => run_demo(&store),
            "11" => {
                let rows = run_benchmark(&BENCHMARK_VOLUMES);
                print_benchmark_table(&rows);
            }
            "12" => {
                println!("Encerrando. Até logo!");
                break;
            }
            _ => println!("Opção inválida. Informe um número de 1 a 12.\n"),
        }
    }
}

fn print_menu() {
    println!("1.  Listar produtos");
    println!("2.  Consultar produto");
    println!("3.  Cadastrar produto");
    println!("4.  Listar clientes");
    println!("5.  Consultar cliente");
    println!("6.  Cadastrar cliente");
    println!("7.  Recomendar produtos para cliente");
    println!("8.  Recomendar produtos relacionados a produto");
    println!("9.  Exibir informações do grafo");
    println!("10. Executar demonstração");
    println!("11. Executar benchmark");
    println!("12. Sair");
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() {
        return String::new();
    }
    buf
}

fn parse_u64(s: &str) -> Option<u64> {
    s.trim().parse().ok()
}

fn list_products(store: &Store) {
    println!("\n--- Produtos ---");
    for p in store.list_products() {
        println!(
            "{} | {} | R$ {:.2} | cat {}",
            p.id, p.name, p.price, p.category_id
        );
    }
    println!();
}

fn consult_product(store: &Store) {
    let id = read_line("ID do produto (número): ");
    let Some(num) = parse_u64(&id) else {
        println!("ID inválido.\n");
        return;
    };
    match store.get_product(ProductId(num)) {
        Some(p) => println!(
            "\n{} — {}\nCategoria: {}\nPreço: R$ {:.2}\n{}\n",
            p.id, p.name, p.category_id, p.price, p.description
        ),
        None => println!("Produto não encontrado.\n"),
    }
}

fn register_product(store: &mut Store) {
    let name = read_line("Nome: ");
    if name.trim().is_empty() {
        println!("Nome não pode ser vazio.\n");
        return;
    }
    let cat = read_line("ID da categoria: ");
    let price_s = read_line("Preço: ");
    let desc = read_line("Descrição: ");
    let Some(cat_id) = parse_u64(&cat) else {
        println!("Categoria inválida.\n");
        return;
    };
    let Ok(price) = price_s.trim().parse::<f64>() else {
        println!("Preço inválido.\n");
        return;
    };
    match store.register_product(name.trim(), CategoryId(cat_id), price, desc.trim()) {
        Ok(p) => println!("Produto cadastrado: {} — {}\n", p.id, p.name),
        Err(StoreError::CategoryNotFound) => println!("Categoria não encontrada.\n"),
        Err(StoreError::InvalidInput) => println!("Dados inválidos (nome ou preço).\n"),
        Err(e) => println!("Erro: {e:?}\n"),
    }
}

fn list_customers(store: &Store) {
    println!("\n--- Clientes ---");
    for c in store.list_customers() {
        println!("{} | {}", c.id, c.name);
    }
    println!();
}

fn consult_customer(store: &Store) {
    let id = read_line("ID do cliente (número): ");
    let Some(num) = parse_u64(&id) else {
        println!("ID inválido.\n");
        return;
    };
    match store.get_customer(CustomerId(num)) {
        Some(c) => {
            println!("\n{} — {}", c.id, c.name);
            if c.purchased_product_ids.is_empty() {
                println!("Compras: nenhuma registrada");
            } else {
                println!("Compras: {:?}", c.purchased_product_ids);
            }
            println!();
        }
        None => println!("Cliente não encontrado.\n"),
    }
}

fn register_customer(store: &mut Store) {
    let id_input = read_line("ID do cliente (Enter = gerar automaticamente): ");
    let name = read_line("Nome: ");
    if name.trim().is_empty() {
        println!("Nome não pode ser vazio.\n");
        return;
    }

    let result = if id_input.trim().is_empty() {
        store.register_customer(name.trim())
    } else {
        let Some(num) = parse_u64(&id_input) else {
            println!("ID inválido.\n");
            return;
        };
        store.register_customer_with_id(CustomerId(num), name.trim())
    };

    match result {
        Ok(c) => {
            println!(
                "Cliente cadastrado: {} — {} (vértice criado no grafo)\n",
                c.id, c.name
            );
        }
        Err(StoreError::CustomerAlreadyExists) => {
            println!("Já existe cliente com esse ID.\n");
        }
        Err(StoreError::InvalidInput) => println!("Nome inválido.\n"),
        Err(e) => println!("Erro: {e:?}\n"),
    }
}

fn recommend_for_customer(store: &Store) {
    let id = read_line("ID do cliente: ");
    let Some(num) = parse_u64(&id) else {
        println!("ID inválido.\n");
        return;
    };
    match RecommendationEngine::for_customer(store, CustomerId(num), 4, 10) {
        Ok(recs) => print_recommendations(store, &recs),
        Err(RecommendationError::CustomerNotFound) => println!("Cliente não encontrado.\n"),
        Err(RecommendationError::NoRecommendations) => {
            println!("Nenhuma recomendação encontrada.\n")
        }
        Err(RecommendationError::ProductNotFound) => println!("Erro interno.\n"),
    }
}

fn recommend_for_product(store: &Store) {
    let id = read_line("ID do produto: ");
    let Some(num) = parse_u64(&id) else {
        println!("ID inválido.\n");
        return;
    };
    match RecommendationEngine::for_product(store, ProductId(num), 4, 10) {
        Ok(recs) => print_recommendations(store, &recs),
        Err(RecommendationError::ProductNotFound) => println!("Produto não encontrado.\n"),
        Err(RecommendationError::NoRecommendations) => {
            println!("Nenhuma recomendação encontrada.\n")
        }
        Err(RecommendationError::CustomerNotFound) => println!("Erro interno.\n"),
    }
}

fn print_recommendations(
    store: &Store,
    recs: &[conectastore::recommendation::ScoredRecommendation],
) {
    println!("\n--- Recomendações ---");
    for (i, r) in recs.iter().enumerate() {
        let name = store
            .get_product(r.product_id)
            .map(|p| p.name.as_str())
            .unwrap_or("?");
        println!(
            "{}. {} ({}) — score {:.3} | profundidade {}",
            i + 1,
            r.product_id,
            name,
            r.score,
            r.depth
        );
    }
    println!();
}

fn show_graph_info(store: &Store) {
    let g = store.graph();
    println!("\n--- Grafo ---");
    println!("Vértices: {}", g.vertex_count());
    println!("Arestas (não direcionadas): {}", g.edge_count());
    println!("Estrutura: lista de adjacência (HashMap + Vec<Edge>)\n");
}

fn run_demo(store: &Store) {
    println!("\n=== Demonstração ===");
    println!(
        "Cliente Ana (C1) comprou Notebook A; esperamos recomendações como Notebook B, Mouse X...\n"
    );
    match RecommendationEngine::for_customer(store, CustomerId(1), 4, 8) {
        Ok(recs) => print_recommendations(store, &recs),
        Err(e) => println!("Demo falhou: {e:?}\n"),
    }
}
