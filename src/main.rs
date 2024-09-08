use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::env;
use git2::Repository;
use walkdir::WalkDir;

const 
   SOURCE_CODE_EXTENSIONS: [&str;36] = ["go", "rs", "cpp", "java", "dart", "py", "js", "ts", "kt", "swift", "kt", "rb", "php", "cs", "c", "c++", "c#", "scala", "groovy", "perl", "r", "sql", "pl", "asm", "vb", "lua", "haskell", "clojure", "elixir", "f#", "fortran", "lisp", "ocaml", "pascal", "prolog", "racket", ];

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
        return Ok(());
    }

    let repo_url = &args[1];
    let output_file = &args[2];
    let local_path = "cloned_repo";

    //check local path exist
    if Path::new(local_path).exists() {
        //try to remove local directory
        if let Err(e) = fs::remove_dir_all(local_path) {
            eprintln!("Failed to remove directory: {}", e);
        }
    }

    // Clone the repository
    clone_repo(repo_url, local_path)?;

    // Process the cloned repository
    process_repo(local_path, output_file)?;

    println!("Source code extraction completed. Check {}", output_file);
    Ok(())
}

fn print_usage() {
    println!("Usage: {} <repository_url> <output_file>", env::args().next().unwrap());
    println!("Example: {} https://github.com/example/repo.git all_source_code.txt", env::args().next().unwrap());
}

fn clone_repo(url: &str, path: &str) -> io::Result<()> {
    println!("Cloning repository from {}", url);
    match Repository::clone(url, path) {
        Ok(_) => println!("Repository cloned successfully"),
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    }
    Ok(())
}

fn process_repo(repo_path: &str, output_file: &str) -> io::Result<()> {
    let mut output = File::create(output_file)?;

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if SOURCE_CODE_EXTENSIONS.contains(&extension.to_str().unwrap_or("")) {
                    let relative_path = path.strip_prefix(repo_path).unwrap();
                    writeln!(output, "File: {}", relative_path.display())?;
                    writeln!(output, "{}", "-".repeat(80))?;
                    
                    let content = fs::read_to_string(path)?;
                    writeln!(output, "{}", content)?;
                    writeln!(output, "{}", "=".repeat(80))?;
                    writeln!(output)?;
                }
            }
        }
    }

    Ok(())
}