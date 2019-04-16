// The device which renders the objects and draws them to the screen. They are all contained in
// this one device struct which is why I'm giving it its own file.

use crate::render_objects::*;
use crate::structures::*;

pub struct Device {
    // The width and height of the screen. This will of course be the dimensions of the terminal.
    pub dimensions: (usize, usize),
    pub camera: Camera,
    pub meshes: Vec<Mesh>,
    pub pixels: Vec<Colour>,
}

impl Device {
    pub fn new(camera: Camera, meshes: Vec<Mesh>, colour_space: Colour) -> Device {
        let colour = match colour_space {
            Colour::Rgba(r,g,b,a) => Colour::Rgba(0.0, 0.0, 0.0, 0.0),
            Colour::Grey(a) => Colour::Grey(0.0),
        };

        let dimensions = term_size::dimensions().unwrap();

        Device {
            dimensions: term_size::dimensions().unwrap(),
            camera: camera,
            meshes: meshes,
            pixels: vec![colour; dimensions.0 * dimensions.1],
        }
    }

    // Make an array of pixels, probably one dimensional array. Then make a function that takes in
    // an x and y and sets that pixel to that colour. Then make functions for lines and triangles.

    pub fn draw_point(self, x: usize, y: usize, colour: Colour) {
        println!("\033[2J");
    }
}
