use std::{env, path::PathBuf};

fn main() {
    let mut target_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let lib_name = env::var("CARGO_PKG_NAME").unwrap();

    let dll_file_name = format!("{}.dll", lib_name);

    target_dir.push(dll_file_name);

    let dll_path = target_dir.to_str().unwrap();

    let games = dirs_next::data_local_dir().unwrap();

    let games_path = games.join("KrajcEngine/installed_games.txt");

    std::fs::create_dir_all(games_path.parent().unwrap()).unwrap();
    let games = std::fs::read_to_string(&games_path).unwrap_or_default();

    if !games.split('\n').any(|p| p == dll_path) {
        let new_paths = if games.is_empty() {
            dll_path.to_string()
        } else {
            format!("{}\n{}", games, dll_path)
        };
        std::fs::write(games_path, new_paths).unwrap();
    }
    println!("cargo:rerun-if-changed={}", dll_path);
}
