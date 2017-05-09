extern crate xivdb;

use xivdb::XivDb;

fn main() {
  let xivdb = XivDb::default();

  let result = xivdb.search("Hildibrand", Default::default()).unwrap();
  println!("There are {} Hildbrand NPCs.", result.npcs.unwrap().results.len());
}
