// The objects that help render and are to be rendered. Built on the data structures as defined in
// structures.rs.

use crate::structures::Vector;
use std::fs::File;
use std::io::prelude::*;

// The point of view of which to render from
#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: Vector,
    pub target: Vector,
    pub up: Vector,
}

// A mesh to be rendered. Contains vertex information and the like.
#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub verticies: Vec<Vector>,
    pub pos: Vector,
    pub rot: Vector,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: Vector::new(),
            target: Vector::new(),
            up: Vector::from(0.0, 1.0, 0.0, 0.0),
        }
    }

    pub fn from(pos: Vector, target: Vector, up: Vector) -> Camera {
        Camera {
            pos: pos,
            target: target,
            up: up,
        }
    }
}

impl Mesh {
    pub fn new(name: String) -> Mesh {
        Mesh {
            name: name,
            verticies: Vec::new(),
            pos: Vector::new(),
            rot: Vector::new(),
        }
    }

    pub fn from(name: String, verticies: Vec<Vector>, pos: Vector, rot: Vector) -> Mesh {
        Mesh {
            name: name,
            verticies: verticies,
            pos: pos,
            rot: rot,
        }
    }

    // Reads a file containing vector information and returns a mesh.
    // Much easier than just making a vector with the information like *some people I know*
    pub fn from_file(name: String, filename: String, pos: Vector, rot: Vector) -> Mesh {
        let mut f: File = File::open(filename).unwrap();
        let mut contents = String::new();

        f.read_to_string(&mut contents).unwrap();

        // The file data should be in the form:
        // 
        // x1 y1 z1
        // x2 y2 z2
        // x3 y3 x3
        // ...
        //
        // And so on for all the vectors. This should give an easy to input and easy to parse file
        // from which we can create our mesh
        
        let mut lines: Vec<&str> = contents.split("\n").collect();
        let mut vec_data: Vec<Vector> = Vec::new();

        for i in 0..(lines.len()-1) { // Note that the last element of lines is an empty list.
            let mut l: Vec<&str> = lines[i].split(" ").collect();
            let mut v = Vector::new();

            v.x = l[0].parse::<f64>().unwrap();
            v.y = l[1].parse::<f64>().unwrap();
            v.z = l[2].parse::<f64>().unwrap();
            v.w = 1.0;

            vec_data.push(v);
        }

        Mesh {
            name: name,
            verticies: vec_data,
            pos: pos,
            rot: rot,
        }
    }
}
