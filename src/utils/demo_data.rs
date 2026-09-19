use crate::repository::Store;

/// Conjunto fictício coerente para demonstração em vídeo e CLI.
pub fn load_demo_store() -> Store {
    let mut store = Store::new();

    let eletronicos = store.register_category("Eletrônicos").expect("cat");
    let informatica = store.register_category("Informática").expect("cat");
    let celulares = store.register_category("Celulares").expect("cat");
    let livros = store.register_category("Livros").expect("cat");
    let casa = store.register_category("Casa").expect("cat");
    let moveis = store.register_category("Móveis").expect("cat");
    let vestuario = store.register_category("Vestuário").expect("cat");

    let notebook_a = store
        .register_product(
            "Notebook A",
            informatica.id,
            3499.90,
            "Intel i5, 16GB RAM, SSD 512GB",
        )
        .expect("p");
    let notebook_b = store
        .register_product(
            "Notebook B",
            informatica.id,
            4299.00,
            "Intel i7, 16GB RAM, SSD 1TB",
        )
        .expect("p");
    let mouse_x = store
        .register_product("Mouse X", informatica.id, 189.90, "Ergonômico sem fio")
        .expect("p");
    let teclado = store
        .register_product(
            "Teclado Mecânico Pro",
            informatica.id,
            459.00,
            "Switch brown, RGB",
        )
        .expect("p");
    let monitor = store
        .register_product("Monitor 27 4K", eletronicos.id, 2199.00, "IPS, 144Hz")
        .expect("p");
    let smartphone = store
        .register_product(
            "Smartphone Z",
            celulares.id,
            2999.00,
            "128GB, câmera tripla",
        )
        .expect("p");
    let capa = store
        .register_product("Capa Smartphone Z", celulares.id, 79.90, "Antichoque")
        .expect("p");
    let livro_rust = store
        .register_product("Rust em Ação", livros.id, 189.00, "Programação de sistemas")
        .expect("p");
    let livro_algos = store
        .register_product(
            "Algoritmos Ilustrados",
            livros.id,
            129.00,
            "Estruturas de dados",
        )
        .expect("p");
    let lampada = store
        .register_product("Lâmpada LED Smart", casa.id, 99.90, "Wi-Fi, dimmer")
        .expect("p");
    let mesa = store
        .register_product("Mesa Escritório", moveis.id, 899.00, "120cm, ajustável")
        .expect("p");
    let camiseta = store
        .register_product("Camiseta Dev", vestuario.id, 79.00, "Algodão, estampa Rust")
        .expect("p");

    store
        .link_similar_products(notebook_a.id, notebook_b.id, 0.92)
        .expect("sim");
    store
        .link_similar_products(notebook_b.id, mouse_x.id, 0.75)
        .expect("sim");
    store
        .link_similar_products(notebook_a.id, teclado.id, 0.8)
        .expect("sim");
    store
        .link_similar_products(notebook_a.id, monitor.id, 0.7)
        .expect("sim");
    store
        .link_similar_products(smartphone.id, capa.id, 0.95)
        .expect("sim");
    store
        .link_similar_products(livro_rust.id, livro_algos.id, 0.88)
        .expect("sim");
    store
        .link_similar_products(mesa.id, monitor.id, 0.6)
        .expect("sim");

    let ana = store.register_customer("Ana Silva").expect("c");
    let bruno = store.register_customer("Bruno Costa").expect("c");
    let carla = store.register_customer("Carla Mendes").expect("c");

    store
        .link_purchase(ana.id, notebook_a.id, 1.0)
        .expect("buy");
    store.link_interest(ana.id, monitor.id, 0.7).expect("int");
    store
        .link_rating(ana.id, notebook_a.id, 0.95)
        .expect("rate");

    store
        .link_purchase(bruno.id, smartphone.id, 1.0)
        .expect("buy");
    store.link_interest(bruno.id, capa.id, 0.8).expect("int");

    store
        .link_purchase(carla.id, livro_rust.id, 1.0)
        .expect("buy");
    store
        .link_interest(carla.id, camiseta.id, 0.5)
        .expect("int");

    let _ = lampada;

    store
}
