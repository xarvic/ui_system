use std::vec::Vec;

use crate::basics::*;

mod builder;
pub use builder::Builder;
mod renderer;
pub use renderer::{Renderer, make_shader, make_shader_single_file};


use image::ImageFormat;
use std::rc::Rc;
use glium::backend::Context;
use std::fs::read;
use glium::texture::{texture2d::Texture2d};


pub fn load_texture(path: &str, format: ImageFormat, context: &Rc<Context>) -> Texture2d{
    println!("Load Texture:");
    let bytes = read(path).expect(&(String::from("Cant read imagefile: ")+path));
    println!("read bytes");
    let image = image::load(std::io::Cursor::new(bytes.as_slice()),
                            format).unwrap();
    println!("created image");
    let image = image.to_rgba();
    println!("format image");
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    println!("made texture");
    Texture2d::new(context, image).unwrap()
}

pub struct CommandBuffer{
    pub color_rects: Vec<ColorVertex>,
    pub glyphs: Vec<GlyphVertex>,
    pub line_elements: Vec<BorderVertex>,

}

impl CommandBuffer{
    pub fn new() -> CommandBuffer{
        CommandBuffer{color_rects: Vec::new(), glyphs: Vec::new(), line_elements: Vec::new()}
    }
}

#[derive(Copy, Clone)]
pub struct ColorVertex{
    position: [f32; 2],
    round_index: [f32; 2],
    color: [f32; 4],
    pixels: f32,
}
impl ColorVertex{
    pub fn from(position: Vector, round_index: Vector, color: Color, pixels: f32) -> ColorVertex{
        ColorVertex{position: [position.x, position.y], round_index: [round_index.x, round_index.y], color: [color.r, color.g, color.b, color.a], pixels}
    }
}

implement_vertex!(ColorVertex, position, round_index, color, pixels);

#[derive(Copy, Clone)]
pub struct GlyphVertex {
    position: [f32; 2],
    coord: [f32; 2],
}

impl GlyphVertex {
    fn from(position: Vector, coord: Vector) -> GlyphVertex {
        GlyphVertex {position: [position.x, position.y], coord: [coord.x, coord.y]}
    }
}

implement_vertex!(GlyphVertex, position, coord);

//Border

#[derive(Copy, Clone)]
pub struct BorderVertex{
    position: [f32; 2],
    color: [f32; 4],
    width: f32,
    radius: f32,
    control: [f32; 2],
    mode: i32,
}
implement_vertex!(BorderVertex, position, color, width, control, radius, mode);

impl BorderVertex {
    pub fn new(position: Vector, color: Color, width: f32, control: Vector, radius: f32, mode: i32) -> BorderVertex {
        BorderVertex{position: position.into(), color: color.into(), width, control: control.into(), radius, mode}
    }

}