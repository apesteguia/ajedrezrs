use raylib::prelude::*;

pub mod pieza;
pub mod tablero;
pub mod vector2;

const SIZE: i32 = 800;
const N: usize = 8;
const DIM: i32 = SIZE / N as i32;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SIZE, SIZE)
        .title("Hello, World")
        .build();
    let tablero = tablero::Tablero::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        for i in 0..N {
            for j in 0..N {
                let c: Color;
                let b: Color;
                if j % 2 == 0 {
                    if i % 2 == 0 {
                        c = Color::BLACK;
                        b = Color::WHITE;
                    } else {
                        c = Color::WHITE;
                        b = Color::BLACK;
                    }
                } else {
                    if i % 2 == 0 {
                        c = Color::WHITE;
                        b = Color::BLACK;
                    } else {
                        c = Color::BLACK;
                        b = Color::WHITE;
                    }
                }
                match tablero.piezas[j][i] {
                    // Camiar porque sino esta el tablero en direccion
                    // horizontal
                    Some(pieza) => {
                        let f = format!("{:?}", pieza.tipo);
                        d.draw_rectangle(i as i32 * DIM, j as i32 * DIM, DIM, DIM, b);
                        d.draw_text(&f, i as i32 * DIM, j as i32 * DIM, 5, c);
                    }
                    None => {
                        d.draw_rectangle(i as i32 * DIM, j as i32 * DIM, DIM, DIM, b);
                        d.draw_text("Vacia", i as i32 * DIM, j as i32 * DIM, 5, c)
                    }
                }
            }
        }
    }
}
