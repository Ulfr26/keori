extern crate term_size;

mod structures;
mod render_objects;
mod device;

use structures::*;
use render_objects::*;
use device::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut m = Mesh::from_file(String::from("objects/donut.obj"), Vector::from(2.0, 3.0, 4.0, 1.0), Vector::new()).unwrap();
    let mut angle = 0.01f64;

    let camera = Camera::from(Vector::from(3.0, 4.0, 5.0, 1.0), Vector::new(), Vector::from(0.0, 1.0, 0.0, 0.0));
    let mut device = Device::new(camera, vec![m.clone()], Colour::Grey(0.0));


    loop {
        device.clear_screen();
        device.test_render();
        device.meshes[0] = m.clone()*Matrix::rotation(angle, Vector::from(1.0, 1.0, 1.0, 0.0));
        thread::sleep(Duration::from_millis(33));

        angle += 0.2f64;
    }
}
