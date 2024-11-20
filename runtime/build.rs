use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use cargo_metadata::MetadataCommand;

fn main() {
    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .exec()
        .expect("Failed to read metadata");

    let root = metadata.root_package().unwrap();
    let component_world_path = root.metadata.get("component_world").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bindgen_macro.rs");

    let mut f = File::create(&dest_path).unwrap();
    f.write_all(
        format!(
            r#"
bindgen!({{
    path: {},
    additional_derives: [
        serde::Deserialize,
        serde::Serialize,
    ],
}});"#,
            component_world_path
        )
        .as_bytes(),
    )
    .unwrap();
}
