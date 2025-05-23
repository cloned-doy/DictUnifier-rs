use fs_extra::dir;
use lazy_static::lazy_static;
use std::env;

mod utils;
#[allow(unused_imports)]
use utils::{from_archive, from_folder};

fn main() -> anyhow::Result<()> {
    lazy_static! {
        static ref DEST: String = String::from("./dist");
    }
    dir::create_all("./dist/output", true)?;
    let args: Vec<String> = env::args().collect();
    if let Some(file) = args.get(1) {
        // test file: "./__test__/stardict-oald-cn-2.4.2.tar.bz2"
        from_archive(file, &DEST)?;
        // if the folder has multiple dics, set from folders: true
        // let multi_dicts = true;
        // from_folder(file, &DEST, multi_dicts)?;
    }

    Ok(())
}
