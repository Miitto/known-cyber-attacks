use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let ignore_dirs = ["target", ".git", "src", "Template", ".vscode"];

    // Read all directories and files in the current directory
    let paths = std::fs::read_dir(".").unwrap().filter_map(|path_r| {
        let path = if let Ok(path) = path_r {
            path
        } else {
            return None;
        };

        if !path.file_type().unwrap().is_dir() {
            return None;
        }

        if ignore_dirs
            .iter()
            .any(|dir| path.path().starts_with(format!("./{dir}")))
        {
            return None;
        }

        // Get the path
        let path = path.path().join("README.md");

        // Check if the path exists
        if !path.exists() {
            return None;
        }

        Some(path)
    });
    let compile_path = PathBuf::from("./Compile.md");

    dbg!(&compile_path);

    let compile_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(compile_path)?;
    let mut compile_writer = BufWriter::new(compile_file);

    // Iterate over the paths
    paths.for_each(|path| {
        dbg!(&path);

        let file = if let Ok(file) = File::open(&path) {
            file
        } else {
            return;
        };

        let mut reader = std::io::BufReader::new(file);

        // Copy the file to the compile file
        std::io::copy(&mut reader, &mut compile_writer).unwrap();

        // Add a new line to separate the files
        writeln!(compile_writer, "\n\n").unwrap();
    });

    Ok(())
}
