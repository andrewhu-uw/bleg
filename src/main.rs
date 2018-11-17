extern crate clap;
use clap::{App, Arg};

fn main() {
  let matches = App::new("Bleg")
    .version("0.1")
    .about("bleg")
    .author("Andrew Hu inspired by Britt \"Bleg\" Henderson")
    .arg(Arg::with_name("#")
      .index(1))
    .get_matches();

  if let Some(num_str) = matches.value_of("#") {
    let num = match num_str.parse::<i32>() {
      Ok(num) => num,
      Err(err) => {
        println!("{} could not be parsed as a number: \n{:?}", num_str, err);
        std::process::exit(1);
      }
    };
    for _ in 0..num {
      println!("bleg");
    }
  } else {
    println!("bleg");
  }
}
