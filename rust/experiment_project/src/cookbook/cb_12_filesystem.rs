pub fn main() {
    read_lines::main().unwrap();
    // dont_use_the_same_files::main().unwrap();
    mmap::main().unwrap();
    files_mod_in_24h::main().unwrap();
    // detect_loops::main();
    find_duplicates::main();
    find_all_jsons::main().unwrap();
    traverse_skip_dot_files::main();
    calculate_file_sizes::main();
    find_all_pngs::main().unwrap();
    find_all_regex::main().unwrap();
}

mod read_lines {

    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Write};

    pub fn main() -> Result<(), Error> {
        let path = "lines.txt";

        let mut output = File::create(path)?;
        write!(output, "Rust\n💖\nFun")?;

        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            println!("{}", line?);
        }

        Ok(())
    }
}

mod dont_use_the_same_files {
    // cargo run >> ./new.txt
    use same_file::Handle;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind};
    use std::path::Path;

    #[allow(dead_code)]
    pub fn main() -> Result<(), Error> {
        let path_to_read = Path::new("new.txt");

        let stdout_handle = Handle::stdout()?;
        let handle = Handle::from_path(path_to_read)?;

        if stdout_handle == handle {
            return Err(Error::new(
                ErrorKind::Other,
                "You are reading and writing to the same file",
            ));
        } else {
            let file = File::open(path_to_read)?;
            let file = BufReader::new(file);
            for (num, line) in file.lines().enumerate() {
                println!("{} : {}", num, line?.to_uppercase());
            }
        }

        Ok(())
    }
}

mod mmap {

    use memmap::Mmap;
    use std::fs::File;
    use std::io::{Error, Write};

    pub fn main() -> Result<(), Error> {
        write!(
            File::create("content.txt")?,
            "My hovercraft is full of eels!"
        )?;

        let file = File::open("content.txt")?;
        let map = unsafe { Mmap::map(&file)? };

        let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
        assert_eq!(&map[3..13], b"hovercraft");
        let random_bytes: Vec<u8> = random_indexes.iter().map(|&idx| map[idx]).collect();
        assert_eq!(&random_bytes[..], b"My loaf!");
        Ok(())
    }
}

mod files_mod_in_24h {

    use error_chain::error_chain;

    use std::{env, fs};

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            SystemTimeError(std::time::SystemTimeError);
        }
    }

    pub fn main() -> Result<()> {
        let current_dir = env::current_dir()?;
        println!(
            "Entries modified in the last 24 hours in {:?}:",
            current_dir
        );

        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            let metadata = fs::metadata(&path)?;
            let last_modified = metadata.modified()?.elapsed()?.as_secs();

            if last_modified < 24 * 3600 && metadata.is_file() {
                println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
            }
        }

        Ok(())
    }
}

mod detect_loops {

    // test commands:
    // mkdir -p /tmp/foo/bar/baz
    // ln -s /tmp/foo/  /tmp/foo/bar/baz/qux

    use same_file::is_same_file;
    use std::io;
    use std::path::{Path, PathBuf};

    #[allow(dead_code)]
    fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
        let path = path.as_ref();
        let mut path_buf = path.to_path_buf();
        while path_buf.pop() {
            if is_same_file(&path_buf, path)? {
                return Ok(Some((path_buf, path.to_path_buf())));
            } else if let Some(looped_paths) = contains_loop(&path_buf)? {
                return Ok(Some(looped_paths));
            }
        }
        Ok(None)
    }

    #[allow(dead_code)]
    pub fn main() {
        assert_eq!(
            contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
            Some((
                PathBuf::from("/tmp/foo"),
                PathBuf::from("/tmp/foo/bar/baz/qux")
            ))
        );
    }
}

mod find_duplicates {

    use std::collections::HashMap;
    use walkdir::WalkDir;

    pub fn main() {
        let mut filenames = HashMap::new();

        for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let f_name = String::from(entry.file_name().to_string_lossy());
            let counter = filenames.entry(f_name.clone()).or_insert(0);
            *counter += 1;

            if *counter == 2 {
                println!("{}", f_name);
            }
        }
    }
}

mod find_all_jsons {

    use error_chain::error_chain;

    use walkdir::WalkDir;

    error_chain! {
        foreign_links {
            WalkDir(walkdir::Error);
            Io(std::io::Error);
            SystemTime(std::time::SystemTimeError);
        }
    }

    pub fn main() -> Result<()> {
        println!("Find all json modified within a day");
        for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            let sec = entry.metadata()?.modified()?;

            // all jsons modified within a day
            if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
                println!("{}", f_name);
            }
        }

        Ok(())
    }
}

mod traverse_skip_dot_files {

    use walkdir::{DirEntry, WalkDir};

    fn is_not_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| entry.depth() == 0 || !s.starts_with('.'))
            .unwrap_or(false)
    }

    pub fn main() {
        WalkDir::new(".")
            .into_iter()
            .filter_entry(is_not_hidden)
            .filter_map(|v| v.ok())
            .for_each(|x| println!("{}", x.path().display()));
    }
}

mod calculate_file_sizes {

    use walkdir::WalkDir;

    pub fn main() {
        let total_size = WalkDir::new(".")
            .min_depth(1)
            .max_depth(3)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .fold(0, |acc, m| acc + m.len());

        println!("Total size (depth between 1 and 3): {} bytes.", total_size);
    }
}

mod find_all_pngs {

    use error_chain::error_chain;

    use glob::glob;

    error_chain! {
        foreign_links {
            Glob(glob::GlobError);
            Pattern(glob::PatternError);
        }
    }

    pub fn main() -> Result<()> {
        for entry in glob("**/*.png")? {
            println!("{}", entry?.display());
        }

        Ok(())
    }
}

mod find_all_regex {

    use error_chain::error_chain;
    use glob::{glob_with, MatchOptions};

    error_chain! {
        foreign_links {
            Glob(glob::GlobError);
            Pattern(glob::PatternError);
        }
    }

    pub fn main() -> Result<()> {
        let options = MatchOptions {
            case_sensitive: false,
            ..Default::default()
        };

        for entry in glob_with("./src/*/cb_[0-9]_*.rs", options)? {
            println!("{}", entry?.display());
        }

        Ok(())
    }
}
