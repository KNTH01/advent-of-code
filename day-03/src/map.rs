#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Digit,
    Period,
    Symbol,
}

#[derive(Debug)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub count_tiles: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let width = input
            .chars()
            .position(|c| c == '\n')
            .expect("input should have lines");
        let height = input.lines().collect::<Vec<_>>().len();
        let count_tiles = width * height;

        Self {
            width: width as u32,
            height: height as u32,
            count_tiles,
            tiles: input
                .chars()
                .map(|c| match c {
                    '.' => TileType::Period,
                    c if c.is_ascii_digit() => TileType::Digit,
                    _ => TileType::Symbol,
                })
                .collect::<Vec<_>>(),
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
