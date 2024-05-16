use crate::pieza::*;
use crate::vector2::Vect2;

pub const N: usize = 8;

#[derive(Debug)]
pub struct Tablero {
    pub piezas: [[Option<Pieza>; N]; N],
    pub turno: Color,
    pub ganador: Option<Color>,
    pub valor_blancas: i32,
    pub valor_negras: i32,
}

impl Tablero {
    pub fn new() -> Self {
        let mut piezas = [[None; N]; N];

        piezas[0] = [
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Torre,
                Vect2::new(0, 0),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Caballo,
                Vect2::new(0, 1),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Alfil,
                Vect2::new(0, 2),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Reina,
                Vect2::new(0, 3),
            )),
            Some(Pieza::new(Color::Blancas, TipoPieza::Rey, Vect2::new(0, 4))),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Alfil,
                Vect2::new(0, 5),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Caballo,
                Vect2::new(0, 6),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Torre,
                Vect2::new(0, 7),
            )),
        ];

        for i in 0..N {
            piezas[1][i] = Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Peon,
                Vect2::new(1, i),
            ));
        }

        piezas[N - 1] = [
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Torre,
                Vect2::new(N - 1, 0),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Caballo,
                Vect2::new(N - 1, 1),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Alfil,
                Vect2::new(N - 1, 2),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Reina,
                Vect2::new(N - 1, 3),
            )),
            Some(Pieza::new(Color::Blancas, TipoPieza::Rey, Vect2::new(0, 4))),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Alfil,
                Vect2::new(N - 1, 5),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Caballo,
                Vect2::new(N - 1, 6),
            )),
            Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Torre,
                Vect2::new(N - 1, 7),
            )),
        ];

        for i in 0..N {
            piezas[6][i] = Some(Pieza::new(
                Color::Blancas,
                TipoPieza::Peon,
                Vect2::new(1, i),
            ));
        }

        Self {
            piezas,
            ganador: None,
            turno: Color::Blancas,
            valor_blancas: 0,
            valor_negras: 0,
        }
    }
}
