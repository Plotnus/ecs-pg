use crate::enemy_state::EnemyState;
use crate::player_state::PlayerState;
use crate::projectile_state::ProjectileState;
use crate::tile::Tile;

const MAX_NUM_PROJECTILES: usize = 4;
const MAX_NUM_ENEMIES: usize = 8;
const NUM_TILES_WIDE: usize = 64;
const NUM_TILES_HIGH: usize = 64;

pub struct GameState {
    pub player: PlayerState,
    // here we are following her posting where we only get four on screen
    // this is just so we can see how things progress as we shift to an Entity Component system
    pub projectiles: [Option<ProjectileState>; MAX_NUM_PROJECTILES],

    // we store all the level data in the tiles
    // Tiles can index into an array of sprites and an array of properties like `is_traversable`
    pub level_tiles: [[Tile; NUM_TILES_WIDE]; NUM_TILES_HIGH],

    // Maximum of 8 enemies on screen.
    pub enemies: [Option<EnemyState>; MAX_NUM_ENEMIES],
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: PlayerState::new(),
            projectiles: [None; MAX_NUM_PROJECTILES],
            level_tiles: [[Tile::new(); NUM_TILES_WIDE]; NUM_TILES_HIGH],
            enemies: [None; MAX_NUM_ENEMIES],
        }
    }
}
