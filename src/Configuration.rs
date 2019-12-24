use clap::{ArgMatches, App, Arg};
use std::net::SocketAddr;
use std::str::FromStr;
use std::env;
use std::path::Path;


pub struct Config {
    pub html_path : String,
    pub url : SocketAddr
}

impl Config {
    pub fn new() ->Config {
        let matches = App::new("Test clap App")
            .author(crate_authors!())
            .version(crate_version!())
            .about("Awesome web server")
            .arg(Arg::with_name("URL")
                .short("u")
                .long("url")
                .help("url (including port). Example: 127.0.0.1:8888")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("DIR")
            .short("d")
            .long("dir")
            .help("Directory to serve HTML from.")
            .takes_value(true)
            )
            .get_matches();

            return Config {url: Config::get_url(&matches), html_path: Config::get_dir(&matches)};
    }

    fn get_url(matches: &ArgMatches) -> SocketAddr {
        let url_str_val = matches.value_of("URL").unwrap();
        let url = SocketAddr::from_str(url_str_val);
        if url.is_err(){
            panic!("URL not valid")
        }
        url.unwrap()
    }

    fn get_dir(matches: &ArgMatches) -> String {
        let dir_str = matches.value_of("DIR");
        match dir_str{
            Some(x) =>String::from(x),
            None => String::from((env::current_dir().unwrap().join("html")).to_str().unwrap())
        }
    }
}