use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Seek, Write};
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct Stride {
    text: String,
    prefixes: Vec<String>,
    wanted: bool,
    paths: Vec<PathBuf>,
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Stride {
    pub fn new(text: &'static str, prefixes: &'static [&'static str]) -> Self {
        Stride {
            text: text.into(),
            prefixes: prefixes.iter().map(|s| s.to_string()).collect(),
            wanted: false,
            paths: vec![],
        }
    }
}

const TOP_STRING: &[u8] = b"<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">
    <title>Stride</title>
  </head>
  <body>
    <nav>";

const MID_STRING: &[u8] = b"\n</nav>\n<main>\n";

const BOTTOM_STRING: &[u8] = b"</main>
  </body>
</html>
";

fn main() {
    let args = std::env::args();

    let mut search_dict = [
        Stride::new("Spoofing", &["s"]),
        Stride::new("Tampering", &["t"]),
        Stride::new("Repudiation", &["r"]),
        Stride::new("Information Disclosure", &["i"]),
        Stride::new("Denial of Service", &["d"]),
        Stride::new("Elevation of Privillege", &["e"]),
    ];

    args.for_each(|arg| {
        search_dict
            .iter_mut()
            .for_each(|search| search.wanted = search.prefixes.contains(&arg) || search.wanted)
    });

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
    let keywords: Vec<(PathBuf, Vec<Stride>)> = std::fs::read_dir(".")
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
        .map(|(path, mut file)| {
            let mut reader = BufReader::new(&file);
            std::io::copy(&mut reader, &mut compile_writer).unwrap();
            compile_writer.write_all("\n\n".as_bytes()).unwrap();

            let _ = file.rewind();

            let search_reader = BufReader::new(file);

            let mut lines = search_reader.lines();

            let has_keywords: Vec<Stride> = search_dict
                .iter()
                .filter(|term| {
                    lines.any(|line| {
                        line.map(|line| line.to_lowercase().contains(&term.text.to_lowercase()))
                            .ok()
                            .filter(|t| *t)
                            .is_some()
                    })
                })
                .map(|stride| (*stride).clone())
                .collect();

            let _ = Some(&has_keywords)
                .filter(|keywords| keywords.iter().any(|keyword| keyword.wanted))
                .map(|_| {
                    println!(
                        "{}",
                        path.parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                    )
                });

            (path, has_keywords)
        })
        .collect();

    keywords.iter().for_each(|(path, keywords)| {
        keywords.iter().for_each(|key| {
            let found = search_dict.iter_mut().find(|term| *term == key);
            found.map(|term| {
                term.paths.push(path.clone());
            });
        });
    });

    search_dict.iter().for_each(|_| {
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("./site.html")
            .map(BufWriter::new);

        let _ = writer.map(|mut writer| {
            let _ = writer.write_all(TOP_STRING);
            search_dict.iter().for_each(|stride| {
                let _ = writer.write_all(format!("\n     <h3>{}</h3><ul>", stride.text).as_bytes());
                stride
                    .paths
                    .iter()
                    .map(|path| path.parent().unwrap().file_name().unwrap())
                    .for_each(|dir| {
                        let _ = writer.write_all(
                            format!(
                                "<li><a href=\"#{}\">{}</a></li>",
                                dir.to_string_lossy().replace(' ', "-"),
                                dir.to_string_lossy()
                            )
                            .as_bytes(),
                        );
                    });
            });
            let _ = writer.write_all(MID_STRING);

            search_dict.iter().for_each(|stride| {
                let _ = writer.write_all(format!("<h2>{}</h2>\n<div>\n", stride.text).as_bytes());

                stride.paths.iter().for_each(|path| {
                    let _ = File::open(path).map(BufReader::new).map(|reader| {
                        let dir = path
                            .parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_string_lossy();
                        let _ = writer.write_all(
                            format!("<h3 id=\"{}\">{}</h3>\n", dir.replace(' ', "-"), dir)
                                .as_bytes(),
                        );
                        reader.lines().for_each(|line| {
                            let _ = line.map(|str| {
                                let _ = writer.write_all(str.as_bytes());
                            });
                            let _ = writer.write_all(b"<br />\n");
                        })
                    });
                });
                let _ = writer.write_all(b"</div>");
            });
            let _ = writer.write_all(BOTTOM_STRING);
            writer
        });
    });
}
