extern crate term_size;

mod structures;
mod render_objects;
mod device;

use structures::*;
use render_objects::*;
use device::*;

fn main() {
    let m = Mesh::from_file(String::from("test"), String::from("cube.txt"), Vector::from(1.0, 1.0, 1.0, 1.0), Vector::new());
    let camera = Camera::from(Vector::from(3.0, 4.0, 5.0, 1.0), Vector::new(), Vector::from(0.0, 1.0, 0.0, 0.0));

    let device = Device::new(camera, vec![m], Colour::Grey(0.0));
    println!("\033[2J");
}
