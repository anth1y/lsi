use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let meta = fs::metadata(filename)?;
    let inode = meta.ino();
    println!("{:?}", inode);
    Ok(())
}
