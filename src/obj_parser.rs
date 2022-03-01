use std::fs::File;
use std::io::{ prelude::*, BufReader};
use std::path::Path;
/*
# List of geometric vertices, with (x, y, z [,w]) coordinates, w is optional and defaults to 1.0.
v 0.123 0.234 0.345 1.0
v ...
...
# List of texture coordinates, in (u, [,v ,w]) coordinates, these will vary between 0 and 1. v, w are optional and default to 0.
vt 0.500 1 [0]
vt ...
...
# List of vertex normals in (x,y,z) form; normals might not be unit vectors.
vn 0.707 0.000 0.707
vn ...
...
# Parameter space vertices in ( u [,v] [,w] ) form; free form geometry statement ( see below )
vp 0.310000 3.210000 2.100000
vp ...
...
# Polygonal face element (see below)
f 1 2 3
f 3/1 4/2 5/3
f 6/4/1 3/5/3 7/6/5
f 7//1 8//2 9//3
f ...
...
# Line element (see below)
l 5 8 1 2 4 9
*/
// https://forums.raywenderlich.com/t/face-normal-vs-vertex-normal/100928
// https://stackoverflow.com/questions/23723993/converting-quadriladerals-in-an-obj-file-into-triangles

pub struct Objf {
    pub v : Option<Vec<f32>>,
    pub vt : Option<Vec<f32>>,
    pub vn : Option<Vec<f32>>,
    pub vp : Option<Vec<f32>>,
    pub f : Option<Vec<f32>>,
    pub l : Option<Vec<f32>>,
}



impl Objf {

    pub fn new() -> Self {
        Self{ v: None, vt: None, vn: None, vp: None, f:None, l:None}
    }
    pub fn read_file(&mut self, filename: &String) {
        match Path::new(&filename).extension() {
            None => {panic!("file must have obj extension")},
            Some(ext) => {assert_eq!("obj", ext, "testing filename extension : must be obj found {:?}",ext );},
        };
        let file = match File::open(filename) {
            Err(err) => { panic!("Couldn't open obj file : {}", err)},
            Ok(ok) => ok,
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.parse_line(&line.unwrap());
        }
        println!("{:?}", self.v);
    }
    fn parse_line(&mut self, line: &String) {
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "v" => self.parse_v(split),
            "vt" => self.parse_vt(split),
            // "vn" => todo!(),
            // "vp" => todo!(),
            // "f" => todo!(),
            // "l" => todo!(),
            _ => {},
        }
    }

    fn parse_v(&mut self, split: Vec<&str>) {
        let len = split.len();
        let mut vec : Vec<f32> = Vec::<f32>::new();
        if len != 4 && len != 5 { return; }
        split[1 .. len].into_iter().for_each(|val|  {
            match val.parse::<f32>() {
                Err(_) => {},
                Ok(num) => {vec.push(num);},
            };
        });
        if vec.len() != len - 1 { return; }
        if len == 4 { vec.push(1.0)}
        self.v = match self.v.clone() {
            None => Some(vec),
            Some(mut v) => {
                for num in vec {
                    v.push(num);
                }
                Some(v)
            }, 

        }
    }

    fn parse_vt(&mut self, split: Vec<&str>) {
        let len = split.len();
        let mut vec : Vec<f32> = Vec::<f32>::new();
        if len > 4 && len < 2 { return; }
        split[1 .. len].into_iter().for_each(|val|  {
            match val.parse::<f32>() {
                Err(_) => {},
                Ok(num) => {vec.push(num);},
            };
        });
        if vec.len() != len - 1 { return; }
        while vec.len() < 3 { vec.push(0.)}
        self.vt = match self.vt.clone() {
            None => Some(vec),
            Some(mut v) => {
                for num in vec {
                    v.push(num);
                }
                Some(v)
            }, 

        }
    }

    pub fn get_v(&self) -> Option<Vec<f32>>{
        self.v.clone()
    }
}