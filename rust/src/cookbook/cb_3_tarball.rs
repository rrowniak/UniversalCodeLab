use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::str;
use std::{fs::File, path::PathBuf, process::Command};
use tar::Archive;

pub fn compress(path: &PathBuf) {
    if let Err(e) = compress_int(path) {
        println!("Compression error: {}", e);
    }
}

fn compress_int(path: &PathBuf) -> Result<(), std::io::Error> {
    println!("Starting...");
    println!("{}", cmd("ls", &["-al"]));
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("archive", path)?;
    // tar.append_dir(path);
    Ok(())
}

pub fn decompress(file: &PathBuf) {
    if let Err(e) = decompress_int(file) {
        println!("Decompression error: {}", e);
    }
}

fn decompress_int(file: &PathBuf) -> Result<(), std::io::Error> {
    let tar_gz = File::open(file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

fn cmd(cmd: &str, args: &[&str]) -> String {
    let out = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to run command");

    str::from_utf8(&out.stdout)
        .expect("Error std::from_utf8")
        .to_string()
}
