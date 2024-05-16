use crate::vector2::Vect2;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TipoPieza {
    Peon,
    Alfil,
    Caballo,
    Torre,
    Rey,
    Reina,
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Blancas,
    Negras,
}

#[derive(Debug, Copy, Clone)]
pub struct Pieza {
    pub vacia: bool,
    pub pos: Vect2<usize>,
    pub color: Option<Color>,
    pub tipo: Option<TipoPieza>,
}

impl Pieza {
    pub fn new(color: Color, tipo: TipoPieza, pos: Vect2<usize>) -> Self {
        Self {
            vacia: false,
            pos,
            color: Some(color),
            tipo: Some(tipo),
        }
    }
    pub fn vacia(pos: Vect2<usize>) -> Self {
        Self {
            vacia: true,
            pos,
            color: None,
            tipo: None,
        }
    }
}
