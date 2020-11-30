use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub steps: i64,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            steps: 0,
        }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
