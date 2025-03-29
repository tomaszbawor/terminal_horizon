use rand::{Rng, thread_rng};

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Water,
}

pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let mut tiles = vec![vec![Tile::Floor; width]; height];

        // Add some random walls
        for y in 0..height {
            for x in 0..width {
                if rng.gen_ratio(1, 10) && !(x == 10 && y == 10) {
                    // Don't place a wall at player's starting position
                    tiles[y][x] = Tile::Wall;
                }
            }
        }
        // Add borders
        for x in 0..width {
            tiles[0][x] = Tile::Wall;
            tiles[height - 1][x] = Tile::Wall;
        }

        for y in 0..height {
            tiles[y][0] = Tile::Wall;
            tiles[y][width - 1] = Tile::Wall;
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return true;
        }
        matches!(self.tiles[y][x], Tile::Wall)
    }

    pub fn get_tile_symbol(&self, x: usize, y: usize) -> &str {
        match self.tiles[y][x] {
            Tile::Floor => ".",
            Tile::Wall => "#",
            Tile::Door => "+",
            Tile::Water => "~",
        }
    }
}
