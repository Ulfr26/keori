// The objects that help render and are to be rendered. Built on the data structures as defined in
// structures.rs.

use crate::structures::Vector;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub enum Colour {
    Rgba(f64, f64, f64, f64), // (r, g, b, a)
    Grey(f64),                // (a)
}

// The point of view of which to render from
#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: Vector,
    pub target: Vector,
    pub up: Vector,
}

#[derive(Debug, Clone)]
pub struct Face<'a> {
    pub verticies: [&'a Vector; 3],
    pub colour: Colour,
}

// A mesh to be rendered. Contains vertex information and the like.
#[derive(Debug, Clone)]
pub struct Mesh<'a> {
    pub name: String,
    pub verticies: Vec<Vector>,
    pub faces: Vec<Face<'a>>,
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

impl<'a> Mesh<'a> {
    pub fn new(name: String) -> Mesh<'a> {
        Mesh {
            name: name,
            verticies: Vec::new(),
            faces: Vec::new(),
            pos: Vector::new(),
            rot: Vector::new(),
        }
    }

    pub fn from(name: String, verticies: Vec<Vector>, faces: Vec<Face<'a>>, pos: Vector, rot: Vector) -> Mesh<'a> {
        // This function is pretty much useless because actually implementing it would be horrific.
        // Instead, use from_file.
        Mesh {
            name: name,
            verticies: verticies,
            faces: faces,
            pos: pos,
            rot: rot,
        }
    }

    // Reads a file containing vector information and returns a mesh.
    // Much easier than just making a vector with the information like *some people I know*
    pub fn from_file(name: String, filename: String, pos: Vector, rot: Vector) -> Result<Mesh<'a>, String> {
        let mut f: File = File::open(filename).unwrap();
        let mut contents = String::new();

        f.read_to_string(&mut contents).unwrap();
        
        let mut lines: Vec<&str> = contents.split("\n").collect();
        let mut vertex_data: Vec<Vector> = Vec::new();
        let mut face_data: Vec<Face> = Vec::new();

        let mut mesh = Mesh {
            name: name,
            verticies: Vec::new(),
            faces: Vec::new(),
            pos: pos,
            rot: rot,
        };

        for i in 0..(lines.len()-1) { // Note that the last element of lines is an empty list.
            let mut l: Vec<&str> = lines[i].split(" ").collect();

            match l[0] {
                "v" => {
                    let mut v = Vector::new();

                    v.x = l[1].parse::<f64>().unwrap();
                    v.y = l[2].parse::<f64>().unwrap();
                    v.z = l[3].parse::<f64>().unwrap();
                    v.w = 1.0;

                    mesh.verticies.push(v);
                },

                "f" => {
                    let mut f = Face::new(
                        &mesh.verticies[l[1].parse::<usize>().unwrap()],
                        &mesh.verticies[l[2].parse::<usize>().unwrap()],
                        &mesh.verticies[l[3].parse::<usize>().unwrap()],
                        Colour::Grey(0.0)
                    );
                },

                _ => return Err(String::from("Error in file")),
            }
        }

        Ok(mesh) 
    }
}

impl<'a> Face<'a> {
    fn new(v1: &'a Vector, v2: &'a Vector, v3: &'a Vector, colour: Colour) -> Face<'a> {
        Face {
            verticies: [v1, v2, v3],
            colour: colour,
        }
    } 

    fn normal(&self) -> Vector {
        // TODO: Returns the cross product v1 x v2 where v1 is the vector between vertex 1 and
        // vertex 2 and v2 is the vector between vertex 2 and vertex 3.
        Vector::new()
    }
}
