#[derive(Clone, Copy)]
pub struct Interval {
    pub lb: f32,
    pub ub: f32,
}

impl Interval {
    pub fn new(lb: f32, ub: f32) -> Self {
        Self { lb, ub }
    }

    pub fn contains(&self, val: f32) -> bool {
        val >= self.lb && val <= self.ub
    }

    pub fn surrounds(&self, val: f32) -> bool {
        val > self.lb && val < self.ub
    }
}
