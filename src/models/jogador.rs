use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jogador {
    id: u32,
    nome: String,
    sobrenome: String,
    altura: String,
    posicao: String,
    escola: String,
    login: String,
    senha: String,
}

impl Jogador {
    pub fn new(id: u32, nome: String, sobrenome: String, altura: String, posicao: String, escola: String, login: String, senha: String) -> Self {
        Jogador { id, nome, sobrenome, altura, posicao, escola, login, senha }
    }
}