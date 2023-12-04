#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Digit,
    Period,
    Symbol,
}

#[derive(Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub count_tiles: usize,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub t: TileType,
    pub c: char,
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
            width: width as i32,
            height: height as i32,
            count_tiles,
            tiles: str::replace(input, "\n", "")
                .chars()
                .map(|c| match c {
                    '.' => Tile {
                        t: TileType::Period,
                        c: '.',
                    },
                    c if c.is_ascii_digit() => Tile {
                        t: TileType::Digit,
                        c,
                    },
                    _ => Tile {
                        t: TileType::Symbol,
                        c,
                    },
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn get_tile(&self, p: &Point) -> &Tile {
        let Point { x, y } = p;

        &self.tiles[self.map_index(*x, *y)]
    }

    pub fn map_index(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn map_point(&self, index: usize) -> Point {
        let index = index as i32;
        let x = index % self.width;
        let y = index / self.width;

        Point { x, y }
    }

    pub fn in_bounds(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }
}
