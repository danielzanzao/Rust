use chrono::NaiveDate;
use std::fmt::{self, Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::str::FromStr;

//Os gêneros dos livros
#[derive(Debug)]
pub enum Genero {
    Ficcao,
    NaoFiccao,
    Ciencia,
    Historia,
    Biografia,
    Quadrinhos,
    Outro,
}

impl Display for Genero {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let genero_str = match *self {
            Genero::Ficcao => "Ficcao",
            Genero::NaoFiccao => "NaoFiccao",
            Genero::Ciencia => "Ciencia",
            Genero::Historia => "Historia",
            Genero::Biografia => "Biografia",
            Genero::Quadrinhos => "Quadrinhos",
            Genero::Outro => "Outro",
        };
        write!(f, "{}", genero_str)
    }
}

impl FromStr for Genero {
    type Err = String;

    fn from_str(input: &str) -> Result<Genero, Self::Err> {
        match input {
            "Ficcao" => Ok(Genero::Ficcao),
            "NaoFiccao" => Ok(Genero::NaoFiccao),
            "Ciencia" => Ok(Genero::Ciencia),
            "Historia" => Ok(Genero::Historia),
            "Biografia" => Ok(Genero::Biografia),
            "Quadrinhos" => Ok(Genero::Quadrinhos),
            "Outro" => Ok(Genero::Outro),
            _ => Err("Gênero inválido".to_string()),
        }
    }
}
//Struct do Livro
#[derive(Debug)]
pub struct Livro {
    pub titulo: String,
    pub num_paginas: u32,
    pub data_publicacao: NaiveDate,
    pub genero: Genero,
}

impl Livro {
    // Cria um novo livro validando ele
    pub fn new(titulo: String, num_paginas: u32, data_publicacao: NaiveDate, genero: Genero) -> Result<Self, String> {
        if titulo.is_empty() {
            return Err("Título não pode estar vazio.".to_string());
        }
        if num_paginas == 0 {
            return Err("O número de páginas deve ser maior que zero.".to_string());
        }

        Ok(Livro {
            titulo,
            num_paginas,
            data_publicacao,
            genero,
        })
    }

    // Função de persistência
    pub fn salvar(&self, arquivo: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(arquivo)?;
        let mut writer = BufWriter::new(file);

        writeln!(
            writer,
            "{};{};{};{}",
            self.titulo,
            self.num_paginas,
            self.data_publicacao.format("%d-%m-%Y"),
            self.genero
        )?;
        Ok(())
    }

    // Função de load dos livros
    pub fn carregar_livros(arquivo: &str) -> io::Result<Vec<Livro>> {
        let path = Path::new(arquivo);
        if !path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut livros = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let campos: Vec<&str> = line.split(';').collect();

            if campos.len() != 4 {
                continue;
            }

            let titulo = campos[0].to_string();
            let num_paginas = campos[1].parse::<u32>().unwrap_or(0);
            let data_publicacao = NaiveDate::parse_from_str(campos[2], "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd(1900, 1, 1));
            let genero = campos[3].parse::<Genero>().unwrap_or(Genero::Outro);

            if let Ok(livro) = Livro::new(titulo, num_paginas, data_publicacao, genero) {
                livros.push(livro);
            }
        }

        Ok(livros)
    }
}
