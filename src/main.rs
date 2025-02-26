#[macro_use] extern crate rocket;

mod config; // Configuração da conexão com o banco de dados
mod models; // Modelos de dados (structs)
mod routes; // Rotas da aplicação
mod guards; // Guards de autenticação e autorização
mod utils; //  Funções auxiliares (JWT, Hashing)

use rocket::form::Form;
use rocket::State;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use mysql::*;
use mysql::prelude::*;
//use serde::{Deserialize, Serialize};

use crate::models::jogador::Jogador;

// Estrutura para conexão com o MySQL
struct DbPool(Pool);

// Estrutura para capturar os dados do formulário

#[derive(FromForm)]
struct JogadorInput {
    nome: String,
    sobrenome: String,
    altura: String,
    posicao: String,
    escola_id: u32,  // Alterado para armazenar o ID da escola
    login: String,
    senha: String,
}


#[get("/")]
fn index() -> Template {
    // Aqui, você pode passar dados opcionais para o template usando o contexto
    Template::render("index", context! {
        title: "Página Inicial",
        message: "Bem-vindo ao Acampamento de Treinamento da Juventude do Japão!"
    })
}

// Rota para listar os jogadores do BD
#[get("/listar_jogadores")]
fn listar_jogadores(pool: &State<DbPool>) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let jogadores: Vec<Jogador> = conn.query_map(
        "SELECT j.id, j.nome, j.sobrenome, j.altura, j.posicao, e.nome AS escola, j.login, j.senha
         FROM jogadores j
         LEFT JOIN escolas e ON j.escola_id = e.id",
        |(id, nome, sobrenome, altura, posicao, escola, login, senha)| Jogador::new(id, nome, sobrenome, altura, posicao, escola, login, senha),
    ).expect("Falha ao buscar jogadores");

    Template::render("jogadores", context! {
        title: "Lista de jogadores",
        jogadores
    })
}

#[post("/adicionar_escola", data = "<nome>")]
fn adicionar_escola(pool: &State<DbPool>, nome: String) -> Redirect {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop("INSERT INTO escolas (nome) VALUES (?)", (&nome,))
        .expect("Erro ao adicionar escola");

    Redirect::to("/listar_escolas")
}


#[get("/listar_escolas")]
fn listar_escolas(pool: &State<DbPool>) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let escolas: Vec<(u32, String)> = conn.query_map(
        "SELECT id, nome FROM escolas",
        |(id, nome)| (id, nome),
    ).expect("Falha ao buscar escolas");

    Template::render("listar_escolas", context! {
        title: "Lista de Escolas",
        escolas
    })
}


// Rota para exibir o formulário 
#[get("/adicionar_jogador")]
fn exibir_formulario(pool: &State<DbPool>) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let escolas: Vec<(u32, String)> = conn.query_map(
        "SELECT id, nome FROM escolas",
        |(id, nome)| (id, nome),
    ).expect("Falha ao buscar escolas");

    Template::render("adicionar_jogador", context! {
        title: "Adicionar Jogador",
        escolas  // Passando as escolas para o template
    })
}


// Rota para processar os dados do formulário
#[post("/adicionar_jogador", data = "<jogador_input>")]
fn adicionar_jogador(pool: &State<DbPool>, jogador_input: Form<JogadorInput>) -> Template {
    let jogador = jogador_input.into_inner();
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop(
        "INSERT INTO jogadores (nome, sobrenome, altura, posicao, escola_id, login, senha) VALUES (?, ?, ?, ?, ?, ?, ?)",
        (&jogador.nome, &jogador.sobrenome, &jogador.altura, &jogador.posicao, &jogador.escola_id, &jogador.login, &jogador.senha),
    ).expect("Erro ao inserir jogador");

    Template::render("success", context! {
        title: "Jogador adicionado",
        message: format!("Jogador {} cadastrado com sucesso!", jogador.nome)
    })
}



// Rota POST para processar o formulário e renderizar a página de saudação
#[post("/submit", data = "<jogador_input>")]
fn submit(jogador_input: Form<JogadorInput>) -> Template {
    let jogador = jogador_input.into_inner();
    Template::render("greeting", context! {
        title: "Saudação",
        greeting_message: format!("Olá, {} {}!", jogador.nome, jogador.sobrenome)
    })
}

// Rota para lista de jogadores
#[get("/melhores_jogadores")]
fn melhores_jogadores() -> Template {

    let melhores_jogadores = vec![
        "Shoyo Hinata - Escola Secundária Karasuno",
        "Tobio Kageyama - Escola Secundária Karasuno",
        "Korai Hoshiumi - Colégio Kamomedai",
        "Kiyoomi Sakusa - Academia Itachiyama",
        "Atsumu Miya - Escola Secundária Inarizaki",
    ];

    Template::render("melhores_jogadores", context! {
        title: "Melhores jogadores",
        melhores_jogadores
    })
}


// Rota para deletar jogador
#[get("/deletar_jogador/<id>")]
fn deletar_jogador(pool: &State<DbPool>, id: u32) -> Redirect {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop("DELETE FROM jogadores WHERE id = ?", (id,))
        .expect("Erro ao deletar jogador");

    Redirect::to("/listar_jogadores")
}

// Página para editar jogador
#[get("/editar_jogador/<id>")]
fn editar_jogador(pool: &State<DbPool>, id: u32) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let jogadores: Vec<Jogador> = conn.exec_map(
        "SELECT id, nome, sobrenome, altura, posicao, escola, login, senha FROM jogadores WHERE id = ? LIMIT 1",
        (id,),
        |(id, nome, sobrenome, altura, posicao, escola, login, senha)| Jogador::new (id, nome, sobrenome, altura, posicao, escola, login, senha),
    ).expect("Erro ao buscar jogador");

    if let Some(jogador) = jogadores.into_iter().next() {
        Template::render("editar_jogador", context! { title: "Editar Jogador", jogador })
    } else {
        Template::render("error", context! { message: "Jogador não encontrado!" })
    }
}

// Rota para atualizar jogador
#[post("/atualizar_jogador/<id>", data = "<jogador_input>")]
fn atualizar_jogador(pool: &State<DbPool>, id: u32, jogador_input: Form<JogadorInput>) -> Redirect {
    let jogador = jogador_input.into_inner();
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop(
        "UPDATE jogadores SET nome = ?, sobrenome = ?, altura = ?, posicao = ?, escola_id = ?, login = ?, senha = ? WHERE id = ?",
        (&jogador.nome, &jogador.sobrenome, &jogador.altura, &jogador.posicao, &jogador.escola_id, &jogador.login, &jogador.senha, id),
    ).expect("Erro ao atualizar jogador");

    Redirect::to("/listar_jogadores")
}


// BANCO
#[launch]
fn rocket() -> _ {

    let url = "mysql://root:@localhost:3306/haikyuu";
    let pool = Pool::new(url).expect("Falha ao criar conexão com MySQL");

    // Criando a tabela se não existir
    let mut conn = pool.get_conn().expect("Falha ao conectar ao banco");
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS jogadores (
            id INT AUTO_INCREMENT PRIMARY KEY,
            nome VARCHAR(100),
            sobrenome VARCHAR(100),
            altura VARCHAR(10),
            posicao VARCHAR(50),
            escola_id INT,
            login VARCHAR(50) UNIQUE,
            senha VARCHAR(255),
            FOREIGN KEY (escola_id) REFERENCES escolas(id) ON DELETE SET NULL
        )"
    ).expect("Erro ao criar tabela jogadores");
    


    rocket::build()
        .manage(DbPool(pool)) // Adiciona a conexão ao estado do Rocket
        .mount("/", routes![index, submit, listar_jogadores, melhores_jogadores, exibir_formulario, adicionar_jogador, editar_jogador, atualizar_jogador, deletar_jogador, adicionar_escola,listar_escolas ])
        .attach(Template::fairing()) // Anexa o fairing do Handlebars para processar templates
}