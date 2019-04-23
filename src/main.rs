extern crate term_size;

mod structures;
mod render_objects;
mod device;

use structures::*;
use render_objects::*;
use device::*;
use std::{thread, time};

fn main() {
    let m = Mesh::from_file(String::from("objects/donut.obj"), Vector::from(2.0, 3.0, 4.0, 1.0), Vector::new()).unwrap();
    let camera = Camera::from(Vector::from(3.0, 4.0, 5.0, 1.0), Vector::new(), Vector::from(0.0, 1.0, 0.0, 0.0));
    let device = Device::new(camera, vec![m], Colour::Grey(0.0));
}
