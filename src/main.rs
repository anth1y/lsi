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
    // TODO: make fname mut and replace it with &args[1]
    let fname = ".";
    get_inodes(fname.to_string());
}
