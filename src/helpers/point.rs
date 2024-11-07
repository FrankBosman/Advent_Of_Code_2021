#[derive(Hash, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn clone(&self) -> Point {
        Point {x: self.x, y: self.y}
    }
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point { Point { x, y } }
    pub fn from(text: &str) -> Point {
        let parsed = text.split(",").map(|x| x.trim().parse::<i16>().unwrap()).collect::<Vec<i16>>();
        Point { x: parsed[0], y: parsed[1] }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
