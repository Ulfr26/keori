// The device which renders the objects and draws them to the screen. They are all contained in
// this one device struct which is why I'm giving it its own file.

use crate::render_objects::*;
use crate::structures::*;

pub struct Device {
    // The width and height of the screen. This will of course be the dimensions of the terminal.
    pub dimensions: (usize, usize),
    pub camera: Camera [ID],
    pub meshes: Vec<Mesh>,
    pub pixels: Vec<Color>,
}

impl Device {
    pub fn new(camera: Camera, meshes: Vec<Mesh>, colour_space: Color) -> Device {
        let colour = match colour_space {
            Color::Rgba => Color::Rgba(0.0, 0.0, 0.0, 0.0),
            Color::Grey => Color::Grey(0.0),
        }

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

    pub fn render(&self) {
        // The main function that renders all the meshes to the screen. Let's do this!
        // First, create the MVP matrixes - Model, View, Projection - for the meshes
        let mut mvp_matricies: Vec<Matrix> = Vec::new();

        // Model matrix - the position of the mesh
        let mut model_matricies: Vec<Matrix> = Vec::new();

        for i in 0..self.meshes.len() {
            let mut m = Matrix::new();
            
            m.vals = [[1.0, 0.0, 0.0, self.meshes[i].pos.x],
                      [0.0, 1.0, 0.0, self.meshes[i].pos.y],
                      [0.0, 0.0, 1.0, self.meshes[i].pos.z],
                      [0.0, 0.0, 0.0, 1.0]];

            model_matricies.push(m);
        }

        //TODO: THE REST OF THE MVP
    }
}
