use std::fs::File;
use std::io::{prelude::*, BufReader};
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
const V: usize = 0;
const VT: usize = 1;
const VN: usize = 2;


// TODO: remplir par objet et non pas par fichier
pub struct Objfile {
    pub v: Vec<Vec<f32>>,
    pub vt: Vec<Vec<f32>>,
    pub vn: Vec<f32>,
    pub vp: Vec<f32>,
    pub f: Vec<Vec<usize>>,
    pub l: Vec<f32>,
}

impl Objfile {
    pub fn new() -> Self {
        Self {
            v: vec![],
            vt: vec![],
            vn: vec![],
            vp: vec![],
            f: vec![Vec::<usize>::new(); 3],
            l: vec![],
        }
    }
    pub fn read_file(&mut self, filename: &String) {
        match Path::new(&filename).extension() {
            None => {
                panic!("file must have obj extension")
            }
            Some(ext) => {
                assert_eq!(
                    "obj", ext,
                    "testing filename extension : must be obj found {:?}",
                    ext
                );
            }
        };
        let file = match File::open(filename) {
            Err(err) => {
                panic!("Couldn't open obj file : {}", err)
            }
            Ok(ok) => ok,
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.parse_line(&line.unwrap());
        }
        //println!("{:?}", self.v);
    }
    fn parse_line(&mut self, line: &String) {
        let mut split: Vec<&str> = line.split(' ').collect();
        split = split.iter().filter_map(|s| (!s.is_empty()).then(|| *s)).collect();
        if split.is_empty() {return}
        match split[0] {
            "v" => self.parse_v(split),
            "vt" => self.parse_vt(split),
            // "vn" => todo!(),
            // "vp" => todo!(),
            "f" => self.parse_f(split),
            // "l" => todo!(),
            _ => {}
        }
    }

    fn parse_v(&mut self, split: Vec<&str>) {
        let len = split.len();
        let mut vec: Vec<f32> = Vec::<f32>::new();
        if len != 4 && len != 5 {
            return;
        }
        split[1..len].into_iter().for_each(|val| {
            match val.parse::<f32>() {
                Err(_) => {}
                Ok(num) => {
                    vec.push(num);
                }
            };
        });
        if vec.len() != len - 1 {
            return;
        }
        if len == 4 {
            vec.push(1.0)
        }
        self.v.push(vec);
    }

    fn parse_vt(&mut self, split: Vec<&str>) {
        let len = split.len();
        let mut vec: Vec<f32> = Vec::<f32>::new();
        if len > 4 && len < 2 {
            return;
        }
        split[1..len].into_iter().for_each(|val| {
            match val.parse::<f32>() {
                Err(_) => {}
                Ok(num) => {
                    vec.push(num);
                }
            };
        });
        if vec.len() != len - 1 {
            return;
        }
        while vec.len() < 3 {
            vec.push(0.)
        }
        self.vt.push(vec);
    }

    fn parse_f(&mut self, split: Vec<&str>) {
        let len = split.len();
        let mut vec = vec![Vec::<usize>::new(); 3];
        if len < 4 {
            return;
        }
        split[1..len].into_iter().for_each(|point| {
            let v_type : Vec<&str>= point.split('/').collect();
            for (i, val ) in v_type.iter().enumerate() {
                match val.parse::<usize>() {
                Err(_) => {}
                Ok(num) => {
                    vec[i].push(num);
                }
            };
        }
        });
        let veclen = vec[V].len();
        if veclen != len - 1 {
            return;
        }
        if veclen > 3 {
            // let mut collec: Vec<Vec<usize>> = Vec::new();
            for j in 0..veclen - 2 {
                for (i, val) in vec.iter().enumerate() {
                    if val.len() != 0{
                        self.f[i].push(val[0]);
                        self.f[i].push(val[j + 1]);
                        self.f[i].push(val[j + 2]);
                    }
                }
            //     collec.push(vec![vec[0], vec[i + 1], vec[i + 2]])
            }
            // collec.iter().for_each(|val| self.f.push(val.clone()));
        } else {
            for (i, val) in vec.iter().enumerate() {
                val.iter().for_each(|v| self.f[i].push(*v))
            }        }
    }

    pub fn get_v(&self) -> Vec<f32> {
        let mut arr: Vec<f32> = Vec::new();
        // println!("{:?}", self.f);
        if self.f[V].len() == 0 { return arr }
        self.f[V].iter().for_each(|index| {
                self.v[*index - 1].iter().for_each(|vertice| {
                    arr.push(*vertice);
                });
            });
        arr
    }
}
