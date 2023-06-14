pub fn main() {
    extract_login_from_email::main();
    extract_hashtags::main();
    extract_phone::main().unwrap();
    filter_log::main().unwrap();
    replace::main();
    unicode_graphemes::main();
    from_str_trait::main();
}

mod extract_login_from_email {

    use lazy_static::lazy_static;

    use regex::Regex;

    fn extract_login(input: &str) -> Option<&str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            "
            )
            .unwrap();
        }
        RE.captures(input)
            .and_then(|cap| cap.name("login").map(|login| login.as_str()))
    }

    pub fn main() {
        assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
        assert_eq!(
            extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
            Some(r"sdf+sdsfsd.as.sdsd")
        );
        assert_eq!(extract_login(r"More@Than@One@at.com"), None);
        assert_eq!(extract_login(r"Not an email@email"), None);
    }
}

mod extract_hashtags {

    use lazy_static::lazy_static;

    use regex::Regex;
    use std::collections::HashSet;

    fn extract_hashtags(text: &str) -> HashSet<&str> {
        lazy_static! {
            static ref HASHTAG_REGEX: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
        }
        HASHTAG_REGEX
            .find_iter(text)
            .map(|mat| mat.as_str())
            .collect()
    }

    pub fn main() {
        let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
        let tags = extract_hashtags(tweet);
        assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
        assert_eq!(tags.len(), 3);
    }
}

mod extract_phone {

    use error_chain::error_chain;

    use regex::Regex;
    use std::fmt;

    error_chain! {
        foreign_links {
            Regex(regex::Error);
            Io(std::io::Error);
        }
    }

    struct PhoneNumber<'a> {
        area: &'a str,
        exchange: &'a str,
        subscriber: &'a str,
    }

    impl<'a> fmt::Display for PhoneNumber<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
        }
    }

    pub fn main() -> Result<()> {
        let phone_text = "
    +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
    (202) 991 9534
    Alex 5553920011
    1 (800) 233-2010
    1.299.339.1020";

        let re = Regex::new(
            r#"(?x)
          (?:\+?1)?                       # Country Code Optional
          [\s\.]?
          (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
          [\s\.\-]?
          ([2-9]\d{2})                    # Exchange Code
          [\s\.\-]?
          (\d{4})                         # Subscriber Number"#,
        )?;

        let phone_numbers = re.captures_iter(phone_text).filter_map(|cap| {
            let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
            match groups {
                (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                    area: area.as_str(),
                    exchange: ext.as_str(),
                    subscriber: sub.as_str(),
                }),
                _ => None,
            }
        });

        assert_eq!(
            phone_numbers.map(|m| m.to_string()).collect::<Vec<_>>(),
            vec![
                "1 (505) 881-9292",
                "1 (505) 778-2212",
                "1 (505) 881-9297",
                "1 (202) 991-9534",
                "1 (555) 392-0011",
                "1 (800) 233-2010",
                "1 (299) 339-1020",
            ]
        );

        Ok(())
    }
}

mod filter_log {

    use error_chain::error_chain;

    use regex::RegexSetBuilder;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Regex(regex::Error);
        }
    }

    pub fn main() -> Result<()> {
        let log_path = "application.log";
        let buffered = BufReader::new(File::open(log_path)?);

        let set = RegexSetBuilder::new([
            r#"version "\d\.\d\.\d""#,
            r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
            r#"warning.*timeout expired"#,
        ])
        .case_insensitive(true)
        .build()?;

        buffered
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| set.is_match(line.as_str()))
            .for_each(|x| println!("{}", x));

        Ok(())
    }
}

mod replace {

    use lazy_static::lazy_static;

    use regex::Regex;
    use std::borrow::Cow;

    fn reformat_dates(before: &str) -> Cow<str> {
        lazy_static! {
            static ref ISO8601_DATE_REGEX: Regex =
                Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        }
        ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
    }

    pub fn main() {
        let before = "2012-03-14, 2013-01-15 and 2014-07-05";
        let after = reformat_dates(before);
        assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
    }
}

mod unicode_graphemes {

    use unicode_segmentation::UnicodeSegmentation;

    pub fn main() {
        let name = "José Guimarães\r\n";
        let graphemes = UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
        assert_eq!(graphemes[3], "é");
    }
}

mod from_str_trait {

    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    struct Rgb {
        r: u8,
        g: u8,
        b: u8,
    }

    impl FromStr for Rgb {
        type Err = std::num::ParseIntError;

        // Parses a color hex code of the form '#rRgGbB..' into an
        // instance of 'Rgb'
        fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
            // u8::from_str_radix(src: &str, radix: u32) converts a string
            // slice in a given base to u8
            let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
            let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
            let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

            Ok(Rgb { r, g, b })
        }
    }

    pub fn main() {
        let code: &str = r"#fa7268";
        match Rgb::from_str(code) {
            Ok(rgb) => {
                println!(
                    r"The Rgb color code is: R: {} G: {} B: {}",
                    rgb.r, rgb.g, rgb.b
                );
            }
            Err(_) => {
                println!("{} is not a valid color hex code!", code);
            }
        }

        // test whether from_str performs as expected
        assert_eq!(
            Rgb::from_str(r"#fa7268").unwrap(),
            Rgb {
                r: 250,
                g: 114,
                b: 104
            }
        );
    }
}
