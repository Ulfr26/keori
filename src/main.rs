extern crate term_size;

mod structures;
mod render_objects;
mod device;

use structures::*;
use render_objects::*;
use device::*;

fn main() {
    let m = Mesh::from_file(String::from("test"), String::from("cube.txt"), Vector::from(1.0, 1.0, 1.0, 1.0), Vector::new()).unwrap();
    let camera = Camera::from(Vector::from(3.0, 4.0, 5.0, 1.0), Vector::new(), Vector::from(0.0, 1.0, 0.0, 0.0));

    let device = Device::new(camera, vec![m], Colour::Grey(0.0));

    device.clear_screen();
    //device.draw_triangle((2.0, 3.0), (20.0, 40.0), (80.0, 30.0), Colour::Grey(1.0), true);
    device.draw_triangle((12.0, 3.0), (30.0, 50.0), (90.0, 40.0), Colour::Grey(1.0), false);
}
