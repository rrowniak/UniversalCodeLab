pub fn main() {
    cmd_stdout::main().unwrap();
    child_process::main().unwrap();
    piped_commands::main().unwrap();
    redirect_stdioerr_to_file::main().unwrap();
    process_output_in_parallel::main().unwrap();
    env::main().unwrap();
}

mod cmd_stdout {
    use error_chain::error_chain;

    use regex::Regex;
    use std::process::Command;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Regex(regex::Error);
            Utf8(std::string::FromUtf8Error);
        }
    }

    #[derive(PartialEq, Default, Clone, Debug)]
    struct Commit {
        hash: String,
        message: String,
    }

    pub fn main() -> Result<()> {
        let output = Command::new("git").arg("log").arg("--oneline").output()?;

        if !output.status.success() {
            error_chain::bail!("Command executed with failing error code");
        }

        let pattern = Regex::new(
            r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message",
        )?;

        String::from_utf8(output.stdout)?
            .lines()
            .filter_map(|line| pattern.captures(line))
            .map(|cap| Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            })
            .take(5)
            .for_each(|x| println!("{:?}", x));

        Ok(())
    }
}

mod child_process {

    use error_chain::error_chain;

    use std::collections::HashSet;
    use std::io::Write;
    use std::process::{Command, Stdio};

    error_chain! {
        errors { CmdError }
        foreign_links {
            Io(std::io::Error);
            Utf8(std::string::FromUtf8Error);
        }
    }

    pub fn main() -> Result<()> {
        let mut child = Command::new("python3")
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        child
            .stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!")?
            .write_all(b"import this; copyright(); credits(); exit()")?;

        let output = child.wait_with_output()?;

        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout)?;
            let words = raw_output
                .split_whitespace()
                .map(|s| s.to_lowercase())
                .collect::<HashSet<_>>();
            println!("Found {} unique words:", words.len());
            println!("{:#?}", words);
            Ok(())
        } else {
            let err = String::from_utf8(output.stderr)?;
            error_chain::bail!("External command failed:\n {}", err)
        }
    }
}

mod piped_commands {

    use error_chain::error_chain;

    use std::process::{Command, Stdio};

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Utf8(std::string::FromUtf8Error);
        }
    }

    pub fn main() -> Result<()> {
        let directory = std::env::current_dir()?;
        let mut du_output_child = Command::new("du")
            .arg("-ah")
            .arg(&directory)
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(du_output) = du_output_child.stdout.take() {
            let mut sort_output_child = Command::new("sort")
                .arg("-hr")
                .stdin(du_output)
                .stdout(Stdio::piped())
                .spawn()?;

            du_output_child.wait()?;

            if let Some(sort_output) = sort_output_child.stdout.take() {
                let head_output_child = Command::new("head")
                    .args(["-n", "10"])
                    .stdin(sort_output)
                    .stdout(Stdio::piped())
                    .spawn()?;

                let head_stdout = head_output_child.wait_with_output()?;

                sort_output_child.wait()?;

                println!(
                    "Top 10 biggest files and directories in '{}':\n{}",
                    directory.display(),
                    String::from_utf8(head_stdout.stdout).unwrap()
                );
            }
        }

        Ok(())
    }
}

mod redirect_stdioerr_to_file {

    use std::fs::File;
    use std::io::Error;
    use std::process::{Command, Stdio};

    pub fn main() -> Result<(), Error> {
        let outputs = File::create("out.txt")?;
        let errors = outputs.try_clone()?;

        Command::new("ls")
            .args([".", "oops"])
            .stdout(Stdio::from(outputs))
            .stderr(Stdio::from(errors))
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }
}

mod process_output_in_parallel {

    use std::io::{BufRead, BufReader, Error, ErrorKind};
    use std::process::{Command, Stdio};

    pub fn main() -> Result<(), Error> {
        let stdout = Command::new("journalctl")
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

        let reader = BufReader::new(stdout);

        reader
            .lines()
            .filter_map(|line| line.ok())
            // .filter(|line| line.find("usb").is_some())
            .filter(|line| line.contains("usb"))
            .for_each(|line| println!("{}", line));

        Ok(())
    }
}

mod env {

    use std::env;
    use std::fs;
    use std::io::Error;

    pub fn main() -> Result<(), Error> {
        // read `config_path` from the environment variable `CONFIG`.
        // If `CONFIG` isn't set, fall back to a default config path.
        let config_path = env::var("CONFIG").unwrap_or("/etc/myapp/config".to_string());

        let config: String = fs::read_to_string(config_path)?;
        println!("Config: {}", config);

        Ok(())
    }
}
