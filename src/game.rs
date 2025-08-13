#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Point {
    pub y: isize,
    pub x: isize,
}

impl Point {
    pub fn new(y: isize, x: isize) -> Self {
        Self { x, y }
    }
}

pub enum Direcao {
    Cima,
    Baixo,
    Esquerda,
    Direita,
}

pub fn parse_direcao(input: &str) -> Option<Direcao> {
    match input.trim().to_uppercase().as_str() {
        "W" => Some(Direcao::Cima),
        "S" => Some(Direcao::Baixo),
        "A" => Some(Direcao::Esquerda),
        "D" => Some(Direcao::Direita),
        _ => None,
    }
}
