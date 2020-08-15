use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => { if let Ok(entries) = fs::read_dir(".") {
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
        },
        2 => {
        let filename = &args[1];
        match filename.parse() {
            if let Ok(entries) = fs::read_dir(filename) {
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
        }

    }


}
