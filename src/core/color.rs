pub enum Transparency {
    Opaque,
    Translucent,

}

pub struct HSV{
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl HSV {
    pub const fn new(hue: f32, saturation: f32, value: f32) -> Self {
        HSV{hue, saturation, value}
    }
    pub const fn to_rgb() -> Color{
        Color::grey(0.0)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color{
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,

}

pub static RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub static GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub static BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
pub static YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
pub static ORANGE: Color = Color::new(1.0, 0.5, 0.0, 1.0);
pub static PUPAL: Color = Color::new(1.0, 0.0, 1.0, 1.0);
pub static WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub static BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);


impl Color {

    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color{red, green, blue, alpha}
    }
    pub const fn grey(value: f32) -> Color {
        Color{red: value, green: value, blue: value, alpha: 1.0}
    }
}

impl Into<[f32; 4]> for Color{
    fn into(self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}
