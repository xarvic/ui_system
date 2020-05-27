use crate::renderer::{ColorVertex, CommandBuffer, GlyphVertex, BorderVertex};
use core::color::Color;
use core::position::Vector;

pub struct Builder<'a>{
    position: Vector,
    buffer_written: bool,

    buffer: &'a mut CommandBuffer,
}

impl<'a> Builder<'a>{
    pub fn create_with(buffer: &mut CommandBuffer) -> Builder{
        Builder{position: Vector{x: 0.0, y: 0.0}, buffer_written: false, buffer}
    }
    #[inline]
    pub fn draw_color_triangle(&mut self, start: Vector, v_end: Vector, color: Color){
        self.buffer.color_rects.push(ColorVertex::from(self.position + start, Vector::new(-1.0, -1.0), color, 0.0));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, v_end.y), Vector::new(-1.0, -1.0), color, 0.0));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(v_end.x, start.y), Vector::new(-1.0, -1.0), color, 0.0));
    }
    #[inline]
    pub fn draw_round_color_triangle(&mut self, start: Vector, v_end: Vector, color: Color, mut round: f32){
        round = round.min((start.x-v_end.x).abs().min((start.y-v_end.y).abs()));
        self.buffer.color_rects.push(ColorVertex::from(self.position + start, Vector::new(0.0, 0.0), color, round));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, v_end.y), Vector::new(0.0, -((start.y-v_end.y).abs())), color, round));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(v_end.x, start.y), Vector::new(-((start.x-v_end.x).abs()), 0.0), color, round));
    }
    pub fn draw_color_rect(&mut self, start: Vector, end: Vector, color: Color){
        self.draw_round_color_triangle(start, end, color, 5.0);
        self.draw_round_color_triangle(end, start, color, 5.0);
    }
    pub fn draw_round_rect(&mut self, start: Vector, end: Vector, color: Color, round: [f32; 4]){
        match round {
            [0.0, 0.0, 0.0, 0.0] => self.draw_color_rect(start, end, color),
            _ => {
                let mid = (start + end) * 0.5;
                self.draw_round_color_triangle(start, mid, color, round[0]);
                self.draw_round_color_triangle(end, mid, color, round[1]);
                self.draw_round_color_triangle(Vector::new(start.x, end.y), mid, color, round[2]);
                self.draw_round_color_triangle(Vector::new(end.x, start.y), mid, color, round[3]);

                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(mid.x, start.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(end.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(end.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(mid.x, end.y), Vector::new(-1.0, -1.0), color, 0.0));
            }
        }
    }
    pub fn draw_glyph(&mut self, start: Vector, end: Vector, glyph: u32){
        let x = glyph as f32 % 13.0;
        let y = (glyph / 13) as f32;
        self.buffer.glyphs.push(GlyphVertex::from(self.position + start, Vector{x: x / 13.0, y: 1.0 - y / 12.0}));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: start.x, y: end.y}, Vector{x: x / 13.0, y: 1.0 - (y + 1.0) / 12.0}));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: end.x, y: start.y}, Vector{x: (x + 1.0) / 13.0, y: 1.0 - y / 12.0}));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: start.x, y: end.y}, Vector{x: x / 13.0, y: 1.0 - (y + 1.0) / 12.0}));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: end.x, y: start.y}, Vector{x: (x + 1.0) / 13.0, y: 1.0 - y / 12.0}));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + end, Vector{x: (x + 1.0) / 13.0, y: 1.0 - (y + 1.0) / 12.0}));
    }

    //Lines

    pub fn to_point(&mut self, position: Vector, color: Color, width: f32, restart: bool) {
        let mut option = 0;
        if restart {
            option = 0x2;
        }

        self.buffer.line_elements.push(BorderVertex::new(position + self.position, color, width, Vector::null(), 0.0, option/*No segment before*/));
    }
    pub fn arc_to(&mut self, position: Vector, color: Color, width: f32, control: Vector, radius: f32) {
        self.buffer.line_elements.push(BorderVertex::new(position + self.position, color, width, control + self.position, radius, 1/*arc*/));
    }


    //Builder Options

    pub fn child_builder(&mut self, relative_position: Vector) -> Builder{
        Builder{position: self.position + relative_position, buffer_written: self.buffer_written, buffer: self.buffer}
    }
}