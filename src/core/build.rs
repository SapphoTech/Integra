use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let controllers_dir = Path::new(&crate_dir).join("src").join("app").join("controllers");
    let entries = fs::read_dir(&controllers_dir).unwrap();

    let mut content = String::new();
    
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "rs" {
            let module_name = path.file_stem().unwrap().to_str().unwrap();
            if module_name != "mod" {  // Avoid recursive include
                content.push_str(&format!("pub mod {};\n", module_name));
            }
        }
    }

    fs::write(controllers_dir.join("mod.rs"), content).unwrap();
}
