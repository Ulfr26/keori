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
pub struct Face {
    pub vertices: [usize; 3],
    pub colour: Colour,
    pub normal: Vector,
}

// A mesh to be rendered. Contains vertex information and the like.
#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vector>,
    pub faces: Vec<Face>,
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
            vertices: Vec::new(),
            faces: Vec::new(),
            pos: Vector::new(),
            rot: Vector::new(),
        }
    }

    pub fn from(name: String, vertices: Vec<Vector>, faces: Vec<Face>, pos: Vector, rot: Vector) -> Mesh {
        // This function is pretty much useless because actually implementing it would be horrific.
        // Instead, use from_file.
        Mesh {
            name: name,
            vertices: vertices,
            faces: faces,
            pos: pos,
            rot: rot,
        }
    }

    // Reads a file containing vector information and returns a mesh.
    // Much easier than just making a vector with the information like *some people I know*
    pub fn from_file(filename: String, pos: Vector, rot: Vector) -> Result<Mesh, String> {
        let mut f: File = File::open(filename).unwrap();
        let mut contents = String::new();

        f.read_to_string(&mut contents).unwrap();
        
        let mut lines: Vec<&str> = contents.split("\n").collect();
        let mut vertex_data: Vec<Vector> = Vec::new();
        let mut faces: Vec<([usize; 3], usize)> = Vec::new();
        let mut normals: Vec<Vector> = Vec::new();

        let mut mesh = Mesh {
            name: String::new(),
            vertices: Vec::new(),
            faces: Vec::new(),
            pos: pos,
            rot: rot,
        };

        println!("1");

        for i in 0..(lines.len()-1) { // Note that the last element of lines is an empty list.  
            let mut l: Vec<&str> = lines[i].split(" ").collect(); 

            match l[0] {
                "o" => {
                    mesh.name = String::from(l[1]);
                },

                "v" => {
                    let mut v = Vector::new();

                    v.x = l[1].parse::<f64>().unwrap();
                    v.y = l[2].parse::<f64>().unwrap();
                    v.z = l[3].parse::<f64>().unwrap();
                    v.w = 1.0;

                    vertex_data.push(v);
                },
                
                "vn" => {
                    let mut vn = Vector::new();

                    vn.x = l[1].parse::<f64>().unwrap();
                    vn.y = l[2].parse::<f64>().unwrap();
                    vn.z = l[3].parse::<f64>().unwrap();
                    vn.w = 0.0;

                    normals.push(vn);
                },

                "f" => {
                    let mut vertices = [0usize; 3];
                    let mut normal = 0usize;

                    for j in 1..l.len() {
                        let vf: Vec<&str> = l[j].split("//").collect();
                        if j == 1 {
                            // We're assuming the normal is the same for each vector
                            normal = vf[1].parse::<usize>().unwrap()-1;
                        }

                        vertices[j-1] = vf[0].parse::<usize>().unwrap()-1;
                    }

                    faces.push((vertices, normal));
                },

                _ => {}
            }
        }

        println!("2");

        let mut face_structs: Vec<Face> = Vec::new();

        for i in 0..faces.len() {
            let test = normals[faces[i].1].clone();

            face_structs.push(
                Face::from(faces[i].0[0], faces[i].0[1], faces[i].0[2], normals[faces[i].1].clone(), Colour::Grey(1.0)) // TODO Actually calculate colour
            );
        }

        mesh.vertices = vertex_data;
        mesh.faces = face_structs;

        Ok(mesh) 
    }
}

impl Face {
    fn from(v1: usize, v2: usize, v3: usize, normal: Vector, colour: Colour) -> Face {
        Face {
            vertices: [v1, v2, v3],
            colour: colour,
            normal: normal,
        }
    } 

    fn new() -> Face {
        Face {
            vertices: [0; 3],
            colour: Colour::Grey(1.0),
            normal: Vector::new(),
        }
    }
}
