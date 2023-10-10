use std::path::PathBuf;

use directories::UserDirs;

pub fn get_file_path(function_name: &str) -> PathBuf {
    let cwd: PathBuf =
        UserDirs::new().map_or(PathBuf::new(), |user_dirs| user_dirs.home_dir().to_owned());

    let file_name = format!("{}.wasm", function_name);

    cwd.join("projects/wasm_modules/").join(file_name)
}
