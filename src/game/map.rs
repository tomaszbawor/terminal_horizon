use rand::{Rng, rng};

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Water,
}

#[derive(Clone)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rng();
        let mut tiles = vec![vec![Tile::Floor; width]; height];

        // Add some random walls
        for tile_row in tiles.iter_mut() {
            for tile in tile_row.iter_mut() {
                if rng.random_ratio(1, 10) {
                    *tile = Tile::Wall;
                }
            }
        }
        // Add borders
        for x in 0..width {
            tiles[0][x] = Tile::Wall;
            tiles[height - 1][x] = Tile::Wall;
        }

        for item in tiles.iter_mut().take(height) {
            item[0] = Tile::Wall;
            item[width - 1] = Tile::Wall;
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
