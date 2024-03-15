use std::{fs, path::PathBuf};

use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    delete_files(path.into());
}

fn delete_files(path: PathBuf) {
    if path.is_file() {
        let metadata = fs::metadata(path.clone()).unwrap();
        let mut perms = metadata.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
            fs::set_permissions(path.clone(), perms).unwrap();
        }
        match fs::remove_file(path.clone()) {
            Ok(_) => {},
            Err(err) => panic!("{}: {}", path.display(), err.kind()),
        }
    } else {
        let folder = path.clone();
        fs::read_dir(path).unwrap()
            .par_bridge()
            .for_each(|entry| {
                delete_files(entry.unwrap().path());
            });

        let metadata = fs::metadata(folder.clone()).unwrap();
        let mut perms = metadata.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
            fs::set_permissions(folder.clone(), perms).unwrap();
        }
        fs::remove_dir(folder).unwrap();
    }
}