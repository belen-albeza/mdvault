use mdvault::Vault;
use std::path::PathBuf;

fn main() {
    let vault = Vault::open(&PathBuf::from("tests/fixtures/basic")).unwrap();
    println!("Vault at {:?}", vault.base_path);
    println!("Notes ({}):", vault.notes.len());
    for note in vault.notes {
        println!("\t{:#?}", note.file.rel_path().unwrap())
    }
    println!("Assets ({}):", vault.assets.len());
    for asset in vault.assets {
        println!("\t{:?}", asset.file.rel_path().unwrap())
    }
}
