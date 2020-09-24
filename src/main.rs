use clap::{Arg, App};

fn main() {

    let matches = App::new("CandorStart")
        .version("1.0")
        .author("Innoxium Tech <innoxium.co.uk>")
        .about("Launches Candor")
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .value_name("URL")
            .about("Sets the url of the mod to download")
            .required(true)
            .takes_value(true))
        .get_matches();

    if matches.is_present("url") {

        println!("{}", matches.value_of("url").unwrap());
    }
}