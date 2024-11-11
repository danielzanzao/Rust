mod livro;

use livro::{Genero, Livro};
use chrono::NaiveDate;

fn main() {
    //Criando um livro qualquer
    let livro = Livro::new(
        "Diário de um banana".to_string(),
        1178,
        NaiveDate::from_ymd(29,7,2007),
        Genero::Quadrinhos,
    );

    // Salvar o livro no txt
    match livro {
        Ok(livro) => {
            if let Err(e) = livro.salvar("livros.txt") {
                eprintln!("Erro ao salvar o livro: {}", e);
            }
        }
        Err(e) => eprintln!("Erro ao criar o livro: {}", e),
    }

    // Carregar e exibir todos os livros armazenados
    match Livro::carregar_livros("livros.txt") {
        Ok(livros) => {
            for livro in livros {
                println!("{:?}", livro);
            }
        }
        Err(e) => eprintln!("Erro ao carregar livros: {}", e),
    }
}
