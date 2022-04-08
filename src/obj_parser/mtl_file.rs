use crate::env::Color;
// http://paulbourke.net/dataformats/mtl/
pub struct mtl {
	name: String;
	ambient_color: Color,
	diffuse_color: Color,
	specular_color: Color,
	illum : u32,
	shininess: f32,

}