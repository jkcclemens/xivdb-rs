extern crate xivdb;

use xivdb::XivDb;

use std::env::args;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
  let name = match args().nth(1) {
    Some(n) => n,
    None => {
      println!("Please specify a name");
      return;
    }
  };

  let mut params = HashMap::new();
  params.insert(String::from("one"), String::from("characters"));

  let xivdb = XivDb::default();
  let results = match xivdb.search(name, params) {
    Ok(r) => r,
    Err(e) => {
      println!("could not search: {}", e);
      return;
    }
  };
  let characters = results.characters.unwrap();
  println!("Which character?");
  for (i, val) in characters.results.iter().enumerate() {
    let name = val["name"].as_str().unwrap_or("<unknown name>");
    let server = val["server"].as_str().unwrap_or("<unknown server>");
    println!("  {}: {} on {}", i + 1, name, server);
  }
  print!("Enter a number: ");
  io::stdout().flush().unwrap();
  let mut line = String::new();
  io::stdin().read_line(&mut line).unwrap();
  let number: usize = match line[..line.len() - 1].parse() {
    Ok(n) => n,
    Err(_) => {
      println!("Please enter a valid number.");
      return;
    }
  };
  let search_character = match characters.results.get(number - 1) {
    Some(c) => c,
    None => {
      println!("There was no character {}.", number);
      return;
    }
  };
  let character = match xivdb.character(search_character["id"].as_u64().unwrap()) {
    Ok(c) => c,
    Err(e) => {
      println!("Error downloading character: {}", e);
      for err in e.iter() {
        println!("{}", err);
      }
      return;
    }
  };
  println!("{:#?}", character);
}
