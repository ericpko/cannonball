use glam::Vec2;




pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(0.2, 0.2),
            vel: Vec2::new(10.0, 15.0),
        }
    }
}
