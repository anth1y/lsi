use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

fn readpath() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        let filename = env::current_dir();
        let argpath = fs::read_dir(filename);
    } else {
        let filename = &args[1];
        let argpath = fs::read_dir(filename);
    }
}

fn main() -> io::Result<()> {
    if let Ok(entries) = readpath() {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    println!("{} {}", entry.path().display(), metadata.ino());
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }
    }
    Ok(())
}
