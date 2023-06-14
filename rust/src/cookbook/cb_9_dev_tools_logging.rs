pub fn main(decision: usize) {
    match decision {
        0 => std_log_cfg(),
        // panic if called - cannot initialize logger twice
        1 => stdout_for_err(),
        2 => custom_log(),
        3 => syslog(),
        4 => logs_per_module(),
        5 => logs_cust_env_variable(),
        6 => logs_with_timestamp(),
        7 => log_to_file(),
        8 => versioning::main(),
        _ => println!("Make a selection"),
    }
}

fn std_log_cfg() {
    env_logger::init();
    println!("To see the log run $ RUST_LOG=debug cargo run log");
    println!("This is because the default log level is error");
    log::debug!("Logging this number: {}", 10);
    println!("Print error then:");
    log::error!("We've got an error!");
}

#[allow(dead_code)]
fn stdout_for_err() {
    use env_logger::{Builder, Target};

    Builder::new().target(Target::Stdout).init();

    log::error!("This error has been printed to stdout");
}

use log::{Level, LevelFilter, Metadata, Record};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn custom_log() {
    log::set_logger(&CONSOLE_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    log::info!("hello log");
    log::warn!("warning");
    log::error!("oops");
}

fn syslog() {
    use syslog::Facility;

    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )
    .unwrap();
    println!("Check out syslog");
    log::debug!("this is a debug {}", "message");
    log::error!("this is an error!");
}

////////////////////////

mod foo {
    mod bar {
        pub fn run() {
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
        }
    }

    pub fn run() {
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        bar::run();
    }
}

fn logs_per_module() {
    env_logger::init();
    println!("Logs per module. Use RUST_LOG=\"warn,test::foo=info,test::foo::bar=debug\" ./test");
    log::warn!("[root] warn");
    log::info!("[root] info");
    log::debug!("[root] debug");
    foo::run();
}

fn logs_cust_env_variable() {
    use env_logger::{Builder, Env};

    let env = Env::new().filter("MY_LOG");

    Builder::new()
        .filter_level(LevelFilter::Off)
        .parse_env(env)
        .init();

    println!("Run as MY_LOG=\"warn\" cargo run log 5");

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
}

fn logs_with_timestamp() {
    use chrono::Local;
    use env_logger::Builder;
    // use log::LevelFilter;
    use std::io::Write;

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
}

fn log_to_file() {
    // use log::LevelFilter;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    log::info!("Hello, world!");
}

mod versioning {

    use semver::{BuildMetadata, Error, Prerelease, Version};

    pub fn main() {
        let t = version_test_1();
        if let Err(e) = t {
            println!("Test 1 error: {:}", e);
        }
    }

    fn version_test_1() -> Result<(), Error> {
        let parsed_version = Version::parse("0.2.6")?;

        assert_eq!(
            parsed_version,
            Version {
                major: 0,
                minor: 2,
                patch: 6,
                pre: Prerelease::new("").unwrap(),
                build: BuildMetadata::EMPTY,
            }
        );

        Ok(())
    }
}
