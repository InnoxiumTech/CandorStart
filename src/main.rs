use clap::{Arg, App};
use std::process::{Command, Stdio};
use std::io::{BufReader, Error, ErrorKind, BufRead};

fn main() -> Result<(), Error> {

    // Define the jar file, as we use it multiple times
    let candor_jar = "CandorManager-snapshot.jar";

    // Ask clap to parse our command line, and return a set of matches.
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

    // Create the candor process.
    let child = Command::new("java")
        .stdout(Stdio::piped())
        .arg(format!("{}{}", "-javaagent:", candor_jar))
        .arg("-jar")
        .arg(candor_jar)
        .arg("url")
        .arg(matches.value_of("url").unwrap())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not create std out"))?;

    // Create reader for us to pipe in to our own output
    let reader = BufReader::new(child);

    // Now we can funnel the output in to our stdout.
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    // Return either the error, or ok
    Ok(())
}