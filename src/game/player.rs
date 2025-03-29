use super::map::GameMap;

pub struct Player {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub hp: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub level: u32,
    pub exp: u32,
}

impl Player {
    pub fn new(name: &str, hp: u32, attack: u32, defense: u32) -> Self {
        Self {
            name: name.to_string(),
            x: 10,
            y: 10,
            hp,
            max_hp: hp,
            attack,
            defense,
            level: 1,
            exp: 0,
        }
    }

    pub fn move_up(&mut self, map: &GameMap) -> bool {
        if self.y > 0 && !map.is_wall(self.x, self.y - 1) {
            self.y -= 1;
            true
        } else {
            false
        }
    }

    pub fn move_down(&mut self, map: &GameMap) -> bool {
        if self.y < map.height - 1 && !map.is_wall(self.x, self.y + 1) {
            self.y += 1;
            true
        } else {
            false
        }
    }

    pub fn move_left(&mut self, map: &GameMap) -> bool {
        if self.x > 0 && !map.is_wall(self.x - 1, self.y) {
            self.x -= 1;
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self, map: &GameMap) -> bool {
        if self.x < map.width - 1 && !map.is_wall(self.x + 1, self.y) {
            self.x += 1;
            true
        } else {
            false
        }
    }
}
