#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TerrainTile {
    #[default]
    Ocean,
    Grass,
}

impl TerrainTile {
    pub fn sprite_index(&self) -> usize {
        match self {
            TerrainTile::Ocean => 10,
            TerrainTile::Grass => 0,
        }
    }
}
