use crate::errors::AppError;
use crate::game::components::{
    AiState, BasicAi, BlocksTile, Enemy, Health, Name, Player, Position, Renderable, Stats,
};
use crate::game::map::GameMap;
use crate::game::state::{ActionJournal, GameTurn};
use crate::game::systems;
use crate::input::handlers::GameAction;
use bevy_ecs::change_detection::DetectChangesMut;
use bevy_ecs::schedule::{IntoSystemConfigs, Schedule, SystemSet, apply_deferred};
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use crossterm::event::{self, Event};
use rand::{Rng, rng};
use ratatui::style::Color;

pub enum AppScreen {
    MainMenu,
    Options,
    Game,
}

#[derive(Resource, PartialEq, Debug)]
pub struct GameInputAction(pub Option<GameAction>);

pub struct App {
    pub screen: AppScreen,
    pub should_quit: bool,
    pub menu_index: usize,
    pub world: World,
    pub schedule: Schedule,
}

const ENEMIES_COUNT: usize = 10;
const MAP_WIDTH: usize = 150;
const MAP_HEIGHT: usize = 120;

pub static MENU_ITEMS: [&str; 4] = ["New Game", "Continue", "Options", "Quit"];

impl App {
    pub fn new() -> Self {
        let mut world = World::new();
        let map = GameMap::new(MAP_WIDTH, MAP_HEIGHT);

        world.insert_resource(map.clone()); // Clone since we need map to spawn enemies on start
        world.insert_resource(ActionJournal::default());
        world.insert_resource(GameTurn::default());
        world.insert_resource(GameInputAction(None));

        // Spawn entities
        world.spawn((
            Player,
            Name("Hero".to_string()),
            Position { x: 2, y: 2 },
            Renderable {
                symbol: 'u'.to_string(),
                fg: Color::Yellow,
                bg: Color::Reset,
            },
            Stats {
                health: Health::new(100),
                attack: 10,
                defense: 5,
            },
            // Add the new player parameter here
        ));

        let mut enemy_count = 0;
        let mut rand = rng();

        while enemy_count < ENEMIES_COUNT {
            let x_pos = rand.random_range(1..MAP_WIDTH);
            let y_pos = rand.random_range(1..MAP_HEIGHT);

            if !map.is_wall(x_pos, y_pos) {
                world.spawn((
                    Enemy, // Enemy tag
                    Name("Goblin".to_string()),
                    Position { x: x_pos, y: y_pos },
                    Renderable {
                        symbol: "g".to_string(),
                        fg: ratatui::style::Color::LightRed,
                        bg: ratatui::style::Color::Reset,
                    },
                    Stats {
                        health: Health::new(20),
                        attack: 5,
                        defense: 2,
                    },
                    BasicAi {
                        // AI Component
                        state: AiState::Idle,
                        target_visible: false,
                        last_known_player_pos: None,
                        fov_radius: 8,
                    },
                    BlocksTile, // Goblins block tiles
                ));
                enemy_count += 1;
            }
        }

        // Setup Schedule
        let mut schedule = Schedule::default();

        schedule.add_systems(
            (
                crate::game::systems::player_input_system.in_set(GameSystemSet::Input),
                crate::game::systems::ai_system.in_set(GameSystemSet::AI),
                // Apply deferred buffer allows systems to safely add/remove components/entities
                apply_deferred.in_set(GameSystemSet::ApplyCommands),
                systems::movement_system.in_set(GameSystemSet::Movement),
                systems::update_turn_system.in_set(GameSystemSet::TurnEnd), // Example system
            )
                .chain(),
        ); // Run systems sequentially for now

        Self {
            screen: AppScreen::MainMenu,
            should_quit: false,
            menu_index: 0,
            schedule,
            world,
        }
    }

    pub fn handle_events(&mut self) -> Result<bool, AppError> {
        let mut gia = self.world.resource_mut::<GameInputAction>();

        // Using hypothetical AppError
        if let Event::Key(key) = event::read().map_err(AppError::Io)? {
            match self.screen {
                AppScreen::MainMenu => {
                    if let Some(action) = crate::input::handlers::handle_menu_input(key) {
                        self.apply_menu_action(action); // New method needed
                    }
                }

                AppScreen::Game => {
                    gia.set_if_neq(GameInputAction(crate::input::handlers::handle_game_input(
                        key,
                    )));
                    if matches!(gia.0, Some(GameAction::OpenMenu)) {
                        self.screen = AppScreen::MainMenu;
                        gia.set_if_neq(GameInputAction(None)); // Consume Acton
                    } else if matches!(gia.0, Some(GameAction::Quit)) {
                        self.should_quit = true;
                        gia.set_if_neq(GameInputAction(None)); // Consume Acton
                    }
                    // Player input action now handled by the system
                }
                AppScreen::Options => { /* Handle options input or keep todo!() */ }
            }
        }
        Ok(self.should_quit)
    }

    // Run schedules only when player did action
    pub fn run_schedule(&mut self) {
        let game_input_action = self.world.resource_mut::<GameInputAction>();
        // Only run the schedule if there was a player action waiting
        if game_input_action.0.is_some() {
            self.schedule.run(&mut self.world);

            // Clear the action after the schedule runs
            let mut game_input_action = self.world.resource_mut::<GameInputAction>();
            game_input_action.set_if_neq(GameInputAction(None));
        }
    }

    fn apply_menu_action(&mut self, action: crate::input::handlers::MenuAction) {
        use crate::input::handlers::MenuAction;
        match action {
            MenuAction::NavigateUp => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                }
            }
            MenuAction::NavigateDown => {
                if self.menu_index < Vec::from(MENU_ITEMS).len() - 1 {
                    self.menu_index += 1;
                }
            }
            MenuAction::Select => match self.menu_index {
                0 | 1 => self.screen = AppScreen::Game, // New Game or Continue
                2 => {}                                 // Options
                3 => self.should_quit = true,           // Quit
                _ => {}
            },
            MenuAction::Quit => self.should_quit = true,
        }
    }
}

// Define System Sets for ordering
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    Input,
    AI,
    ApplyCommands, // To apply commands generated by Input/AI
    Movement,
    Combat, // Placeholder for future combat system
    TurnEnd,
}
