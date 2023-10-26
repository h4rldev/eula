use tracing::{info, error};
use std::env::current_dir;
use std::fs::{OpenOptions, canonicalize};
use std::io::{prelude::*, BufReader, Seek, SeekFrom, Result};
use std::io::BufWriter;
use std::io::Write;
use std::fs::metadata;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    if metadata("eula.txt").is_ok() {
        info!("eula.txt exists!");
        replace_line()
    } else {
        error!("eula.txt does not exist!");
        std::process::exit(1);
    }
}

fn replace_line() -> Result<()> {
    let path = current_dir()?.join("eula.txt");
    let path = canonicalize(path)?;
    let new_path = path.clone();
    info!("Path name = {:?}", path);
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let reader = BufReader::new(&file);

    let mut lines: Vec<String> = reader.lines().map(|l| l.expect("Can't get line")).collect();

    // Insert a line at the 3rd position
    info!("Current line: {}", lines[2]);
    info!("Replacing: {}", lines[2]);
    lines[2] = "eula=true".to_string();

    // Clear the file
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    // Write the lines to a temporary file
    let temp_path = new_path.with_extension("tmp");
    let temp_file = OpenOptions::new().write(true).create(true).open(&temp_path)?;
    let mut writer = BufWriter::new(&temp_file);
    for line in &lines {
        writeln!(writer, "{}", line)?;
    }
    writer.flush()?;

    // Replace the original file with the temporary file
    std::fs::remove_file(&new_path)?;
    std::fs::rename(&temp_path, &new_path)?;

    info!("Replaced successfully!, line is now \"{}\"!", lines[2]);
    Ok(())
}