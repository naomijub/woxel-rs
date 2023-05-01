use litrs::StringLit;
use material_core::MaterialVisualInfo;
use proc_macro::{self, TokenStream, TokenTree};
use proc_macro2::{
    Ident as Ident2, Literal as Literal2, Span as Span2, TokenStream as TokenStream2,
};
use quote::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MaterialList {
    materials: Vec<MaterialVisualInfo>,
}

#[proc_macro]
pub fn include_voxel_materials(input: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = input.into_iter().collect();
    let string_lit =
        StringLit::try_from(&input[0]).expect("Failed to read path. Path must be a string literal");
    let materials_path = string_lit.value();

    let yaml = fs::read_to_string(materials_path).unwrap();
    eprintln!("\n\n{materials_path:?}\n\n");

    let list = serde_yaml::from_str::<MaterialList>(&yaml).expect("Couldn't parse materials.yaml");

    let tokens =
        list.materials
            .iter()
            .enumerate()
            .fold(TokenStream2::new(), |mut acc, (idx, material)| {
                let name = Ident2::new(&material.name, Span2::call_site());
                let id = Literal2::u8_suffixed(idx as u8 + 1);
                let exp = generate_voxel_material(name, id);
                acc.extend(exp);
                acc
            });

    generate_materials_code(materials_path, tokens.clone());

    quote! { #tokens }.into()
}

fn generate_voxel_material(name: Ident2, id: Literal2) -> TokenStream2 {
    let exp = quote!(
      pub struct #name;
      
      #[allow(dead_code)]
      impl #name {
          pub const NAME: &'static str = stringify!(#name);
      }

      impl material_core::VoxelMaterial for #name {
          const ID: u8 = #id;
      }
    );
    exp
}

fn generate_materials_code(materials_path: &str, tokens: TokenStream2) {
    let mut buf = String::from("// == THIS FILE IS AUTO-GENERATED, PLEASE DON'T EDIT == \n\n");
    let out_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let package_path = materials_path.replace(".yaml", ".rs");
    let package = package_path.split('/').last().unwrap_or("materials.rs");

    let ast: syn::File = syn::parse2(tokens).expect("not a valid tokenstream");
    let code = prettyplease::unparse(&ast);
    buf.push_str(&code);

    let out_file = out_dir.join(format!("src/{}", package));
    eprintln!("\nWrite to file {out_file:?}");
    fs::write(out_file, buf).unwrap();
}
