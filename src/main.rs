use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

fn get_inodes(fname: String) -> io::Result<()> {
    if let Ok(entries) = fs::read_dir(fname) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    println!("{} {}", entry.path().display(), metadata.ino());
                } else {
                    println!("Couldn't get metadata for {}", entry.path().display());
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let fname = ".";
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            get_inodes(fname.to_string());
        }
        2 => {
            let fname = &args[1];
            get_inodes(fname.to_string());
        }
        _ => (),
    }
}
