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
                Color::Negras,
                TipoPieza::Torre,
                Vect2::new(N - 1, 0),
            )),
            Some(Pieza::new(
                Color::Negras,
                TipoPieza::Caballo,
                Vect2::new(N - 1, 1),
            )),
            Some(Pieza::new(
                Color::Negras,
                TipoPieza::Alfil,
                Vect2::new(N - 1, 2),
            )),
            Some(Pieza::new(
                Color::Negras,
                TipoPieza::Reina,
                Vect2::new(N - 1, 3),
            )),
            Some(Pieza::new(Color::Negras, TipoPieza::Rey, Vect2::new(0, 4))),
            Some(Pieza::new(
                Color::Negras,
                TipoPieza::Alfil,
                Vect2::new(N - 1, 5),
            )),
            Some(Pieza::new(
                Color::Negras,
                TipoPieza::Caballo,
                Vect2::new(N - 1, 6),
            )),
            Some(Pieza::new(
                Color::Negras,
                TipoPieza::Torre,
                Vect2::new(N - 1, 7),
            )),
        ];

        for i in 0..N {
            piezas[6][i] = Some(Pieza::new(Color::Negras, TipoPieza::Peon, Vect2::new(6, i)));
        }

        Self {
            piezas,
            ganador: None,
            turno: Color::Blancas,
            valor_blancas: 0,
            valor_negras: 0,
        }
    }

    pub fn movimientos_posibles(&self, pos: Vect2<usize>) -> Option<Vec<Vect2<usize>>> {
        let pieza = match self.piezas[pos.x][pos.y] {
            Some(pieza) => pieza,
            None => return None,
        };
        //sabemos que no esta vacia
        let tipo = pieza.tipo.unwrap();

        match tipo {
            TipoPieza::Peon => return Some(self.movimiento_peon(&pieza)),
            TipoPieza::Alfil => return Some(self.movimiento_alfil()),
            TipoPieza::Caballo => return Some(self.movimiento_caballo()),
            TipoPieza::Torre => return Some(self.movimiento_torre()),
            TipoPieza::Rey => return Some(self.movimiento_rey()),
            TipoPieza::Reina => return Some(self.movimiento_reina()),
        };
    }

    pub fn insertar_pieza(&mut self, p: Option<Pieza>, v: Vect2<usize>) {
        match p {
            Some(pieza) => {
                self.piezas[pieza.pos.x][pieza.pos.x] = None;
                self.piezas[v.x][v.y] =
                    Some(Pieza::new(pieza.color.unwrap(), pieza.tipo.unwrap(), v))
            }
            None => self.piezas[v.x][v.y] = None,
        }
    }

    // Hacia arriba, Derecha
    // 0 1 -> Caballo blanco
    // 0 0 -> Torre blanco
    // 1 0 -> Peon blanco
    fn movimiento_peon(&self, p: &Pieza) -> Vec<Vect2<usize>> {
        let mut v: Vec<Vect2<usize>> = Vec::new();

        match p.color.unwrap() {
            Color::Blancas => {
                if p.pos.x == 1 {
                    if self.piezas[p.pos.x + 1][p.pos.y].is_none() {
                        v.push(Vect2::new(p.pos.x + 1, p.pos.y));
                        if self.piezas[p.pos.x + 2][p.pos.y].is_none() {
                            v.push(Vect2::new(p.pos.x + 2, p.pos.y));
                        } else {
                            let pieza_adelante = &self.piezas[p.pos.x + 2][p.pos.y];
                            if let Some(pieza) = pieza_adelante {
                                if pieza.color.unwrap() == Color::Negras {
                                    v.push(Vect2::new(p.pos.x + 2, p.pos.y));
                                }
                            }
                        }
                    } else {
                        let pieza_adelante = &self.piezas[p.pos.x + 1][p.pos.y];
                        if let Some(pieza) = pieza_adelante {
                            if pieza.color.unwrap() == Color::Negras {
                                v.push(Vect2::new(p.pos.x + 1, p.pos.y));
                                return v;
                            }
                        }
                    }
                } else {
                    if p.pos.x > 1 && p.pos.x < 7 {
                        if self.piezas[p.pos.x + 1][p.pos.y].is_none() {
                            v.push(Vect2::new(p.pos.x + 1, p.pos.y));
                        } else {
                            let pieza_adelante = &self.piezas[p.pos.x + 1][p.pos.y];
                            if let Some(pieza) = pieza_adelante {
                                if pieza.color.unwrap() == Color::Negras {
                                    v.push(Vect2::new(p.pos.x + 1, p.pos.y));
                                }
                            }
                        }
                    }
                }
            }

            Color::Negras => {
                if p.pos.x == 6 {
                    if self.piezas[p.pos.x - 1][p.pos.y].is_none() {
                        v.push(Vect2::new(p.pos.x - 1, p.pos.y));
                        if self.piezas[p.pos.x - 2][p.pos.y].is_none() {
                            v.push(Vect2::new(p.pos.x - 2, p.pos.y));
                        } else {
                            let pieza_adelante = &self.piezas[p.pos.x - 2][p.pos.y];
                            if let Some(pieza) = pieza_adelante {
                                if pieza.color.unwrap() == Color::Blancas {
                                    v.push(Vect2::new(p.pos.x - 2, p.pos.y));
                                }
                            }
                        }
                    } else {
                        let pieza_adelante = &self.piezas[p.pos.x - 1][p.pos.y];
                        if let Some(pieza) = pieza_adelante {
                            if pieza.color.unwrap() == Color::Blancas {
                                v.push(Vect2::new(p.pos.x - 1, p.pos.y));
                                return v;
                            }
                        }
                    }
                } else {
                    if p.pos.x < 6 && p.pos.x > 1 {
                        if self.piezas[p.pos.x - 1][p.pos.y].is_none() {
                            v.push(Vect2::new(p.pos.x - 1, p.pos.y));
                        } else {
                            let pieza_adelante = &self.piezas[p.pos.x - 1][p.pos.y];
                            if let Some(pieza) = pieza_adelante {
                                if pieza.color.unwrap() == Color::Blancas {
                                    v.push(Vect2::new(p.pos.x - 1, p.pos.y));
                                }
                            }
                        }
                    }
                }
            }
        }

        // v.push(Vect2::new(p.pos.x, p.pos.y));
        v
    }

    fn movimiento_alfil(&self) -> Vec<Vect2<usize>> {
        let v: Vec<Vect2<usize>> = Vec::new();
        // Lógica para los movimientos del alfil
        v
    }

    fn movimiento_caballo(&self) -> Vec<Vect2<usize>> {
        let v: Vec<Vect2<usize>> = Vec::new();
        // Lógica para los movimientos del caballo
        v
    }

    fn movimiento_torre(&self) -> Vec<Vect2<usize>> {
        let v: Vec<Vect2<usize>> = Vec::new();
        // Lógica para los movimientos de la torre
        v
    }

    fn movimiento_rey(&self) -> Vec<Vect2<usize>> {
        let v: Vec<Vect2<usize>> = Vec::new();
        // Lógica para los movimientos del rey
        v
    }

    fn movimiento_reina(&self) -> Vec<Vect2<usize>> {
        let v: Vec<Vect2<usize>> = Vec::new();
        // Lógica para los movimientos de la reina
        v
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mover_peon() {
        let mut v: Vec<Vect2<usize>> = Vec::new();
        v.push(Vect2::new(1, 1));
        let t = Tablero::new();
        let res = t.movimientos_posibles(Vect2::new(6, 0));

        // 0 1 -> Caballo blanco
        // 0 0 -> Torre blanco
        // 1 0 -> Peon blanco
        // println!("{:?}", t.piezas[6][0].unwrap().tipo.unwrap());
        // println!("{:?}", t.piezas[6][0].unwrap().color.unwrap());

        let a = res.unwrap();
        println!("{:?}", a);
        assert_eq!(a.is_empty(), false);
    }
}
