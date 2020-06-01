use crate::renderer::{ColorVertex, CommandBuffer, GlyphVertex, BorderVertex};
use crate::core::{Color, Vector};
use crate::renderer::style::BorderPart;

pub struct Builder<'a>{
    position: Vector,
    buffer: &'a mut CommandBuffer,
}

impl<'a> Builder<'a>{
    pub fn create_with(buffer: &mut CommandBuffer) -> Builder{
        Builder{position: Vector{x: 0.0, y: 0.0}, buffer}
    }
    #[inline]
    fn triangle(&mut self, start: Vector, v_end: Vector, color: Color){
        self.buffer.color_rects.push(ColorVertex::from(self.position + start, Vector::new(-1.0, -1.0), color, 0.0));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, v_end.y), Vector::new(-1.0, -1.0), color, 0.0));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(v_end.x, start.y), Vector::new(-1.0, -1.0), color, 0.0));
    }
    #[inline]
    fn round_triangle(&mut self, start: Vector, v_end: Vector, color: Color, mut round: f32){
        round = round.min((start.x-v_end.x).abs().min((start.y-v_end.y).abs()));
        self.buffer.color_rects.push(ColorVertex::from(self.position + start, Vector::new(0.0, 0.0), color, round));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, v_end.y), Vector::new(0.0, -((start.y-v_end.y).abs())), color, round));
        self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(v_end.x, start.y), Vector::new(-((start.x-v_end.x).abs()), 0.0), color, round));
    }
    pub fn rect(&mut self, start: Vector, end: Vector, color: Color){
        self.triangle(start, end, color);
        self.triangle(end, start, color);
    }
    pub fn round_rect(&mut self, start: Vector, end: Vector, color: Color, round: [f32; 4]){
        match round {
            [0.0, 0.0, 0.0, 0.0] => self.rect(start, end, color),
            _ => {
                let mid = (start + end) * 0.5;
                self.round_triangle(start, mid, color, round[0]);
                self.round_triangle(end, mid, color, round[1]);
                self.round_triangle(Vector::new(start.x, end.y), mid, color, round[2]);
                self.round_triangle(Vector::new(end.x, start.y), mid, color, round[3]);

                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(mid.x, start.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(end.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(start.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(end.x, mid.y), Vector::new(-1.0, -1.0), color, 0.0));
                self.buffer.color_rects.push(ColorVertex::from(self.position + Vector::new(mid.x, end.y), Vector::new(-1.0, -1.0), color, 0.0));
            }
        }
    }
    pub fn glyph(&mut self, start: Vector, end: Vector, glyph: u32, color: Color){
        let x = glyph as f32 % 13.0;
        let y = (glyph / 13) as f32;
        self.buffer.glyphs.push(GlyphVertex::from(self.position + start, Vector{x: x / 13.0, y: 1.0 - y / 7.0}, color));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: start.x, y: end.y}, Vector{x: x / 13.0, y: 1.0 - (y + 1.0) / 7.0}, color));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: end.x, y: start.y}, Vector{x: (x + 1.0) / 13.0, y: 1.0 - y / 7.0}, color));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: start.x, y: end.y}, Vector{x: x / 13.0, y: 1.0 - (y + 1.0) / 7.0}, color));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + Vector{x: end.x, y: start.y}, Vector{x: (x + 1.0) / 13.0, y: 1.0 - y / 7.0}, color));
        self.buffer.glyphs.push(GlyphVertex::from(self.position + end, Vector{x: (x + 1.0) / 13.0, y: 1.0 - (y + 1.0) / 7.0}, color));
    }

    //Lines
    #[inline(always)]
    pub fn border_corner(&mut self, position: Vector, orientation: Vector, color: Color, width: f32, rounded: f32){
        //Clockwise
        if rounded == 0.0 {
            self.to_point(position, color, width, false);
            return;
        }
        if orientation.x == orientation.y {
            //Left Top or right bottom => y -> x
            self.to_point(position.y(rounded * orientation.y), color, width, false);
            self.arc_to(position.x(rounded * orientation.x), color, width, position, rounded);
        } else {
            //Left bottom or right top => x -> y
            self.to_point(position.x(rounded * orientation.x), color, width, false);
            self.arc_to(position.y(rounded * orientation.y), color, width, position, rounded);
        }
    }

    //clockwise starting at right top so part one is the top border
    pub fn border(&mut self, position: Vector, size: Vector, border_parts: &[BorderPart; 4], corners: [f32; 4]) {
        //Left Top
        self.to_point(Vector::new(corners[3], 0.0), border_parts[0].color, border_parts[0].width, true);
        //Right Top
        self.border_corner(position.x(size.x), Vector::new(-1.0, 1.0), border_parts[0].color, border_parts[0].width, corners[0]);
        //Right Bottom
        self.border_corner(position + size, Vector::new(-1.0, -1.0), border_parts[1].color, border_parts[1].width, corners[1]);
        //Left Bottom
        self.border_corner(position.y(size.y), Vector::new(1.0, -1.0), border_parts[2].color, border_parts[2].width, corners[2]);
        //Left Top
        self.border_corner(position, Vector::new(1.0, 1.0), border_parts[3].color, border_parts[3].width, corners[3]);
    }

    pub fn simple_border(&mut self, position: Vector, size: Vector, color: Color, width: f32, rounded: f32){
        //Left Top
        self.to_point(Vector::new(rounded, 0.0), color, width, true);
        //Right Top
        self.border_corner(position.x(size.x), Vector::new(-1.0, 1.0), color, width, rounded);
        //Right Bottom
        self.border_corner(position + size, Vector::new(-1.0, -1.0), color, width, rounded);
        //Left Bottom
        self.border_corner(position.y(size.y), Vector::new(1.0, -1.0), color, width, rounded);
        //Left Top
        self.border_corner(position, Vector::new(1.0, 1.0), color, width, rounded);

    }

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
        Builder{position: self.position + relative_position, buffer: self.buffer}
    }
    pub fn using(&'a mut self) -> Builder<'a> {
        Builder{
            position: self.position,
            buffer: self.buffer
        }
    }
}