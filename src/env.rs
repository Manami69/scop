use std::collections::HashMap;

/// Window's height
pub const HEIGHT: u32 = 720;
/// Window's width
pub const WIDTH: u32 = 1280;
/// Window's name
pub const NAME: &str = "scop";
/// textures max
pub const TEXT_MAX: i32 = 3;
#[derive(Clone, Copy, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32);
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const SKYBOX_VERTICES: [f32; 108] = [
    // positions
    -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
    -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0,
    -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
    1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
    -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
    -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
    -1.0, 1.0, 1.0, -1.0, 1.0,
];

pub struct ModelEvent {
    pub text_i: i32,
    pub next_text: f32,
    pub trans: Point3d,
    pub cam_z: f32,
    pub turn: Point3d,
    pub poly: bool,
    pub keys: HashMap<String, bool>,
}

impl ModelEvent {
    pub fn new() -> Self {
        Self {
            text_i: 0,
            next_text: 0.0,
            trans: Point3d {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            cam_z: 0.0,
            turn: Point3d {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            poly: false,
            keys: HashMap::new(),
        }
    }

    //pub
}

#[derive(Debug)]
pub struct ScopOption {
    pub concave: bool,
    pub coplana: bool,
    pub skybox: bool,
    pub text: Option<String>,
}
impl ScopOption {
    pub fn new() -> Self {
        Self {
            concave: false,
            coplana: false,
            skybox: false,
            text: None,
        }
    }

    pub fn fill_options(&mut self, args: Vec<String>) {
        let mut next = false;
        for (i, arg) in args.into_iter().enumerate() {
            match arg.as_str() {
                "/concave" => {
                    if next {
                        panic!("Options not well formated")
                    }
                    self.concave = true;
                }
                "/coplanar" => {
                    if next {
                        panic!("Options not well formated")
                    }
                    self.coplana = true;
                }
                "/sky" => {
                    if next {
                        panic!("Options not well formated")
                    }
                    self.skybox = true;
                }
                "/t" => next = true,
                _ => {
                    if next {
                        self.text = Some(arg.to_string());
                        next = false;
                    }
                }
            }
        }
    }
}
