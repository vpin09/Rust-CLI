use std::mem::take;

use clap::{Arg, ArgAction, Command};
fn main() {
    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("Vipin Saini sainivpin@gmal.com")
        .about("Teaches argument parsing")
        .arg(Arg::new("file").short('f').long("file").help("A cool file"))
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("five less than your fav number"),
        )
        .get_matches();
    let default_file = "input.txt".to_owned();

    let myfile = matches.get_one::<String>("file").unwrap_or(&default_file);

    println!("The file passed is: {}", myfile);
    let num_str = matches.get_one::<String>("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        },
    }
}
