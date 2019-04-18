// The device which renders the objects and draws them to the screen. They are all contained in
// this one device struct which is why I'm giving it its own file.

use crate::render_objects::*;
use crate::structures::*;
use std::cmp;

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
    
    pub fn clear_screen(&self) {
        print!("{}[2J", 27 as char);
    }

    fn move_cursor_to(&self, x: usize, y: usize) {
        print!("{}[{};{}H", 27 as char, (x+1).to_string(), (y+1).to_string());
    }

    pub fn draw_point(&self, x: usize, y: usize, colour: Colour) {
        self.move_cursor_to(x, y);
        
        print!("{}", Device::colour_to_char(colour));
    }

    pub fn draw_line_fast(&self, p1: (usize, usize), p2: (usize, usize), colour: Colour) {
        // Implementing Bresenham's Line Algorithm:
        // This is fast but provides no antialiasing.
        
        let pp1 = (p1.1, p1.0);
        let pp2 = (p2.1, p2.0);
        
        if (pp2.1 as i32 - pp1.1 as i32).abs() < (pp2.0 as i32 - pp1.0 as i32).abs() {
            if pp1.0 > pp2.0 {
                self.draw_line_low(pp2, pp1, colour.clone());
            }

            else {
                self.draw_line_low(pp1, pp2, colour.clone());
            }
        }

        else {
            if pp1.1 > pp2.1 {
                self.draw_line_high(pp2, pp1, colour.clone());
            }

            else {
                self.draw_line_high(pp1, pp2, colour.clone());
            }
        }
    }

    fn draw_line_low(&self, p1: (usize, usize), p2: (usize, usize), colour: Colour) {
        let mut dx = p2.0 as f64 - p1.0 as f64;
        let mut dy = p2.1 as f64 - p1.1 as f64;

        let mut yi = 1;

        if dy < 0.0 {
            yi = -1;
            dy *= -1.0;
        }

        let mut D = 2.0*dy - dx;
        let mut y = p1.1 as i32;

        for x in p1.0..p2.0 {
            self.draw_point(x, y as usize, colour.clone());

            if D > 0.0 {
                y += yi;
                D -= 2.0*dx;
            }

            D += 2.0*dy;
        }
    }

    fn draw_line_high(&self, p1: (usize, usize), p2: (usize, usize), colour: Colour) {
        let mut dx = p2.0 as f64 - p1.0 as f64;
        let mut dy = p2.1 as f64 - p1.1 as f64;

        let mut xi: i32 = 1;

        if dx < 0.0 {
            xi = -1;
            dx *= -1.0;
        }

        let mut D = 2.0*dx - dy;
        let mut x = p1.1 as i32;

        for y in p1.1..p2.1 {
            self.draw_point(x as usize, y, colour.clone());

            if D > 0.0 {
                x += xi;
                D -= 2.0*dy;
            }

            D += 2.0*dx;
        }
    }

    pub fn draw_line_antialiased(&self, p1: (f64, f64), p2: (f64, f64), colour: Colour) {
        // Implementing Wu's Line Algorithm:
        // This is slow but antialiased
        // The reason this takes f64 input and not usize like draw_line_fast is because this can
        // give you a line that is not necessarily drawn *from* one pixel to another. Instead since
        // it is antialiased it can give you something that better approximates a line from a float
        // value instead of just integers.
        
        // Funny story, I tried this normally once and for some reason it would swap the x and y
        // values for each point around. There's a problem in my code somewhere but my reasoning is
        // that if I simply swap them around beforehand it will all work out in the end.
        // Modern problems require modern solutions.
        let mut y0 = p1.0; let mut y1 = p2.0;
        let mut x0 = p1.1; let mut x1 = p2.1;
        
        let steep = (y1 - y0).abs() > (x1 - x0).abs();

        if steep {
            // Swap x0 and y0
            let mut temp = x0;
            x0 = y0;
            y0 = temp;

            // Swap x1 and y1
            temp = x1;
            x1 = y1;
            y1 = temp;
        }

        if x0 > x1 {
            // Swap x0 and x1
            let mut temp = x0;
            x0 = x1;
            x1 = temp;

            // Swap y0 and y1
            temp = y0;
            y0 = y1;
            y1 = temp;
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let mut gradient = 1f64;

        if dx != 0.0 {
            gradient = dy/dx;
        }

        // First endpoint. If you haven't realised by now I'm pretty much just reading this off
        // Wikipedia. Donate $3!
        let mut xend = x0.round();
        let mut yend = y0 + gradient * (xend-x0);

        let xgap = 1.0-((x0+0.5).fract());
        let mut xpxl1 = xend;
        let mut ypxl1 = yend.trunc();

        if steep {
            self.draw_point(ypxl1 as usize, xpxl1 as usize, Colour::Grey((1.0-(yend.fract())) * xgap));
            self.draw_point((ypxl1+1.0) as usize, xpxl1 as usize, Colour::Grey((yend.fract()) * xgap));
        }

        else {
            self.draw_point(xpxl1 as usize, ypxl1 as usize, Colour::Grey((1.0-(yend.fract())) * xgap));
            self.draw_point(xpxl1 as usize, (ypxl1+1.0) as usize, Colour::Grey((yend.fract()) * xgap));
        }

        let mut intery = yend + gradient;

        // Second endpoint. Same thing.
        let mut xend = x1.round();
        let mut yend = y1 + gradient * (xend-x1);

        let xgap = 1.0-((x1+0.5).fract());
        let mut xpxl2 = xend;
        let mut ypxl2 = yend.trunc();

        if steep {
            self.draw_point(ypxl2 as usize, xpxl2 as usize, Colour::Grey((1.0-(yend.fract())) * xgap));
            self.draw_point((ypxl2+1.0) as usize, xpxl2 as usize, Colour::Grey((yend.fract()) * xgap));
        }

        else {
            self.draw_point(xpxl2 as usize, ypxl2 as usize, Colour::Grey((1.0-(yend.fract())) * xgap));
            self.draw_point(xpxl2 as usize, (ypxl2+1.0) as usize, Colour::Grey((yend.fract()) * xgap));
        }

        if steep {
            for x in (xpxl1 as usize + 1)..(xpxl2 as usize) {
                self.draw_point(intery.trunc() as usize, x, Colour::Grey(1.0-intery.fract()));
                self.draw_point(intery.trunc() as usize + 1, x, Colour::Grey(intery.fract()));
                intery += gradient;
            }
        }

        else {
            for x in (xpxl1 as usize + 1)..(xpxl2 as usize) {
                self.draw_point(x, intery.trunc() as usize, Colour::Grey(1.0-intery.fract()));
                self.draw_point(x, intery.trunc() as usize + 1, Colour::Grey(intery.fract()));
                intery += gradient;
            }
        }
    }

    fn colour_to_char(colour: Colour) -> char {
        let alpha = match colour {
            Colour::Rgba(r,g,b,a) => a,
            Colour::Grey(a) => a,
        };

        // █#&+-
        if alpha <= 0.2 {
            return '-';
        }

        else if alpha <= 0.4 {
            return '+';
        }

        else if alpha <= 0.6 {
            return '&';
        }

        else if alpha <= 0.8 {
            return '#';
        }

        else {
            return '█';
        }
    }

    pub fn draw_triangle(&self, p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), colour: Colour, antialiased: bool) {
        // First, draw the lines between each vertex.
        if antialiased {
            self.draw_line_antialiased(p1, p2, colour.clone());
            self.draw_line_antialiased(p2, p3, colour.clone());
            self.draw_line_antialiased(p3, p1, colour.clone());
            self.fill_triangle((p1.1, p1.0), (p2.1, p2.0), (p3.1, p3.0), colour.clone());
        }

        else {
            // This doesn't quite work yet... Just don't use it?
            self.draw_line_fast((p1.0 as usize, p1.1 as usize), (p2.0 as usize, p2.1 as usize), colour.clone());
            self.draw_line_fast((p2.0 as usize, p2.1 as usize), (p3.0 as usize, p3.1 as usize), colour.clone());
            self.draw_line_fast((p3.0 as usize, p3.1 as usize), (p1.0 as usize, p1.1 as usize), colour.clone());
            self.fill_triangle((p1.1, p1.0), (p2.1, p2.0), (p3.1, p3.0), colour.clone());
        }
    }

    pub fn fill_triangle(&self, p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), colour: Colour) {
        let min = (f64::min(f64::min(p1.0, p2.0), p3.0), f64::min(f64::min(p1.1, p2.1), p3.1));
        let max = (f64::max(f64::max(p1.0, p2.0), p3.0), f64::max(f64::max(p1.1, p2.1), p3.1));

        let vs1 = (p2.0 - p1.0, p2.1 - p1.1);
        let vs2 = (p3.0 - p1.0, p3.1 - p1.1);

        for x in (min.0 as usize)..(max.0 as usize+1) {
            for y in (min.1 as usize)..(max.1 as usize+1) {
                let q = (x as f64 - p1.0, y as f64 - p1.1);

                let s = Device::cross_point(q, vs2) / Device::cross_point(vs1, vs2);
                let t = Device::cross_point(vs1, q) / Device::cross_point(vs1, vs2);

                if (s >= 0.0 && t >= 0.0) && (s+t <= 1.0) {
                    self.draw_point(x as usize, y as usize, colour.clone());
                }
            }
        }
    }

    fn cross_point(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        return (p1.0*p2.1)-(p1.1*p2.0);
    }

    pub fn render(&self) {
        // Wow, the main render function! Snazzy.
        // First, generate the MVP matricies: Model, View, Projection.
        // Model matrix: the matrix that describes the basic position, rotation and scaling of each
        // mesh.
        

    }
}
