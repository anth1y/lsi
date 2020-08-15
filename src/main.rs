use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

fn main() -> io::Result<()> {
    if let Ok(entries) = fs::read_dir(".") {
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
