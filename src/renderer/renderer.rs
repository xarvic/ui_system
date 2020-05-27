use glium::backend::{Facade, Context};
use glium::{Program, DrawParameters, Blend, Frame, Surface, ProgramCreationError};
use std::fs::{read_to_string, File};
use std::rc::Rc;
use crate::renderer::{load_texture, CommandBuffer, Builder};
use image::ImageFormat;
use glium::texture::{texture2d::Texture2d};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::mem::replace;
use std::fmt::{Formatter, Debug};
use glium::index::PrimitiveType;
use crate::component::component::Component;

pub fn make_shader(path: &str, facade: &dyn Facade) -> Program{
    let vertex_shader = read_to_string(String::from(path) + ".vs").expect(&format!("cant read {}.vs", path));
    let fragment_shader = read_to_string(String::from(path) + ".fs").expect(&format!("cant read {}.fs", path));
    let geometry_shader = read_to_string(String::from(path) + ".gs").unwrap_or(String::from(""));

    println!("With geometry Shader!");

    let geometry_shader= if geometry_shader.is_empty(){
        None
    } else {
        Some(geometry_shader.as_str())
    };

    Program::from_source(facade, vertex_shader.as_str(), fragment_shader.as_str(), geometry_shader).expect("Cant create Programm!")
}

#[derive(Copy, Clone)]
enum ShaderType {
    Vertex,
    Geometry,
    Framgent,
}

pub enum ShaderError {
    CompilationError(ProgramCreationError),
    ReadingError(std::io::Error),
    MultipleSourceError(String),
    MissingSource(String),
}

impl Debug for ShaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            ShaderError::CompilationError(error) => {Debug::fmt(error, f)},
            ShaderError::ReadingError(error) => {Debug::fmt(error, f)},
            ShaderError::MultipleSourceError(error) => {writeln!(f, "{}", error)},
            ShaderError::MissingSource(error) => {writeln!(f, "{}", error)},
        }
    }
}

impl From<ProgramCreationError> for ShaderError {
    fn from(error: ProgramCreationError) -> Self {
        ShaderError::CompilationError(error)
    }
}

impl From<std::io::Error> for ShaderError {
    fn from(error: std::io::Error) -> Self {
        ShaderError::ReadingError(error)
    }
}

pub fn make_shader_single_file(path: &str, facade: &dyn Facade) -> Result<Program, ShaderError> {
    let reader = BufReader::new(File::open(Path::new(path))?);

    let mut vertex_shader: Option<String> = None;
    let mut geometry_shader: Option<String> = None;
    let mut fragment_shader: Option<String> = None;

    let mut current_type: Option<ShaderType> = None;
    let mut current_source = String::new();

    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.eq("#vertex") || line.eq("#fragment") || line.eq("#geometry") {
            if let Some(type_) = current_type {
                match type_ {
                    ShaderType::Vertex => {
                        if let Some(_) = vertex_shader {
                            return Err(ShaderError::MultipleSourceError(format!("Multiple Vertex Sources for Shader {}", path)));
                        } else {
                            vertex_shader = Some(replace(&mut current_source, String::new()));
                        }
                    }
                    ShaderType::Geometry => {
                        if let Some(_) = geometry_shader {
                            return Err(ShaderError::MultipleSourceError(format!("Multiple Geometry Sources for Shader {}", path)));
                        } else {
                            geometry_shader = Some(replace(&mut current_source, String::new()));
                        }
                    }
                    ShaderType::Framgent => {
                        if let Some(_) = fragment_shader {
                            return Err(ShaderError::MultipleSourceError(format!("Multiple Fragment Sources for Shader {}", path)));
                        } else {
                            fragment_shader = Some(replace(&mut current_source, String::new()));
                        }
                    }
                }
            } else {
                current_source = String::new();
            }

            current_type = match line {
                "#vertex" => {
                    Some(ShaderType::Vertex)
                }
                "#geometry" => {
                    Some(ShaderType::Geometry)
                }
                "#fragment" => {
                    Some(ShaderType::Framgent)
                }
                _ => {
                    unreachable!()
                }
            };
        } else {
            current_source += "\n";
            current_source += line;
        }
    }

    if let Some(type_) = current_type {
        print!("set old:\n");
        match type_{
            ShaderType::Vertex => {
                if let Some(_) = vertex_shader {
                    return Err(ShaderError::MultipleSourceError(format!("Multiple Vertex Sources for Shader {}", path)));
                } else {
                    vertex_shader = Some(replace(&mut current_source, String::new()));
                }
            }
            ShaderType::Geometry => {
                if let Some(_) = geometry_shader {
                    return Err(ShaderError::MultipleSourceError(format!("Multiple Geometry Sources for Shader {}", path)));
                } else {
                    geometry_shader = Some(replace(&mut current_source, String::new()));
                }
            }
            ShaderType::Framgent => {
                if let Some(_) = fragment_shader {
                    return Err(ShaderError::MultipleSourceError(format!("Multiple Fragment Sources for Shader {}", path)));
                } else {
                    fragment_shader = Some(replace(&mut current_source, String::new()));
                }
            }
        }
    }

    let geometry_shader = match geometry_shader {
        Some(ref source) => Some(source.as_str()),
        None => None,
    };

    let vertex_shader = match vertex_shader {
        Some(vs) => vs,
        None => return Err(ShaderError::MissingSource(format!("No Vertex Source for Shader {}", path))),
    };

    let fragment_shader = match fragment_shader {
        Some(fs) => fs,
        None => return Err(ShaderError::MissingSource(format!("No Fragment Source for Shader {}", path))),
    };


    Ok(Program::from_source(facade,
                         vertex_shader.as_str(),
                         fragment_shader.as_str(),
                         geometry_shader)?)
}

pub struct Renderer{
    color_rect_program: Program,
    glyph_program: Program,
    line_program: Program,
    font_buffer: Texture2d,
    context: Rc<Context>,
}

impl Renderer{
    pub fn new(context: &impl Facade) -> Renderer{
        let context = context.get_context();
        Renderer::from(make_shader("shaders/rounded", context),
                       make_shader("shaders/glyph", context),
                       make_shader_single_file("shaders/border.glsl", context).unwrap(),
                       load_texture("data/font.jpeg", ImageFormat::from_path("data/font.jpeg").unwrap_or(ImageFormat::Png), context),
                       context
        )
    }
    pub fn from(color_rect_program: Program, glyph_program: Program, line_program: Program, glyph_texture: Texture2d, context: &Rc<Context>) -> Renderer{
        Renderer{
            color_rect_program,
            glyph_program,
            line_program,
            font_buffer: glyph_texture,            context: context.clone()}
    }
    pub fn render(&mut self, buffer: &mut CommandBuffer, frame: &mut Frame){

        //------------------------------------------General--------------------------------------------
        let mut draw_params = DrawParameters::default();
        draw_params.blend = Blend::alpha_blending();
        draw_params.multisampling = false;
        draw_params.dithering = false;

        // building the index buffer
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let size = frame.get_dimensions();

        let data: [f32; 2] = [size.0 as f32, size.1 as f32];

        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        //-----------------------------------------ColorRects------------------------------------------
        {
            let verticies =
                glium::VertexBuffer::new(&self.context, buffer.color_rects.as_mut()).unwrap();

            let uniforms = uniform! {
                display: data,
            };
            frame.draw(&verticies, &index_buffer, &self.color_rect_program, &uniforms, &draw_params).unwrap();
        }

        //-----------------------------------------Glyphs----------------------------------------------
        {
            let verticies = glium::VertexBuffer::new(&self.context, buffer.glyphs.as_mut()).unwrap();

            let uniforms = uniform! {
                display: data,
                tex: &self.font_buffer,
            };
            frame.draw(&verticies, &index_buffer, &self.glyph_program, &uniforms, &draw_params).expect("Cant render on surface!");
        }

        //----------------------------------------Lines------------------------------------------------
        {
            let verticies = glium::VertexBuffer::new(&self.context, buffer.line_elements.as_mut()).unwrap();

            let uniforms = uniform! {
                display: data,
            };
            let index_buffer = glium::index::NoIndices(PrimitiveType::LineStrip);
            frame.draw(&verticies, &index_buffer, &self.line_program, &uniforms, &draw_params).expect("Cant render on surface!");
        }
    }
    pub fn render_screen(&mut self, component: &mut dyn Component, mut frame: Frame){
        let mut buffer = CommandBuffer::new();
        component.build(Builder::create_with(&mut buffer));

        println!("Build Frame({})", (buffer.color_rects.len() + buffer.glyphs.len()) / 6);

        self.render(&mut buffer, &mut frame);
        frame.finish().unwrap();
    }
    pub fn render_buffer(&mut self, buffer: &mut CommandBuffer, mut frame: Frame){
        println!("Build Buffer with {} elements", (buffer.color_rects.len() + buffer.glyphs.len()) / 6);

        self.render(buffer, &mut frame);
        frame.finish().unwrap();
    }
}