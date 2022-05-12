use super::mtl_file::Mtl;
use crate::env::*;
use crate::env::{Color, Point3d};
use crate::mathlib::{classes::{matrix::Matrix, vector::Vector}, operations::other::lerp};
use crate::render_gl::texture::Texture;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};
use std::path::Path;
//use itertools::izip; //     for (x, y, z) in izip!(&a, &b, &c) {
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
// https://web.cse.ohio-state.edu/~shen.94/581/Site/Lab3_files/Labhelp_Obj_parser.htm
const V: usize = 0;
const VT: usize = 1;
const VN: usize = 2;

// TODO: remplir par objet/groupe et non pas par fichier
pub struct Objfile {
    pub v: Vec<Vec<f32>>,
    pub vt: Vec<Vec<f32>>,
    pub vn: Vec<Vec<f32>>,
    // pub vp: Vec<f32>,
    pub f: HashMap<String, Vec<Vec<usize>>>,
    pub l: Vec<f32>,
    pub tex: Vec<Mtl>,
    pub mtl_names: Vec<String>,
    pub using: String,
    pub min: Point3d,
    pub max: Point3d,
    pub mid: Point3d,
	pub textures: HashMap<String, Texture>,
	pub gl: gl::Gl,
}

impl Objfile {
    pub fn new(gl: &gl::Gl) -> Self {
        Self {
            v: vec![],
            vt: vec![],
            vn: vec![],
            // vp: vec![],
            f: HashMap::new(),
            l: vec![],
            tex: vec![],
            mtl_names: vec![],
            using: String::new(),
            min: Point3d {
                x: f32::MAX,
                y: f32::MAX,
                z: f32::MAX,
            },
            max: Point3d {
                x: f32::MIN,
                y: f32::MIN,
                z: f32::MIN,
            },
            mid: Point3d {
                x: 0.,
                y: 0.,
                z: 0.,
            },
			textures : HashMap::new(),
			gl: gl.clone(), 
        }
    }
    pub fn read_file(&mut self, filename: &String, opt: &ScopOption) {
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
            self.parse_line(&line.unwrap(), opt);
        }
        //println!("{:?}", self.v);
    }
    fn parse_line(&mut self, line: &String, opt: &ScopOption) {
        let mut split: Vec<&str> = line.split(' ').collect();
        //let mut comm: bool = false;
        split = split
            .iter()
            .filter_map(|s| (!s.is_empty()).then(|| *s))
            .collect();
        if split.is_empty() {
            return;
        }
        split = split
            .into_iter()
            .take_while(|s| match s.find('#') {
                Some(_) => false,
                None => true,
            })
            .collect();
        if split.len() > 0 {
            match split[0] {
                "mtllib" => self.parse_mtllib(split), // parse le fichier et rempli la hashmap de mtl
                "v" => self.parse_v(split),
                "vt" => self.parse_vt(split),
                "vn" => self.parse_vn(split),
                // "vp" => todo!(),
                "usemtl" => self.parse_usemtl(split), // met le bon material dans current si pas trouve alors default value // TODO:
                "f" => self.parse_f(split, opt),
                // "l" => todo!(),
                _ => {}
            }
        }
    }
    fn parse_usemtl(&mut self, split: Vec<&str>) {
        if split.len() < 2 {
            panic!("HOHOHO JE SUIS LE PERE NOEL")
        }
        if self.mtl_names.clone().contains(&split[1].to_string()) {
            self.using = split[1].to_string();
        }
    }
    fn parse_mtllib_line(&mut self, line: &String) {
        let mut split: Vec<&str> = line.split(' ').collect();
        //let mut comm: bool = false;
        split = split
            .iter()
            .filter_map(|s| (!s.is_empty()).then(|| *s))
            .collect();
        if split.is_empty() {
            return;
        }
        split = split
            .into_iter()
            .take_while(|s| match s.find('#') {
                Some(_) => false,
                None => true,
            })
            .collect();
        if split.len() == 0 {
            return;
        }
        match split[0] {
            "newmtl" => {
                if split.len() < 2 {
                    panic!("MTL FILE CORRUPTED");
                }
                self.mtl_names.push(split[1].to_string());
                self.tex.push(Mtl::new(split[1]));
            }
            "Kd" => {
                if split.len() < 4 {
                    panic!("MTL FILE CORRUPTED");
                }
                let l = self.mtl_names.len();
                if l == 0 {
                    panic!("MTL FILE CORRUPTED")
                }

                self.tex[l - 1].diffuse_color = Color(
                    split[1].parse::<f32>().unwrap(),
                    split[2].parse::<f32>().unwrap(),
                    split[3].parse::<f32>().unwrap(),
                );
            }
            "map_Kd" => {
                if split.len() < 2 {
                    panic!("MTL FILE CORRUPTED")
                }
                let l = self.mtl_names.len();
                if l == 0 {
                    panic!("MTL FILE CORRUPTED")
                }
                self.tex[l - 1].text_map = Some(split[1].to_string());
				if !self.textures.contains_key(split[1]) {
					eprintln!("loading texture mtl {}", split[1]);
					self.textures
						.insert(split[1].to_string(), Texture::new(&self.gl));
					self.textures.get_mut(split[1]).unwrap().load(format!("Ressources/{}", split[1]));
				}
            }
            _ => {}
        };
    }

    fn parse_mtllib(&mut self, split: Vec<&str>) {
        if split.len() < 2 {
            eprintln!("CANNOT FIND MTL FILE");
            return;
        }
        let filename = split[1];
        match Path::new(&filename).extension() {
            None => {
                eprintln!("MTL FILE CORRUPTED");
            }
            Some(ext) => {
                assert_eq!(
                    "mtl", ext,
                    "testing filename extension : must be mtl found {:?}",
                    ext
                );
            }
        };
        let file = match File::open(format!("{}{}", "Ressources/", filename)) {
            Err(err) => {
                eprintln!("Couldn't open mtl file : {}", err);
                return;
            }
            Ok(ok) => ok,
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.parse_mtllib_line(&line.unwrap())
        }
    }

    fn parse_v(&mut self, split: Vec<&str>) {
        let len = split.len();
        let mut vec: Vec<f32> = Vec::<f32>::new();
        if len != 4 && len != 5 {
            panic!("unvalid obj file")
        }
        for (i, val) in split[1..len].into_iter().enumerate() {
            match val.parse::<f32>() {
                Err(_) => {}
                Ok(num) => {
                    if i == 0 {
                        if num < self.min.x {
                            self.min.x = num
                        }
                        if num > self.max.x {
                            self.max.x = num
                        }
                    } else if i == 1 {
                        if num < self.min.y {
                            self.min.y = num
                        }
                        if num > self.max.y {
                            self.max.y = num
                        }
                    } else if i == 2 {
                        if num < self.min.z {
                            self.min.z = num
                        }
                        if num > self.max.z {
                            self.max.z = num
                        }
                    }
                    vec.push(num);
                }
            };
        }
        if vec.len() != len - 1 {
            panic!("unvalid obj file")
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
            panic!("unvalid obj file")
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
            panic!("unvalid obj file")
        }
        while vec.len() < 3 {
            vec.push(0.)
        }
        self.vt.push(vec);
    }

    fn parse_vn(&mut self, split: Vec<&str>) {
        let len = split.len();
        let mut vec: Vec<f32> = Vec::<f32>::new();
        if len != 4 {
            panic!("unvalid obj file")
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
            panic!("unvalid obj file");
        }
        self.vn.push(vec);
    }
	// seulement pour les plan > 3m
    fn check_coplanar(&mut self, vec: Vec<usize>) {
    	let mut test :  Vec<f32> = vec![];
		let mut minus: [f32; 3] = [0.; 3];
		let len = vec.len() - 1;
    	for (i, val) in vec.clone().into_iter().enumerate() {
			if i == 0 {
				minus[0] = self.v[val - 1][0];
				minus[1] = self.v[val - 1][1];
				minus[2] = self.v[val - 1][2];
			} else {
				test.push(self.v[val - 1][0] - minus[0]);
				test.push(self.v[val - 1][1] - minus[1]);
				test.push(self.v[val - 1][2] - minus[2]);
			}
		}
    	let mat = Matrix::from((test, [len, 3])).transpose();
		if mat.rank() > 2 {
			if len == 3 && mat.determinant() > 0.00001 { panic!("Non coplanar face");}
			else {
				for i in 0 .. len - 2 {
					if mat.get_mat3_from_offset(i).determinant() > 0.00001 { panic!("Non coplanar face");}
				}
			}
		}
    }

    fn parse_f(&mut self, split: Vec<&str>, opt: &ScopOption) {
        if !self.f.contains_key(&self.using) {
			eprintln!("creating mtl {}", &self.using);
            self.f
                .insert(self.using.clone(), vec![Vec::<usize>::new(); 3]);
        }

        let len = split.len();
        let mut vec = vec![Vec::<usize>::new(); 3];
        if len < 4 {
            return;
        }
        split[1..len].into_iter().for_each(|point| {
            let v_type: Vec<&str> = point.split('/').collect();
            for (i, val) in v_type.iter().enumerate() {
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
        // TODO: mettre les checks
        if veclen > 3 {
			if opt.coplana == true {self.check_coplanar(vec[V].clone());}
            // let mut collec: Vec<Vec<usize>> = Vec::new();
            for j in 0..veclen - 2 {
                for (i, val) in vec.iter().enumerate() {
                    if val.len() != 0 {
                        self.f.get_mut(&self.using).unwrap()[i].push(val[0]);
                        self.f.get_mut(&self.using).unwrap()[i].push(val[j + 1]);
                        self.f.get_mut(&self.using).unwrap()[i].push(val[j + 2]);
                    }
                }
                //     collec.push(vec![vec[0], vec[i + 1], vec[i + 2]])
            }
            // collec.iter().for_each(|val| self.f.push(val.clone()));
        } else {
            for (i, val) in vec.iter().enumerate() {
                val.iter()
                    .for_each(|v| self.f.get_mut(&self.using).unwrap()[i].push(*v))
            }
        }
    }

    pub fn get_v(&mut self) -> Vec<f32> {
        let mut arr: Vec<f32> = Vec::new();
        // println!("{:?}", self.f);
        if self.v.len() == 0 {
            return arr;
        }
        let mut cface = 0;

        let mut rcolor = Color(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        if self.mtl_names.is_empty() {
            self.mtl_names.push(String::new());
            self.tex.push(Mtl::new_default());
            //eprintln!("POPO");
        }
        for (index, name) in self.mtl_names.clone().into_iter().enumerate() {
			if self.f.get(&name).is_none() {continue; }
			self.tex[index].show = true;
			self.tex[index].start = arr.len() as i32;
            //eprintln!("POUET {:?}", self.f);
            let color = self.tex[index].diffuse_color;
            for (i, face) in self.f.get(&name).expect(&format!("mtl {} not found in mtl list (had {:?})", &name, &self.mtl_names))[V].iter().enumerate() {
                if cface % 3 == 0 {
                    cface = 0;
                    rcolor = Color(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
                }
                cface += 1;
                // push vertices
                self.v[*face - 1].iter().for_each(|vertice| {
                    arr.push(*vertice);
                });
                // random color
                arr.push(rcolor.0);
                arr.push(rcolor.1);
                arr.push(rcolor.2);
                // true color
                arr.push(color.0);
                arr.push(color.1);
                arr.push(color.2);
                // text coord
                if !self.vt.is_empty() && i < self.f.get(&name).unwrap()[VT].len() {
                    arr.push(self.vt[self.f.get(&name).unwrap()[VT][i] - 1][0]);
                    arr.push(self.vt[self.f.get(&name).unwrap()[VT][i] - 1][1]);
                } else {
					arr.push(0.);
                    arr.push(0.);
                }
                // normal coord
				if !self.vn.is_empty() && i < self.f.get(&name).unwrap()[VN].len() {
					arr.push(self.vn[self.f.get(&name).unwrap()[VN][i] - 1][0]);
					arr.push(self.vn[self.f.get(&name).unwrap()[VN][i] - 1][1]);
					arr.push(self.vn[self.f.get(&name).unwrap()[VN][i] - 1][2]);
				} else {
					// cross product bb
					let v = Vector::from(self.v[*face - 1 - cface].clone());
					let u = Vector::from(self.v[*face - cface].clone());
					let norm = Vector::cross_product(&u, &v).normalize();
					arr.push(norm.data[0]);
					arr.push(norm.data[1]);
					arr.push(norm.data[2]);
				}
            }
			self.tex[index].end = arr.len() as i32;
        }
        self.mid.x = lerp(self.min.x, self.max.x, 0.5);
        self.mid.y = lerp(self.min.y, self.max.y, 0.5);
        self.mid.z = lerp(self.min.z, self.max.z, 0.5);
        arr
    }
}
