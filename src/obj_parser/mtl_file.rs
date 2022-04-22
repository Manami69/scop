use crate::env::Color;
use crate::render_gl::texture::Texture;
// http://paulbourke.net/dataformats/mtl/
pub struct Mtl {
	pub name: String,
	pub ambient_color: Color,
	pub diffuse_color: Color,
	pub specular_color: Color,
	pub illum : u32,
	pub shininess: f32,
	pub text_map: Option<Texture>,
}

impl Mtl {
	pub fn new_default() -> Self {
		Self {
			name: String::new(),
			ambient_color: Color(0., 0., 0.),
			diffuse_color: Color(0., 0., 0.),
			specular_color: Color(0., 0., 0.),
			illum: 0,
			shininess: 0.,
			text_map: None,
		}
	}
}