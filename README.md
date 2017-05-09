# xivdb-rs

Rust bindings and implementations for XIVDB's API.

It's not quite complete.

## Examples

```rust
extern crate xivdb;

use xivdb::XivDb;

fn main() {
  let xivdb = XivDb::default();

  let result = xivdb.search("Hildibrand", Default::default()).unwrap();
  // There are 60 Hildbrand NPCs.
  println!("There are {} Hildbrand NPCs.", result.npcs.unwrap().results.len());
}
```
