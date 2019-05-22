#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TileVariant {
    Wall = 0,
    Grass = 1,
    /* ... etc ... */
}
#[derive(Copy, Clone)]
pub struct Tile {
    variant: TileVariant,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            variant: TileVariant::Grass,
        }
    }
}
