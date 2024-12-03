#[derive(Hash, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point { Point { x, y } }
    pub fn from(text: &str) -> Point {
        let parsed = text.split(",").map(|x| x.trim().parse::<i16>().unwrap()).collect::<Vec<i16>>();
        Point { x: parsed[0], y: parsed[1] }
    }
    pub fn from_index(index: usize, size: &(usize, usize)) -> Self {
        let x = (index % size.0) as i16;
        let y = (index / size.0) as i16;
        Self::new(x, y)
    }

    fn offset(&self, x_offset: i16, y_offset: i16) -> Self {
        Self::new(self.x + x_offset, self.y + y_offset)
    }

    /** <b>! Only works if the bound is from 0 -> max !</b> */
    fn in_bounds(&self, size: &(usize, usize)) -> bool {
        self.x >= 0 && self.x < size.0 as i16 && self.y >= 0 && self.y < size.1 as i16
    }

    /// <b>! Only works if the bound is from 0 -> max !</b><br>
    /// Retrieves the neighbours
    pub fn get_neighbours(&self, size: &(usize, usize), diagonals: bool) -> Vec<Point> {
        let mut neighbours = Vec::new();

        if diagonals {
            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    let new_point = self.offset(x_offset, y_offset);
                    if self.eq(&new_point) { continue; }
                    if new_point.in_bounds(size) {
                        neighbours.push(new_point);
                    }
                }
            }
        } else {
            for offset in [[-1, 0], [1, 0], [0, -1], [0, 1]].iter() {
                let new_point = self.offset(offset[0], offset[1]);
                if new_point.in_bounds(size) { neighbours.push(new_point); }
            }
        }
        neighbours
    }

    pub fn index(&self, size: &(usize, usize)) -> usize {
        self.x as usize + self.y as usize * size.0
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
