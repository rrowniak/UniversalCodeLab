pub fn main() {
    if let Err(e) = percent_encoding() {
        println!("Percent ecoding error: {}", e);
    }

    form_urlencoded();

    if let Err(e) = hex_coding() {
        println!("Hex coding error: {}", e);
    }

    if let Err(e) = base64_enc() {
        println!("Hex coding error: {}", e);
    }

    csv::csv_tests();
}

use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use std::str::Utf8Error;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn percent_encoding() -> Result<(), Utf8Error> {
    let input = "confident, productive systems programming";

    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    Ok(())
}

use url::form_urlencoded::{byte_serialize, parse};

fn form_urlencoded() {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}

use data_encoding::{DecodeError, HEXUPPER};

fn hex_coding() -> Result<(), DecodeError> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}

use base64::{engine::general_purpose, Engine as _};
use std::str;

fn base64_enc() -> Result<(), base64::DecodeError> {
    let hello = b"hello rustaceans";
    let encoded = general_purpose::STANDARD_NO_PAD.encode(hello);
    let decoded = general_purpose::STANDARD_NO_PAD.decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello).unwrap());
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded).unwrap());

    Ok(())
}

mod csv {
    pub fn csv_tests() {
        if let Err(e) = parse_from_string() {
            println!("Parse from string error: {}", e);
        }

        if let Err(e) = parse_into_structure() {
            println!("Parse into structure error: {}", e);
        }

        if let Err(e) = custom_delimeter() {
            println!("Custom delimeter error: {}", e);
        }

        if let Err(e) = match_predicate() {
            println!("Match predicate error: {}", e);
        }

        if let Err(e) = invalid_data() {
            println!("Handling invalid data error: {}", e);
        }

        if let Err(e) = serialize_to_io() {
            println!("Serialize to io error: {}", e);
        }

        if let Err(e) = serialize_to_io_with_serde() {
            println!("Serialize to io with serde error: {}", e);
        }

        if let Err(e) = transform::csv_transform() {
            println!("csv transform error: {}", e);
        }

        if let Err(e) = structured::parse() {
            println!("structured parse error: {}", e);
        }

        if let Err(e) = structured::parse_toml() {
            println!("structured parse toml error: {}", e);
        }

        if let Err(e) = structured::parse_toml_custom() {
            println!("structured parse toml  custom error: {}", e);
        }

        if let Err(e) = byte_order::main() {
            println!("byte order error: {}", e);
        }
    }

    use csv::Error;

    fn parse_from_string() -> Result<(), Error> {
        let csv = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";

        let mut reader = csv::Reader::from_reader(csv.as_bytes());
        for record in reader.records() {
            let record = record?;
            println!(
                "In {}, {} built the {} model. It is a {}.",
                &record[0], &record[1], &record[2], &record[3]
            );
        }

        Ok(())
    }

    use serde::Deserialize;
    #[derive(Deserialize)]
    struct Record {
        year: u16,
        make: String,
        model: String,
        description: String,
    }

    fn parse_into_structure() -> Result<(), csv::Error> {
        let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

        let mut reader = csv::Reader::from_reader(csv.as_bytes());

        for record in reader.deserialize() {
            let record: Record = record?;
            println!(
                "In {}, {} built the {} model. It is a {}.",
                record.year, record.make, record.model, record.description
            );
        }

        Ok(())
    }

    #[derive(Debug, Deserialize)]
    struct Record2 {
        #[allow(dead_code)]
        name: String,
        #[allow(dead_code)]
        place: String,
        #[serde(deserialize_with = "csv::invalid_option")]
        #[allow(dead_code)]
        id: Option<u64>,
    }

    use csv::ReaderBuilder;

    fn custom_delimeter() -> Result<(), Error> {
        let data = "name\tplace\tid
        Mark\tMelbourne\t46
        Ashley\tZurich\t92";

        let mut reader = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(data.as_bytes());
        for result in reader.deserialize::<Record2>() {
            println!("{:?}", result?);
        }

        Ok(())
    }

    fn match_predicate() -> Result<(), Error> {
        let query = "CA";
        let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

        use std::io;
        let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.write_record(rdr.headers()?)?;

        println!("Records matching \"{}\" criteria", query);
        for result in rdr.records() {
            let record = result?;
            if record.iter().any(|field| field == query) {
                wtr.write_record(&record)?;
            }
        }

        wtr.flush()?;
        Ok(())
    }

    #[derive(Debug, Deserialize)]
    struct Record3 {
        #[allow(dead_code)]
        name: String,
        #[allow(dead_code)]
        place: String,
        #[serde(deserialize_with = "csv::invalid_option")]
        #[allow(dead_code)]
        id: Option<u64>,
    }

    fn invalid_data() -> Result<(), Error> {
        let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        println!("Handling invalid data...");
        for result in rdr.deserialize() {
            let record: Record3 = result?;
            println!("{:?}", record);
        }

        Ok(())
    }

    fn serialize_to_io() -> Result<(), Error> {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());

        wtr.write_record(["Name", "Place", "ID"])?;

        wtr.serialize(("Mark", "Sydney", 87))?;
        wtr.serialize(("Ashley", "Dublin", 32))?;
        wtr.serialize(("Akshat", "Delhi", 11))?;

        println!("Serialize to io");
        wtr.flush()?;
        Ok(())
    }

    use serde::Serialize;

    #[derive(Serialize)]
    struct RecordSerde<'a> {
        name: &'a str,
        place: &'a str,
        id: u64,
    }

    fn serialize_to_io_with_serde() -> Result<(), Error> {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());

        let rec1 = RecordSerde {
            name: "Mark",
            place: "Melbourne",
            id: 56,
        };
        let rec2 = RecordSerde {
            name: "Ashley",
            place: "Sydney",
            id: 64,
        };
        let rec3 = RecordSerde {
            name: "Akshat",
            place: "Delhi",
            id: 98,
        };

        wtr.serialize(rec1)?;
        wtr.serialize(rec2)?;
        wtr.serialize(rec3)?;

        println!("Serialize with serde");
        wtr.flush()?;

        Ok(())
    }
    pub mod transform {
        // transform CSV columns
        use csv::{Reader, Writer};
        use serde::{de, Deserialize, Deserializer};
        use std::str::FromStr;

        type Error = std::io::Error;

        #[derive(Debug)]
        struct HexColor {
            red: u8,
            green: u8,
            blue: u8,
        }

        #[derive(Debug, Deserialize)]
        struct Row {
            color_name: String,
            color: HexColor,
        }

        impl FromStr for HexColor {
            type Err = Error;

            fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
                let trimmed = hex_color.trim_matches('#');
                if trimmed.len() != 6 {
                    Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Invalid length of hex string",
                    ))
                } else {
                    Ok(HexColor {
                        red: u8::from_str_radix(&trimmed[..2], 16).unwrap(),
                        green: u8::from_str_radix(&trimmed[2..4], 16).unwrap(),
                        blue: u8::from_str_radix(&trimmed[4..6], 16).unwrap(),
                    })
                }
            }
        }

        impl<'de> Deserialize<'de> for HexColor {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                FromStr::from_str(&s).map_err(de::Error::custom)
            }
        }

        pub fn csv_transform() -> Result<(), Error> {
            let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00ff"
                .to_owned();
            let mut out = Writer::from_writer(vec![]);
            let mut reader = Reader::from_reader(data.as_bytes());
            for result in reader.deserialize::<Row>() {
                let res = result?;
                out.serialize((
                    res.color_name,
                    res.color.red,
                    res.color.green,
                    res.color.blue,
                ))?;
            }
            let written = String::from_utf8(out.into_inner().unwrap()).unwrap();
            assert_eq!(Some("magenta,255,0,255"), written.lines().last());
            println!("{}", written);
            Ok(())
        }
    }

    mod structured {

        use serde_json::json;
        use serde_json::{Error, Value};

        pub fn parse() -> Result<(), Error> {
            let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

            let parsed: Value = serde_json::from_str(j)?;

            let expected = json!({
                "userid": 103609,
                "verified": true,
                "access_privileges": [
                    "user",
                    "admin"
                ]
            });

            assert_eq!(parsed, expected);

            Ok(())
        }

        pub fn parse_toml() -> Result<(), toml::de::Error> {
            let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

            let package_info: toml::Value = toml::from_str(toml_content)?;

            assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
            assert_eq!(
                package_info["package"]["name"].as_str(),
                Some("your_package")
            );

            Ok(())
        }

        use serde::Deserialize;

        use std::collections::HashMap;

        #[derive(Deserialize)]
        struct Config {
            package: Package,
            dependencies: HashMap<String, String>,
        }

        #[derive(Deserialize)]
        struct Package {
            name: String,
            version: String,
            authors: Vec<String>,
        }

        pub fn parse_toml_custom() -> Result<(), toml::de::Error> {
            let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

            let package_info: Config = toml::from_str(toml_content)?;

            assert_eq!(package_info.package.name, "your_package");
            assert_eq!(package_info.package.version, "0.1.0");
            assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
            assert_eq!(package_info.dependencies["serde"], "1.0");

            Ok(())
        }
    }
    mod byte_order {
        use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
        use std::io::Error;

        #[derive(Default, PartialEq, Debug)]
        struct Payload {
            kind: u8,
            value: u16,
        }

        pub fn main() -> Result<(), Error> {
            let original_payload = Payload::default();
            let encoded_bytes = encode(&original_payload)?;
            let decoded_payload = decode(&encoded_bytes)?;
            assert_eq!(original_payload, decoded_payload);
            Ok(())
        }

        fn encode(payload: &Payload) -> Result<Vec<u8>, Error> {
            let mut bytes = vec![];
            bytes.write_u8(payload.kind)?;
            bytes.write_u16::<LittleEndian>(payload.value)?;
            Ok(bytes)
        }

        fn decode(mut bytes: &[u8]) -> Result<Payload, Error> {
            let payload = Payload {
                kind: bytes.read_u8()?,
                value: bytes.read_u16::<LittleEndian>()?,
            };
            Ok(payload)
        }
    }
}
