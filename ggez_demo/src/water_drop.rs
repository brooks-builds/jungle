use ggez::{
    graphics::{self, DrawParam, Mesh},
    nalgebra::Point2,
    Context, GameResult,
};

pub struct WaterDrop {
    location: Point2<f32>,
    velocity_y: f32,
    acceleration_y: f32,
}

impl WaterDrop {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            location: Point2::new(x, y),
            velocity_y: 0.0,
            acceleration_y: 1.0,
        }
    }

    pub fn update(&mut self) {
        self.velocity_y += self.acceleration_y;
        self.location.y += self.velocity_y;
    }

    pub fn draw(&self, context: &mut Context, mesh: &Mesh) -> GameResult {
        graphics::draw(context, mesh, DrawParam::new().dest(self.location))
    }

    pub fn is_alive(&self, screen_height: f32) -> bool {
        self.location.y < screen_height
    }
}
