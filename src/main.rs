use pieza::Pieza;
use raylib::prelude::*;
use vector2::Vect2;

pub mod load_images;
pub mod pieza;
pub mod tablero;
pub mod vector2;

const SIZE: i32 = 800;
const N: usize = 8;
const DIM: i32 = SIZE / N as i32;
const POS: i32 = DIM;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SIZE, SIZE)
        .title("Hello, World")
        .build();
    let mut tablero = tablero::Tablero::new();
    tablero.insertar_pieza(
        Some(Pieza::new(
            pieza::Color::Negras,
            pieza::TipoPieza::Peon,
            Vect2::new(2, 0),
        )),
        Vect2::new(2, 0),
    );
    tablero.insertar_pieza(
        Some(Pieza::new(
            pieza::Color::Blancas,
            pieza::TipoPieza::Peon,
            Vect2::new(5, 2),
        )),
        Vect2::new(5, 2),
    );

    let mut movimientos: Vec<Vect2<usize>> = Vec::new();
    let images = load_images::Images::new(&mut rl, &thread);
    let my_black = Color::new(70, 70, 70, 255);
    rl.set_target_fps(20);
    let mut seleccionado: Vect2<usize> = Vect2::new(0, 0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let pos = d.get_mouse_position();
        // println!("{:?}", pos);
        let down = d.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);

        if down {
            let col = check_collision(pos);
            if !movimientos.is_empty() {
                let yy = movimientos.iter().any(|&f| f.igual(col));
                if yy {
                    let piz = tablero.piezas[seleccionado.x][seleccionado.y];
                    tablero.insertar_pieza(piz, col);
                }
            } else {
                if tablero.piezas[col.x][col.y].unwrap().color.unwrap() == tablero.turno {
                    movimientos = tablero.movimientos_posibles(col).unwrap();
                }
            }
            seleccionado = col;
        }

        for i in 0..N {
            for j in 0..N {
                let c: Color;
                let b: Color;
                if j % 2 == 0 {
                    if i % 2 == 0 {
                        c = my_black;
                        b = Color::WHITE;
                    } else {
                        c = Color::WHITE;
                        b = my_black;
                    }
                } else {
                    if i % 2 == 0 {
                        c = Color::WHITE;
                        b = my_black;
                    } else {
                        c = my_black;
                        b = Color::WHITE;
                    }
                }
                match tablero.piezas[j][i] {
                    // Camiar porque sino esta el tablero en direccion
                    // horizontal
                    Some(pieza) => {
                        let _f = format!("{:?}", pieza.tipo);
                        let ij = Vect2::new(j, i);
                        let x = movimientos.iter().any(|&f| f.igual(ij));
                        // println!("{:?}", movimientos);
                        if x {
                            d.draw_rectangle(i as i32 * DIM, j as i32 * DIM, DIM, DIM, Color::RED);
                        } else {
                            d.draw_rectangle(i as i32 * DIM, j as i32 * DIM, DIM, DIM, b);
                        }
                        d.draw_text(
                            &format!("{}{}", pieza.pos.x, pieza.pos.y),
                            i as i32 * DIM,
                            j as i32 * DIM,
                            5,
                            c,
                        );
                        if pieza.color.unwrap() == pieza::Color::Negras {
                            match pieza.tipo {
                                Some(tipo) => match tipo {
                                    pieza::TipoPieza::Peon => d.draw_texture(
                                        &images.textures[6],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Alfil => d.draw_texture(
                                        &images.textures[7],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Caballo => d.draw_texture(
                                        &images.textures[8],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Torre => d.draw_texture(
                                        &images.textures[9],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Rey => d.draw_texture(
                                        &images.textures[10],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Reina => d.draw_texture(
                                        &images.textures[11],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                },
                                // d.draw_texture(&t, i as i32 * DIM, j as i32 * DIM, Color::WHITE);
                                None => (),
                            }
                        } else {
                            match pieza.tipo {
                                Some(tipo) => match tipo {
                                    pieza::TipoPieza::Peon => d.draw_texture(
                                        &images.textures[0],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Alfil => d.draw_texture(
                                        &images.textures[1],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Caballo => d.draw_texture(
                                        &images.textures[2],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Torre => d.draw_texture(
                                        &images.textures[3],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Rey => d.draw_texture(
                                        &images.textures[4],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                    pieza::TipoPieza::Reina => d.draw_texture(
                                        &images.textures[5],
                                        i as i32 * POS,
                                        j as i32 * POS,
                                        Color::WHITE,
                                    ),
                                },
                                // d.draw_texture(&t, i as i32 * DIM, j as i32 * DIM, Color::WHITE);
                                None => (),
                            }
                        }
                        // d.draw_texture(&t, i as i32 * DIM, j as i32 * DIM, Color::WHITE);
                        // d.draw_text(&f, i as i32 * DIM, j as i32 * DIM, 5, c);
                    }
                    None => {
                        let ij = Vect2::new(j, i);
                        let x = movimientos.iter().any(|&f| f.igual(ij));
                        if x {
                            d.draw_rectangle(i as i32 * DIM, j as i32 * DIM, DIM, DIM, Color::RED);
                        } else {
                            d.draw_rectangle(i as i32 * DIM, j as i32 * DIM, DIM, DIM, b);
                        }
                        d.draw_text(
                            &format!("Vacia {}{}", i, j),
                            i as i32 * DIM,
                            j as i32 * DIM,
                            5,
                            c,
                        );
                    }
                }
            }
        }
    }
}

fn check_collision(v: Vector2) -> Vect2<usize> {
    let mut result = Vect2 { x: 0, y: 0 };

    result.x = (v.y / 100.0) as usize;
    result.y = (v.x / 100.0) as usize;

    result
}
