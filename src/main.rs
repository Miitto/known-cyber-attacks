use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufWriter, Seek, Write};
use std::path::PathBuf;

struct Stride {
    text: String,
    prefixes: Vec<String>
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let search_dict = vec![
        Stride{
            text: "Spoofing".into(),
            prefixes: vec!["s"].into_iter().map(|e| e.into()).collect()
        }
    ];

    let search_terms: Vec<String> = search_dict.iter().filter(| Stride { prefixes, .. } |
    args.iter().any(|arg| {
        let contains = prefixes.contains(&arg);
            dbg!(prefixes);
            dbg!(arg);
            contains
        }
    )).map(|e| e.text.clone()).collect();

    dbg!(&search_terms);

    let ignore_dirs = ["target", ".git", "src", "Template", ".vscode"];
    let compile_path = PathBuf::from("./Compile.md");

    dbg!(&compile_path);

    let compile_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(compile_path)
        .unwrap();
    let mut compile_writer = BufWriter::new(compile_file);

    // Read all directories and files in the current directory
    std::fs::read_dir(".")
        .unwrap()
        .filter_map(|path_r| {
            path_r
                .ok()
                .filter(|path| {
                    path.file_type().unwrap().is_dir()
                        && !ignore_dirs
                            .iter()
                            .any(|dir| path.path().starts_with(format!("./{dir}")))
                })
                .map(|path| path.path().join("README.md"))
                .filter(|path| path.exists())
                .and_then(|path| File::open(&path).ok().map(|f| (path, f)))
        })
        .for_each(|(path, mut file)| {
            let mut reader = std::io::BufReader::new(&file);
            std::io::copy(&mut reader, &mut compile_writer).unwrap();
            compile_writer.write_all("\n\n".as_bytes()).unwrap();

            let _ = file.rewind();

            let search_reader = std::io::BufReader::new(file);
            
            let mut lines = search_reader.lines();

            let has_keyword = Some(lines.any(|line| {
                let contains = Some(search_terms.iter().any(|term| line.as_ref().ok().map(|line| line.contains(term)).filter(|t| *t).is_some())).filter(|t| *t).is_some();
                contains

            })).filter(|l| *l).map(|_| path);
            has_keyword.and_then(|path| {
                Some(println!("{}", path.to_string_lossy()))});
        });
}
