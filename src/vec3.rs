pub struct Vec3 {
    pub elements: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { elements: [0.0; 3] }
    }
}
