#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Digit,
    Period,
    Symbol,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub count_tiles: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let count_tiles = (width * height) as usize;

        Self {
            width,
            height,
            count_tiles,
            tiles: vec![TileType::Period; count_tiles],
        }
    }

    pub fn map_index(&self, x: u32, y: u32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(self.map_index(point.x, point.y))
        }
    }
}
