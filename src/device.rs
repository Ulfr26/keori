// The device which renders the objects and draws them to the screen. They are all contained in
// this one device struct which is why I'm giving it its own file.

use crate::render_objects::*;
use crate::structures::*;

pub struct Device {
    // The width and height of the screen. This will of course be the dimensions of the terminal.
    pub dimensions: (usize, usize),
    pub camera: Camera,
    pub meshes: Vec<Mesh>,
}

impl Device {
    pub fn new(camera: Camera, meshes: Vec<Mesh>) -> Device {
        Device {
            dimensions: term_size::dimensions().unwrap(),
            camera: camera,
            meshes: meshes,
        }
    }

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
