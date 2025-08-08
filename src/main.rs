use curl::easy::{Easy, List};
use std::{fs::File, io::{ stdout, Write }};
use ansi_term::Colour::{ Green, Red, Yellow };

struct BuildAPIRequest {
    request: String
}

impl BuildAPIRequest {
    fn new() -> Self {
        BuildAPIRequest { request: "".to_string() }
    }

    fn search(&mut self) -> &mut Self {
        self.request.push_str("/search");
        self
    }

    fn repo(&mut self) -> &mut Self {
        self.request.push_str("/repositories");
        self
    }

    fn arg(&mut self) -> &mut Self {
        self.request.push_str("?q=");
        self
    }

    fn filename(&mut self, name: &str) -> &mut Self {
        self.request.push_str("filename:");
        self.request.push_str(&name);
        self.request.push_str("+");
        self
    }

    fn extension(&mut self, extension: &str) -> &mut Self {
        self.request.push_str("extension:");
        self.request.push_str(&extension);
        self.request.push_str("+");
        self
    }

    fn build(&self) -> &str {
        &self.request
    }
}

fn process_response(data: &[u8]) {
    stdout().write_all(data).unwrap();
    print!("\n");
}

fn github_api_request(url: &str) -> Easy {
    let mut easy= Easy::new();
    easy.url(&format!("https://api.github.com{url}")).unwrap();
    let mut header = List::new();
    header.append("User-Agent: Apollo-Package-Manager").unwrap();
    header.append("Accept: application/vnd.github+json").unwrap();
    header.append("X-GitHub-Api-Version: 2022-11-28").unwrap();
    header.append("Connection: close").unwrap();
    easy.http_headers(header).unwrap();
    easy.write_function(|data| {
        process_response(data);
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    easy
}

fn get_github_file(publisher: &str, repo: &str, file: &str) -> Easy {
    let file_path = format!("./{file}");
    
    let mut local_file: File = File::create(file_path).unwrap();
    
    let mut easy = Easy::new();
    easy.follow_location(true).unwrap();
    easy.url(&format!("https://github.com/{publisher}/{repo}/blob/main/{file}?raw=true")).unwrap();
    easy.write_function(move |data| {
        local_file.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    easy
}

fn main() {
    let mut request = BuildAPIRequest::new();
    let url: &str = request.search()
                           .repo()
                           .arg()
                           .filename("apollo")
                           .extension(".lib")
                           .build();

    let mut easy = github_api_request(url);

    let mut response = easy.response_code().unwrap();
    if response == 200 {
        println!("{}", Green.paint("Request Successful"));
    } else {
        println!("{} {}", Red.paint("Recieved Non-200 Response:"), Yellow.paint(format!("{response}")));
    }

    println!("Testing rate limits...");
    for _ in 1..100 {
        easy = get_github_file("Nadelio", "Hades-Programming-Language", "README.md");

        response = easy.response_code().unwrap();
        if response == 200 {
            println!("{}", Green.paint("Request Successful"));
        } else {
            println!("{} {}", Red.paint("Recieved Non-200 Response:"), Yellow.paint(format!("{response}")));
        }
    }
}

/*

./
> nadelio/
> > stdlib/
> > > package.json
> wolfltd/
> > network/
> > > package.json

*/

/*
curl "https://api.github.com/search/repositories?q=filename:apollo+extension:.lib"

curl "https://github.com/{publisher}/{repo}/blob/main/{file}?raw=true" -L >> {file}
*/

/*

> apm find 'hades-pl'

$ 1. hades-pl by Nadelio
$ 2. hades-pl by WolfLtd

? info 1

$ Publisher: Nadelio
$ Repository: Hades-Programming-Language
$ Version: 1.0.0
$ Dependecies:
$   Java 23.0.0 by Oracle

> apm find 'hades-pl'

$ 1. hades-pl by Nadelio
$ 2. hades-pl by WolfLtd

? install 1

$ Installing...
$ Install complete!
$ hades-pl located at ~/Apollo/Packages/Nadelio/hades-pl

> 
*/