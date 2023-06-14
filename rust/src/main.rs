use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};
mod cookbook;
mod string_manip;
mod trees;

fn main() {
    // clap tutorial: https://docs.rs/clap/latest/clap/_tutorial/index.html
    let matches = command!()
        .version("1.0")
        .author("Rafal Rowniak")
        .about("Playground for Rust")
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(Command::new("trees").about("Experiments with trees"))
        .subcommand(Command::new("algo").about("Algorithms from the Rust Cookbook"))
        .subcommand(Command::new("cmd").about("Command line from the Rust Cookbook"))
        .subcommand(
            Command::new("tarball")
                .about("Working with tarballs from the Rust Cookbook")
                .arg(
                    arg!(-d --decompress <FILE> "decompress tarball")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-c --compress <DIR> "compress a directory to tarball")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(Command::new("concurrency").about("Concurrency from the Rust Cookbook"))
        .subcommand(
            Command::new("sha256")
                .about("Calculate sha256 for all files in given directory from the Rust Cookbook")
                .arg(
                    arg!([DIR])
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("thumbnails")
                .about("Generate jpg thumbnails in parallel")
                .arg(
                    arg!([DIR])
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(Command::new("db").about("Sqllite tests"))
        .subcommand(Command::new("datetime").about("Date and time experiments"))
        .subcommand(
            Command::new("log")
                .about("Development tools, logging, versioning and build time tooling")
                .arg(arg!([demo] "select demo").required(true)),
        )
        .subcommand(Command::new("cpp").about("Interoperability with C and C++ code"))
        .subcommand(Command::new("enc").about("Encoding: character sets, CSVm structured data"))
        .subcommand(Command::new("err").about("Error handling"))
        .subcommand(Command::new("fs").about("File system"))
        .subcommand(Command::new("hardware").about("Hardware support"))
        .subcommand(Command::new("mem").about("Memory management"))
        .subcommand(Command::new("net").about("Network e.g. tcp/ip server on loopback"))
        .subcommand(Command::new("os").about("Operating system, calling commands"))
        .subcommand(Command::new("sci").about("Science - math: linear algebra, trigonometry, complex numbers, statistics, miscellaneous"))
        .subcommand(Command::new("text").about("Tex processing"))
        .subcommand(Command::new("web").about("Web programming"))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "list test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    } else if matches.subcommand_matches("trees").is_some() {
        trees::main();
    } else if matches.subcommand_matches("algo").is_some() {
        cookbook::cb_1_algorithms::main();
    } else if matches.subcommand_matches("cmd").is_some() {
        cookbook::cb_2_command_line::main();
    } else if let Some(matches) = matches.subcommand_matches("tarball") {
        if let Some(tarball_file) = matches.get_one::<PathBuf>("decompress") {
            println!("Decompress: {}", tarball_file.display());
            cookbook::cb_3_tarball::decompress(tarball_file);
        } else if let Some(tarball_dir) = matches.get_one::<PathBuf>("compress") {
            println!("Compress: {}", tarball_dir.display());
            cookbook::cb_3_tarball::compress(tarball_dir);
        }
    } else if matches.subcommand_matches("concurrency").is_some() {
        cookbook::cb_4_concurrency::main();
    } else if let Some(matches) = matches.subcommand_matches("sha256") {
        if let Some(path) = matches.get_one::<PathBuf>("DIR") {
            cookbook::cb_4_concurrency::sha256_files(path);
        }
    } else if let Some(matches) = matches.subcommand_matches("thumbnails") {
        if let Some(path) = matches.get_one::<PathBuf>("DIR") {
            cookbook::cb_4_concurrency::generate_thumbnails_parallel(path);
        }
    } else if matches.subcommand_matches("db").is_some() {
        cookbook::cb_7_database::main();
    } else if matches.subcommand_matches("datetime").is_some() {
        cookbook::cb_8_date_time::main();
    } else if let Some(matches) = matches.subcommand_matches("log") {
        if let Some(demo) = matches.get_one::<String>("demo") {
            cookbook::cb_9_dev_tools_logging::main(demo.parse::<usize>().unwrap());
        }
    } else if matches.subcommand_matches("cpp").is_some() {
        cookbook::cb_9_cpp::main();
    } else if matches.subcommand_matches("enc").is_some() {
        cookbook::cb_10_encoding::main();
    } else if matches.subcommand_matches("err").is_some() {
        cookbook::cb_11_error_handling::main();
    } else if matches.subcommand_matches("fs").is_some() {
        cookbook::cb_12_filesystem::main();
    } else if matches.subcommand_matches("hardware").is_some() {
        cookbook::cb_13_hardware::main();
    } else if matches.subcommand_matches("mem").is_some() {
        cookbook::cb_14_mem_management::main();
    } else if matches.subcommand_matches("net").is_some() {
        cookbook::cb_15_network::main();
    } else if matches.subcommand_matches("os").is_some() {
        cookbook::cb_16_os::main();
    } else if matches.subcommand_matches("sci").is_some() {
        cookbook::cb_17_science::main();
    } else if matches.subcommand_matches("text").is_some() {
        cookbook::cb_18_text::main();
    } else if matches.subcommand_matches("web").is_some() {
        cookbook::cb_19_web_programming::main();
    }
}
