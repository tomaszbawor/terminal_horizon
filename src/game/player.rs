use super::{entities::EntityPosition, map::GameMap};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub position: EntityPosition,
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
            position: EntityPosition::new(10, 10),
            hp,
            max_hp: hp,
            attack,
            defense,
            level: 1,
            exp: 0,
        }
    }

    pub fn move_up(&mut self, map: &GameMap) -> bool {
        if self.position.y > 0 && !map.is_wall(self.position.x, self.position.y - 1) {
            self.position.y -= 1;
            true
        } else {
            false
        }
    }

    pub fn move_down(&mut self, map: &GameMap) -> bool {
        if self.position.y < map.height - 1 && !map.is_wall(self.position.x, self.position.y + 1) {
            self.position.y += 1;
            true
        } else {
            false
        }
    }

    pub fn move_left(&mut self, map: &GameMap) -> bool {
        if self.position.x > 0 && !map.is_wall(self.position.x - 1, self.position.y) {
            self.position.x -= 1;
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self, map: &GameMap) -> bool {
        if self.position.x < map.width - 1 && !map.is_wall(self.position.x + 1, self.position.y) {
            self.position.x += 1;
            true
        } else {
            false
        }
    }
}
