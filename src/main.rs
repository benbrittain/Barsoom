// Main.rs
extern crate rustuv;
extern crate green;
extern crate rand;

use octree::{Octree};
mod octree;

mod world_engine;

fn main() {
  println!("Welcome to Barsoom");
  let tree = Octree::new_root();
  world_engine::init(&tree);

}

#[start]
fn start(argc: int, argv: **u8) -> int {
  green::start(argc, argv, rustuv::event_loop, main)
}
