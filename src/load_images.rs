use raylib::prelude::*;

#[derive(Debug)]
pub struct Images {
    pub textures: Vec<Texture2D>,
}

// let i = Image::load_image("/home/mikel/Escritorio/ajedrez/src/static/peon_blanco.png")
//         .expect("UNABLE TO LOAD");
//     let _ = rl
//         .load_texture(
//             &thread,
//             "/home/mikel/Escritorio/ajedrez/src/static/peon_blanco.png",
//         )
//         .expect("could not load texture billboard");
//     let t = rl
//         .load_texture_from_image(&thread, &i)
//         .expect("could not load texture from image");

impl Images {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let paths = [
            "/home/mikel/Escritorio/ajedrez/src/static/peon_blanco.png",
            "/home/mikel/Escritorio/ajedrez/src/static/alfil_blanco.png",
            "/home/mikel/Escritorio/ajedrez/src/static/caballo_blanco.png",
            "/home/mikel/Escritorio/ajedrez/src/static/torre_blanco.png",
            "/home/mikel/Escritorio/ajedrez/src/static/rey_blanco.png",
            "/home/mikel/Escritorio/ajedrez/src/static/reina_blanco.png",
            "/home/mikel/Escritorio/ajedrez/src/static/peon_negro.png",
            "/home/mikel/Escritorio/ajedrez/src/static/alfil_negro.png",
            "/home/mikel/Escritorio/ajedrez/src/static/caballo_negro.png",
            "/home/mikel/Escritorio/ajedrez/src/static/torre_negro.png",
            "/home/mikel/Escritorio/ajedrez/src/static/rey_negro.png",
            "/home/mikel/Escritorio/ajedrez/src/static/reina_negro.png",
        ];

        let mut textures = Vec::new();

        for path in paths.iter() {
            let i = Image::load_image(path)
                .expect(&format!("ERROR CARGANDO IMAGEN {}", path).to_string());
            let _ = rl.load_texture(thread, path);
            textures.push(rl.load_texture_from_image(thread, &i).unwrap());
        }

        Self { textures }
    }
}
