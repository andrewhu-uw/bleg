extern crate clap;
use clap::{App, Arg};

fn main() {
  let matches = App::new("Bleg")
    .version("0.1")
    .about("bleg")
    .author("Andrew Hu inspired by Britt \"Bleg\" Henderson")
    .arg(Arg::from_usage(
      "[#]"))
    .arg(Arg::with_name("ASCII")
      .long("ascii")
      .short("a"))
    .get_matches();

  let output = 
    if matches.is_present("ASCII") {
" ____  _           
|  _ \\| |           
| |_) | | ___  __ _ 
|  _ <| |/ _ \\/ _` |
| |_) | |  __/ (_| |
|____/|_|\\___|\\__, |
               __/ |
              |___/ "
    } else {
      "bleg"
    };

  if let Some(num_str) = matches.value_of("#") {
    let num = match num_str.parse::<i32>() {
      Ok(num) => num,
      Err(err) => {
        println!("{} could not be parsed as a number: \n{:?}", num_str, err);
        std::process::exit(1);
      }
    };
    for _ in 0..num {
      println!("{}", output);
    }
  } else {
    println!("{}", output);
  }
}
