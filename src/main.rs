use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let ignore_dirs = ["target", ".git", "src", "Template", ".vscode"];
    let compile_path = PathBuf::from("./Compile.md");

    dbg!(&compile_path);

    let compile_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(compile_path)?;
    let mut compile_writer = BufWriter::new(compile_file);

    // Read all directories and files in the current directory
    std::fs::read_dir(".")
        .unwrap()
        .filter_map(|path_r| {
            path_r
                .ok()
                .filter(|path| {
                    path.file_type().unwrap().is_dir()
                        && ignore_dirs
                            .iter()
                            .any(|dir| path.path().starts_with(format!("./{dir}")))
                })
                .map(|path| path.path().join("README.md"))
                .filter(|path| path.exists())
        })
        .for_each(|path| {
            if let Ok(file) = File::open(&path) {
                let mut reader = std::io::BufReader::new(file);

                // Copy the file to the compile file
                std::io::copy(&mut reader, &mut compile_writer).unwrap();

                // Add a new line to separate the files
                writeln!(compile_writer, "\n\n").unwrap();
            };
        });

    Ok(())
}
