#[derive(Clone, Copy)]
pub struct Interval<T>
where
    T: std::cmp::PartialOrd,
{
    pub lb: T,
    pub ub: T,
}

impl<T> Interval<T>
where
    T: std::cmp::PartialOrd + Copy,
{
    pub fn new(lb: T, ub: T) -> Self {
        Self { lb, ub }
    }

    pub fn clamp(&self, val: T) -> T {
        num::clamp(val, self.lb, self.ub)
    }

    pub fn contains(&self, val: T) -> bool {
        val >= self.lb && val <= self.ub
    }

    pub fn surrounds(&self, val: T) -> bool {
        val > self.lb && val < self.ub
    }
}
