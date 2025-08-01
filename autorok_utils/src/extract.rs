use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

/// Extracts a .tar.gz file into the given output directory
pub fn tar_gz(input_path: &str, output_dir: &str) -> std::io::Result<()> {
    let tar_gz = File::open(input_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    archive.unpack(Path::new(output_dir))?;
    Ok(())
}
