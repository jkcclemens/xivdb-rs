extern crate xivdb;

use xivdb::XivDb;

fn main() {
  let xivdb = XivDb::default();

  let result = xivdb.search("Hildibrand", Default::default()).unwrap();

  let npcs = result.npcs.unwrap();
  println!("There are {} Hildbrand NPCs on page {}.",
    npcs.results.len(),
    npcs.paging.page);
}
