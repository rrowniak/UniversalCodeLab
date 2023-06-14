pub fn main() {
    error_chain::main();
    err_conv::main();
    backtrace::main();
}

mod error_chain {
    use error_chain::error_chain;

    use std::fs::File;
    use std::io::Read;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            ParseInt(::std::num::ParseIntError);
        }
    }

    fn read_uptime() -> Result<u64> {
        let mut uptime = String::new();
        File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

        Ok(uptime
            .split('.')
            .next()
            .ok_or("Cannot parse uptime data")?
            .parse()?)
    }

    pub fn main() {
        match read_uptime() {
            Ok(uptime) => println!("uptime: {} seconds", uptime),
            Err(err) => eprintln!("error: {}", err),
        };
    }
}

mod err_conv {

    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Reqwest(reqwest::Error);
            ParseIntError(std::num::ParseIntError);
        }
        errors { RandomResponseError(t: String) }
    }

    fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
        let mut body = response.text()?;
        body.pop();
        body.parse::<u32>()
            .chain_err(|| ErrorKind::RandomResponseError(body))
    }

    fn run() -> Result<()> {
        let url = "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain";
        let response = reqwest::blocking::get(url)?;
        let random_value: u32 = parse_response(response)?;
        println!("a random number between 0 and 10: {}", random_value);
        Ok(())
    }

    pub fn main() {
        if let Err(error) = run() {
            match *error.kind() {
                ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
                ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
                ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
                ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
                _ => println!("Other error: {:?}", error),
            }
        }
    }
}

mod backtrace {

    use error_chain::error_chain;
    use serde::Deserialize;

    use std::fmt;

    error_chain! {
        foreign_links {
            Reader(csv::Error);
        }
    }

    #[derive(Debug, Deserialize)]
    struct Rgb {
        red: u8,
        blue: u8,
        green: u8,
    }

    impl Rgb {
        fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
            let color: Rgb = csv::Reader::from_reader(csv_data)
                .deserialize()
                .nth(0)
                .ok_or("Cannot deserialize the first CSV record")?
                .chain_err(|| "Cannot deserialize RGB color")?;

            Ok(color)
        }
    }

    impl fmt::UpperHex for Rgb {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let hexa =
                u32::from(self.red) << 16 | u32::from(self.blue) << 8 | u32::from(self.green);
            write!(f, "{:X}", hexa)
        }
    }

    fn run() -> Result<()> {
        let csv = "red,blue,green
102,256,204";

        let rgb = Rgb::from_reader(csv.as_bytes()).chain_err(|| "Cannot read CSV data")?;
        println!("{:?} to hexadecimal #{:X}", rgb, rgb);

        Ok(())
    }

    pub fn main() {
        if let Err(ref errors) = run() {
            eprintln!("Error level - description");
            errors
                .iter()
                .enumerate()
                .for_each(|(index, error)| eprintln!("└> {} - {}", index, error));

            if let Some(backtrace) = errors.backtrace() {
                eprintln!("{:?}", backtrace);
            }

            // In a real use case, errors should handled. For example:
            // ::std::process::exit(1);
        }
    }
}
