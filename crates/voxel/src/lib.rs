#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct Voxel(pub u8);

impl Voxel {
    pub const EMPTY_VOXEL: Self = Self(0);
}

impl Default for Voxel {
    fn default() -> Self {
        Self::EMPTY_VOXEL
    }
}

pub trait MaterialVoxel {
    fn as_mat_id(&self) -> u8;
}

impl MaterialVoxel for Voxel {
    fn as_mat_id(&self) -> u8 {
        self.0
    }
}
