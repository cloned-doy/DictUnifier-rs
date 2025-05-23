use anyhow::Result;
use fs_extra::dir;
use std::env;

mod utils;

use utils::{from_archive, from_folder};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} [tar|folder|folders] <path> [dest]", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let input_path = &args[2];
    let dest = args.get(3).map_or("./dist".to_string(), |d| d.clone());

    dir::create_all(format!("{}/output", dest), true)?;

    match mode.as_str() {
        "tar" => {
            from_archive(input_path, &dest)?;
        }
        "folder" => {
            from_folder(input_path, &dest, false)?;
        }
        "folders" => {
            from_folder(input_path, &dest, true)?;
        }
        _ => {
            eprintln!("Unknown mode: {}. Use 'tar', 'folder', or 'folders'.", mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
