use std::fs;
use std::path::Path;

fn main() {
    let filename = "COMMAND_REFERENCE_GEN.md".to_string();
    let filepath = Path::new(&filename);
    match fs::remove_file(filepath.clone()) {
        Ok(_) => (),
        Err(e) => println!("Could not delete {:?}: {}", &filename, e) 
    }
}
