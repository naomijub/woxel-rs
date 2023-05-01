#[cfg(test)]
mod tests {
    macro_rules! include_proto {
        ($package: tt) => {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                concat!("/src/", $package, ".rs")
            ));
        };
    }

    pub mod api {
        include_proto!("materials");
    }
    #[test]
    fn it_works() {
        use material_core::VoxelMaterial;
        let a = api::Dirt::NAME;
        let id = api::Wood::ID;
        assert_eq!(a, "Dirt");
        assert_eq!(id, 10);
    }
}
