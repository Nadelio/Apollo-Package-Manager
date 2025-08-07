use curl::easy::{Easy, List};
use std::{fs::File, io::{ stdout, Write }};
use ansi_term::Colour::{ Green, Red, Yellow };

fn process_response(data: &[u8]) {
    stdout().write_all(data).unwrap();
    print!("\n");
}

fn github_api_request(url: &str) -> Easy {
    let mut easy= Easy::new();
    easy.url(&format!("https://api.github.com/{url}")).unwrap();
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
    let mut local_file = File::create_new(format!("./{file}")).unwrap();
    let mut easy = Easy::new();
    easy.follow_location(true).unwrap();
    easy.url(&format!("https://github.com/{publisher}/{repo}/blob/main/apollo.lib?raw=true")).unwrap();
    easy.write_function(move |data| {
        local_file.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    easy
}

fn main() {
    let mut easy = github_api_request("search/repositories?q=filename:apollo+extension:.lib");

    let mut response = easy.response_code().unwrap();
    if response == 200 {
        println!("{}", Green.paint("Request Successful"));
    } else {
        println!("{} {}", Red.paint("Recieved Non-200 Response:"), Yellow.paint(format!("{response}")));
    }

    easy = get_github_file("Nadelio", "Hades-Programming-Language", "apollo.lib");

    response = easy.response_code().unwrap();
    if response == 200 {
        println!("{}", Green.paint("Request Successful"));
    } else {
        println!("{} {}", Red.paint("Recieved Non-200 Response:"), Yellow.paint(format!("{response}")));
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