#[derive(Debug)]
pub struct Rectangle {
    pub width:u32,
    pub height:u32
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]

pub struct Circle{
    pub radius:f64
}

impl Circle{
    pub fn circumference(&self)->f64{
        2.0 * std::f64::consts::PI * self.radius
    }
}