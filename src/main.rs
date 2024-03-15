use std::{fs, path::PathBuf};

use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    delete_files(path.into());
}

fn delete_files(path: PathBuf) {
    if path.is_file() {
        fs::remove_file(path).unwrap();
    } else {
        let folder = path.clone();
        fs::read_dir(path).unwrap()
            .par_bridge()
            .for_each(|entry| {
                let entry = entry.unwrap().path();
                delete_files(entry);
            });
        fs::remove_dir(folder).unwrap();
    }
}