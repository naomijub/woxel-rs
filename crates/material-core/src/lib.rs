use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use voxel::Voxel;

#[macro_export]
macro_rules! voxel_material {
    ($types: ident, $id: expr) => {
        pub struct $types;
        impl $types {
            pub const NAME: &'static str = stringify!($types);
        }
        impl $crate::VoxelMaterial for $types {
            const ID: u8 = $id;
        }
    };
}

// Registry info about a voxel material
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct MaterialVisualInfo {
    pub name: String,
    pub base_color: [u8; 3],
    pub flags: u8, //VoxelMaterialFlags,
    pub emissive: [u8; 3],
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
}

/// Helper / marker trait for voxel materials.
pub trait VoxelMaterial {
    const ID: u8;

    fn into_voxel() -> Voxel {
        Voxel(Self::ID)
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    pub struct VoxelMaterialFlags : u32 {
        const SOLID = 0;
        const LIQUID = 1 << 1;
        const UNBREAKABLE = 1 << 2;
        const VOID = 1 << 3;
        const ALGAE = 1 << 4;
    }
}

impl Default for VoxelMaterialFlags {
    fn default() -> Self {
        Self::SOLID
    }
}

impl VoxelMaterialFlags {
    pub fn to_flag(value: u32) -> Self {
        match value {
            0 => Self::SOLID,
            1 => Self::LIQUID,
            2 => Self::UNBREAKABLE,
            3 => Self::VOID,
            4 => Self::ALGAE,
            _ => unreachable!(),
        }
    }
}
