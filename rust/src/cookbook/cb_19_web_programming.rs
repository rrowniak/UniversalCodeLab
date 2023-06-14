pub fn main() {
    let links = false;
    let urls = true;
    let mimes = false;
    let client = false;
    let webapis = false;
    let downloads = false;
    let auths = true;

    if links {
        links::extract_from_web::main().unwrap();
        links::broken_links::main().unwrap();
        links::extract_unique::main().unwrap();
    }

    if urls {
        url::parse::main().unwrap();
        url::base::main().unwrap();
        url::new::main().unwrap();
        url::extract_origin1::main().unwrap();
        url::extract_origin2::main().unwrap();
        url::remove::main().unwrap();
    }

    if mimes {
        mime::from_string::main();
        mime::from_filename::main();
        mime::parse_http_response::main().unwrap();
    }

    if client {
        clients::make_http_get_request::main().unwrap();
        clients::make_http_get_request_async::main().unwrap();
    }

    if webapis {
        clients::web_api::query_github_api::main().unwrap();
        clients::web_api::exists::main().unwrap();
        // clients::web_api::create_delete_post::main().unwrap();
        clients::web_api::paginated_api::main().unwrap();
    }

    if downloads {
        download::temp::main().unwrap();
        download::post_file::main().unwrap();
        download::partial_download::main().unwrap();
    }

    if auths {
        web_auth::basic::main().unwrap();
    }
}

mod links {
    pub mod extract_from_web {

        use error_chain::error_chain;
        use select::document::Document;
        use select::predicate::Name;

        error_chain! {
              foreign_links {
                  ReqError(reqwest::Error);
                  IoError(std::io::Error);
              }
        }

        #[tokio::main]
        pub async fn main() -> Result<()> {
            let res = reqwest::get("https://www.rust-lang.org/en-US/")
                .await?
                .text()
                .await?;

            Document::from(res.as_str())
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .for_each(|x| println!("{}", x));

            Ok(())
        }
    }

    pub mod broken_links {

        use error_chain::error_chain;
        use reqwest::StatusCode;
        use select::document::Document;
        use select::predicate::Name;
        use std::collections::HashSet;
        use url::{Position, Url};

        error_chain! {
          foreign_links {
              ReqError(reqwest::Error);
              IoError(std::io::Error);
              UrlParseError(url::ParseError);
              JoinError(tokio::task::JoinError);
          }
        }

        async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
            let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).next();
            let base_url = base_tag_href
                .map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
            Ok(base_url)
        }

        async fn check_link(url: &Url) -> Result<bool> {
            let res = reqwest::get(url.as_ref()).await?;
            Ok(res.status() != StatusCode::NOT_FOUND)
        }

        #[tokio::main]
        pub async fn main() -> Result<()> {
            println!("Checking broken links");
            let url = Url::parse("https://www.rust-lang.org/en-US/")?;
            let res = reqwest::get(url.as_ref()).await?.text().await?;
            let document = Document::from(res.as_str());
            let base_url = get_base_url(&url, &document).await?;
            let base_parser = Url::options().base_url(Some(&base_url));
            let links: HashSet<Url> = document
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .filter_map(|link| base_parser.parse(link).ok())
                .collect();
            let mut tasks = vec![];

            for link in links {
                tasks.push(tokio::spawn(async move {
                    match check_link(&link).await {
                        Ok(res) => {
                            if res {
                                println!("{} is OK", link);
                            } else {
                                println!("{} is Broken", link);
                            }
                        }
                        Err(e) => println!("{} is Broken with error: {}", link, e),
                    }
                }));
            }

            for task in tasks {
                task.await?
            }

            Ok(())
        }
    }
    pub mod extract_unique {

        use lazy_static::lazy_static;
        use regex::Regex;
        use std::borrow::Cow;
        use std::collections::HashSet;
        use std::error::Error;

        fn extract_links(content: &str) -> HashSet<Cow<str>> {
            lazy_static! {
                static ref WIKI_REGEX: Regex = Regex::new(
                    r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            "
                )
                .unwrap();
            }

            let links: HashSet<_> = WIKI_REGEX
                .captures_iter(content)
                .map(|c| match (c.name("internal"), c.name("external")) {
                    (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
                    (None, Some(val)) => Cow::from(val.as_str()),
                    _ => unreachable!(),
                })
                .collect();

            links
        }

        #[tokio::main]
        pub async fn main() -> Result<(), Box<dyn Error>> {
            println!("Extract unique");
            let content = reqwest::get(
                "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
            )
            .await?
            .text()
            .await?;

            println!("{:#?}", extract_links(content.as_str()));

            Ok(())
        }
    }
}

mod url {
    pub mod parse {

        use url::{ParseError, Url};

        pub fn main() -> Result<(), ParseError> {
            let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

            let parsed = Url::parse(s)?;
            println!("The path part of the URL is: {}", parsed.path());

            Ok(())
        }
    }

    pub mod base {

        use error_chain::error_chain;

        use url::Url;

        error_chain! {
            foreign_links {
                UrlParse(url::ParseError);
            }
            errors {
                CannotBeABase
            }
        }

        pub fn main() -> Result<()> {
            let full = "https://github.com/rust-lang/cargo?asdf";

            let url = Url::parse(full)?;
            let base = base_url(url)?;

            assert_eq!(base.as_str(), "https://github.com/");
            println!("The base of the URL is: {}", base);

            Ok(())
        }

        fn base_url(mut url: Url) -> Result<Url> {
            match url.path_segments_mut() {
                Ok(mut path) => {
                    path.clear();
                }
                Err(_) => {
                    return Err(Error::from_kind(ErrorKind::CannotBeABase));
                }
            }

            url.set_query(None);

            Ok(url)
        }
    }

    pub mod new {

        use url::{ParseError, Url};

        pub fn main() -> Result<(), ParseError> {
            let path = "/rust-lang/cargo";

            let gh = build_github_url(path)?;

            assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
            println!("The joined URL is: {}", gh);

            Ok(())
        }

        fn build_github_url(path: &str) -> Result<Url, ParseError> {
            const GITHUB: &str = "https://github.com";

            let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
            let joined = base.join(path)?;

            Ok(joined)
        }
    }

    pub mod extract_origin1 {

        use url::{Host, ParseError, Url};

        pub fn main() -> Result<(), ParseError> {
            let s = "ftp://rust-lang.org/examples";

            let url = Url::parse(s)?;

            assert_eq!(url.scheme(), "ftp");
            assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
            assert_eq!(url.port_or_known_default(), Some(21));
            println!("The origin is as expected!");

            Ok(())
        }
    }

    pub mod extract_origin2 {

        use error_chain::error_chain;

        use url::{Host, Origin, Url};

        error_chain! {
            foreign_links {
                UrlParse(url::ParseError);
            }
        }

        pub fn main() -> Result<()> {
            let s = "ftp://rust-lang.org/examples";

            let url = Url::parse(s)?;

            let expected_scheme = "ftp".to_owned();
            let expected_host = Host::Domain("rust-lang.org".to_owned());
            let expected_port = 21;
            let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

            let origin = url.origin();
            assert_eq!(origin, expected);
            println!("The origin is as expected!");

            Ok(())
        }
    }

    pub mod remove {

        use url::{ParseError, Position, Url};

        pub fn main() -> Result<(), ParseError> {
            let parsed =
                Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
            let cleaned: &str = &parsed[..Position::AfterPath];
            println!("cleaned: {}", cleaned);
            Ok(())
        }
    }
}

mod mime {
    pub mod from_string {

        use mime::{Mime, APPLICATION_OCTET_STREAM};

        pub fn main() {
            let invalid_mime_type = "i n v a l i d";
            let default_mime = invalid_mime_type
                .parse::<Mime>()
                .unwrap_or(APPLICATION_OCTET_STREAM);

            println!(
                "MIME for {:?} used default value {:?}",
                invalid_mime_type, default_mime
            );

            let valid_mime_type = "TEXT/PLAIN";
            let parsed_mime = valid_mime_type
                .parse::<Mime>()
                .unwrap_or(APPLICATION_OCTET_STREAM);

            println!(
                "MIME for {:?} was parsed as {:?}",
                valid_mime_type, parsed_mime
            );
        }
    }

    pub mod from_filename {

        use mime::Mime;

        fn find_mimetype(filename: &str) -> Mime {
            let parts: Vec<&str> = filename.split('.').collect();

            let res = match parts.last() {
                Some(v) => match *v {
                    "png" => mime::IMAGE_PNG,
                    "jpg" => mime::IMAGE_JPEG,
                    "json" => mime::APPLICATION_JSON,
                    &_ => mime::TEXT_PLAIN,
                },
                None => mime::TEXT_PLAIN,
            };
            res
        }

        pub fn main() {
            let filenames = vec!["foobar.jpg", "foo.bar", "foobar.png"];
            for file in filenames {
                let mime = find_mimetype(file);
                println!("MIME for {}: {}", file, mime);
            }
        }
    }

    pub mod parse_http_response {

        use error_chain::error_chain;
        use mime::Mime;
        use reqwest::header::CONTENT_TYPE;
        use std::str::FromStr;

        error_chain! {
           foreign_links {
               Reqwest(reqwest::Error);
               Header(reqwest::header::ToStrError);
               Mime(mime::FromStrError);
           }
        }

        #[tokio::main]
        pub async fn main() -> Result<()> {
            let response =
                reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png").await?;
            let headers = response.headers();

            match headers.get(CONTENT_TYPE) {
                None => {
                    println!("The response does not contain a Content-Type header.");
                }
                Some(content_type) => {
                    let content_type = Mime::from_str(content_type.to_str()?)?;
                    let media_type = match (content_type.type_(), content_type.subtype()) {
                        (mime::TEXT, mime::HTML) => "a HTML document",
                        (mime::TEXT, _) => "a text document",
                        (mime::IMAGE, mime::PNG) => "a PNG image",
                        (mime::IMAGE, _) => "an image",
                        _ => "neither text nor image",
                    };

                    println!("The reponse contains {}.", media_type);
                }
            };

            Ok(())
        }
    }
}

mod clients {
    pub mod make_http_get_request {

        use error_chain::error_chain;
        use std::io::Read;

        error_chain! {
            foreign_links {
                Io(std::io::Error);
                HttpRequest(reqwest::Error);
            }
        }

        pub fn main() -> Result<()> {
            println!("Calling http://httpbin.org/get");
            let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
            let mut body = String::new();
            res.read_to_string(&mut body)?;

            println!("Status: {}", res.status());
            println!("Headers:\n{:#?}", res.headers());
            println!("Body:\n{}", body);

            Ok(())
        }
    }

    pub mod make_http_get_request_async {

        use error_chain::error_chain;

        error_chain! {
            foreign_links {
                Io(std::io::Error);
                HttpRequest(reqwest::Error);
            }
        }

        #[tokio::main]
        pub async fn main() -> Result<()> {
            println!("Calling http://httpbin.org/get - async version");
            let res = reqwest::get("http://httpbin.org/get").await?;
            println!("Status: {}", res.status());
            println!("Headers:\n{:#?}", res.headers());

            let body = res.text().await?;
            println!("Body:\n{}", body);
            Ok(())
        }
    }

    pub mod web_api {
        pub mod query_github_api {

            use reqwest::Error;
            use serde::Deserialize;

            #[derive(Deserialize, Debug)]
            #[allow(dead_code)]
            struct User {
                login: String,
                id: u32,
            }

            #[tokio::main]
            pub async fn main() -> Result<(), Error> {
                let request_url = format!(
                    "https://api.github.com/repos/{owner}/{repo}/stargazers",
                    owner = "rust-lang-nursery",
                    repo = "rust-cookbook"
                );
                println!("{}", request_url);
                // let response = reqwest::get(&request_url).await?;
                static APP_USER_AGENT: &str = "user";

                let client = reqwest::Client::builder()
                    .user_agent(APP_USER_AGENT)
                    .build()?;
                let response = client.get(request_url).send().await?;
                // println!("Response: {:?}", response.text().await?);

                let users: Vec<User> = response.json().await?;
                println!("{:?}", users);
                Ok(())
            }
        }

        pub mod exists {

            use reqwest::ClientBuilder;
            use reqwest::Result;
            use std::time::Duration;

            #[tokio::main]
            pub async fn main() -> Result<()> {
                let user = "ferris-the-crab";
                let request_url = format!("https://api.github.com/users/{}", user);
                println!("{}", request_url);

                let timeout = Duration::new(5, 0);
                let client = ClientBuilder::new().timeout(timeout).build()?;
                let response = client.head(&request_url).send().await?;

                if response.status().is_success() {
                    println!("{} is a user!", user);
                } else {
                    println!("{} is not a user!", user);
                }

                Ok(())
            }
        }

        pub mod create_delete_post {

            use error_chain::error_chain;
            use reqwest::Client;
            use serde::Deserialize;
            use serde_json::json;
            use std::env;

            error_chain! {
                foreign_links {
                    EnvVar(env::VarError);
                    HttpRequest(reqwest::Error);
                }
            }

            #[derive(Deserialize, Debug)]
            #[allow(dead_code)]
            struct Gist {
                id: String,
                html_url: String,
            }

            #[tokio::main]
            #[allow(dead_code)]
            pub async fn main() -> Result<()> {
                let gh_user = env::var("GH_USER")?;
                let gh_pass = env::var("GH_PASS")?;

                let gist_body = json!({
                "description": "the description for this gist",
                "public": true,
                "files": {
                     "main.rs": {
                     "content": r#"fn main() { println!("hello world!");}"#
                    }
                }});

                let request_url = "https://api.github.com/gists";
                let response = Client::new()
                    .post(request_url)
                    .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
                    .json(&gist_body)
                    .send()
                    .await?;

                let gist: Gist = response.json().await?;
                println!("Created {:?}", gist);

                let request_url = format!("{}/{}", request_url, gist.id);
                let response = Client::new()
                    .delete(&request_url)
                    .basic_auth(gh_user, Some(gh_pass))
                    .send()
                    .await?;

                println!(
                    "Gist {} deleted! Status code: {}",
                    gist.id,
                    response.status()
                );
                Ok(())
            }
        }

        pub mod paginated_api {

            use reqwest::Result;
            use serde::Deserialize;

            #[derive(Deserialize)]
            struct ApiResponse {
                dependencies: Vec<Dependency>,
                meta: Meta,
            }

            #[derive(Deserialize)]
            #[allow(dead_code)]
            struct Dependency {
                crate_id: String,
            }

            #[derive(Deserialize)]
            struct Meta {
                total: u32,
            }

            struct ReverseDependencies {
                crate_id: String,
                dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
                client: reqwest::blocking::Client,
                page: u32,
                per_page: u32,
                total: u32,
            }

            impl ReverseDependencies {
                // #[allow(dead_code)]
                fn of(crate_id: &str) -> Result<Self> {
                    Ok(ReverseDependencies {
                        crate_id: crate_id.to_owned(),
                        dependencies: vec![].into_iter(),
                        // client: reqwest::blocking::Client::new(),
                        client: reqwest::blocking::Client::builder()
                            .user_agent("user")
                            .build()?,
                        page: 0,
                        per_page: 100,
                        total: 0,
                    })
                }

                fn try_next(&mut self) -> Result<Option<Dependency>> {
                    if let Some(dep) = self.dependencies.next() {
                        return Ok(Some(dep));
                    }

                    if self.page > 0 && self.page * self.per_page >= self.total {
                        return Ok(None);
                    }

                    self.page += 1;
                    let url = format!("https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
                          self.crate_id,
                          self.page,
                          self.per_page);
                    println!("Calling {}", url);
                    let response = self.client.get(url).send()?;
                    println!("Response: {:?}", response);
                    let response = response.json::<ApiResponse>()?;
                    self.dependencies = response.dependencies.into_iter();
                    self.total = response.meta.total;
                    Ok(self.dependencies.next())
                }
            }

            impl Iterator for ReverseDependencies {
                type Item = Result<Dependency>;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.try_next() {
                        Ok(Some(dep)) => Some(Ok(dep)),
                        Ok(None) => None,
                        Err(err) => Some(Err(err)),
                    }
                }
            }

            pub fn main() -> Result<()> {
                for dep in ReverseDependencies::of("serde")? {
                    println!("reverse dependency: {}", dep?.crate_id);
                }
                Ok(())
            }
        }
    }
}

mod download {
    pub mod temp {

        use error_chain::error_chain;
        use std::fs::File;
        use std::io::copy;
        use tempfile::Builder;

        error_chain! {
             foreign_links {
                 Io(std::io::Error);
                 HttpRequest(reqwest::Error);
             }
        }

        #[tokio::main]
        pub async fn main() -> Result<()> {
            let tmp_dir = Builder::new().prefix("example").tempdir()?;
            let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
            let response = reqwest::get(target).await?;

            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");

                println!("file to download: '{}'", fname);
                let fname = tmp_dir.path().join(fname);
                println!("will be located under: '{:?}'", fname);
                File::create(fname)?
            };
            let content = response.text().await?;
            copy(&mut content.as_bytes(), &mut dest)?;
            Ok(())
        }
    }

    pub mod post_file {

        use error_chain::error_chain;
        use std::fs::File;
        use std::io::Read;

        error_chain! {
            foreign_links {
                HttpRequest(reqwest::Error);
                IoError(::std::io::Error);
            }
        }
        #[tokio::main]

        pub async fn main() -> Result<()> {
            let paste_api = "https://paste.rs";
            let mut file = File::open("content.txt")?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let client = reqwest::Client::new();
            let res = client.post(paste_api).body(contents).send().await?;
            let response_text = res.text().await?;
            println!("Your paste is located at: {}", response_text);
            Ok(())
        }
    }

    pub mod partial_download {

        use error_chain::error_chain;
        use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
        use reqwest::StatusCode;
        use std::fs::File;
        use std::str::FromStr;

        error_chain! {
            foreign_links {
                Io(std::io::Error);
                Reqwest(reqwest::Error);
                Header(reqwest::header::ToStrError);
            }
        }

        struct PartialRangeIter {
            start: u64,
            end: u64,
            buffer_size: u32,
        }

        impl PartialRangeIter {
            pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
                if buffer_size == 0 {
                    Err("invalid buffer_size, give a value greater than zero.")?;
                }
                Ok(PartialRangeIter {
                    start,
                    end,
                    buffer_size,
                })
            }
        }

        impl Iterator for PartialRangeIter {
            type Item = HeaderValue;
            fn next(&mut self) -> Option<Self::Item> {
                if self.start > self.end {
                    None
                } else {
                    let prev_start = self.start;
                    self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
                    Some(
                        HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                            .expect("string provided by format!"),
                    )
                }
            }
        }

        pub fn main() -> Result<()> {
            let url = "https://httpbin.org/range/102400?duration=2";
            const CHUNK_SIZE: u32 = 10240;

            let client = reqwest::blocking::Client::new();
            let response = client.head(url).send()?;
            let length = response
                .headers()
                .get(CONTENT_LENGTH)
                .ok_or("response doesn't include the content length")?;
            let length =
                u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;

            let mut output_file = File::create("download.bin")?;

            println!("starting download...");
            for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
                println!("range {:?}", range);
                let mut response = client.get(url).header(RANGE, range).send()?;

                let status = response.status();
                if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
                    error_chain::bail!("Unexpected server response: {}", status)
                }
                std::io::copy(&mut response, &mut output_file)?;
            }

            let content = response.text()?;
            std::io::copy(&mut content.as_bytes(), &mut output_file)?;

            println!("Finished with success!");
            Ok(())
        }
    }
}

mod web_auth {
    pub mod basic {

        use reqwest::blocking::Client;
        use reqwest::Error;

        pub fn main() -> Result<(), Error> {
            let client = Client::new();

            let user_name = "testuser".to_string();
            let password: Option<String> = None;

            let response = client
                .get("https://httpbin.org/")
                .basic_auth(user_name, password)
                .send();

            println!("{:?}", response);

            Ok(())
        }
    }
}
