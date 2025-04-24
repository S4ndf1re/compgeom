pub type Position = [f32; 3];
pub type Color = [f32; 3];

#[repr(C, packed)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
}
pub struct Polygon {
    pub verticies: Vec<Vertex>,
}

impl Polygon {
    pub fn new(verticies: Vec<Vertex>) -> Self {
        Self { verticies }
    }

    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let mut bounds = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);

        for v in &self.verticies {
            bounds = (
                bounds.0.min(v.position[0]),
                bounds.1.max(v.position[0]),
                bounds.2.min(v.position[1]),
                bounds.3.max(v.position[1]),
            );
        }

        bounds
    }

    /// Shift polygon so that x_0, y_0 are set to x, y
    pub fn shift_to(&mut self, x: f32, y: f32) {
        let bounds = self.get_bounds();

        let x_diff = x - bounds.0;
        let y_diff = y - bounds.2;

        for v in &mut self.verticies {
            v.position[0] = v.position[0] + x_diff;
            v.position[1] = v.position[1] + y_diff;
        }
    }

    pub fn set_color(&mut self, color: Color) {
        for v in &mut self.verticies {
            v.color = color;
        }
    }
}
