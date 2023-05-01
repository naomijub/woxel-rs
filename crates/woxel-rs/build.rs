fn main() {
    println!("cargo:rerun-if-changed=assets/prototypes/materials.yaml");

    material_derive::include_voxel_materials!("assets/prototypes/materials.yaml");
}
