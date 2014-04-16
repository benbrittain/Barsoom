// Octree Data Structure
extern crate rand;

use rand::{weak_rng, Rng};
use std::num;


#[deriving(Show)]
struct Point {
  x: f32,
  y: f32,
  z: f32
}

impl Point {
  fn new(x: f32, y: f32, z: f32) -> Point {
    Point {
      x: x,
      y: y,
      z: z
    }
  }
}

#[deriving(Show)]
enum Node {
  Leaf,
  INode(~[~Octree])
}


#[deriving(Show)]
pub struct Octree {
  origin: Point, // center of node
  size: f32, // size of node
  data: Option<Point>,
  children: Node
}

impl Octree {
  pub fn new_root() -> Octree {
    Octree{
      origin: Point::new(0.0,0.0,0.0),
      size: 200000000000.0,
      data: None,
      children: Leaf
    }
  }
  pub fn new(pnt: Point, size: f32) -> Octree {
    Octree{
      origin: pnt,
      size: size,
      data: None,
      children: Leaf
    }
  }


  /*   My First attempt at the noble art of ascii
   *   This is the top level of the octree,
   *   the bottom level is the same +4
   *        ^        ___________________
   *        |       /        / |       /|
   *        |      /|  0    /  |   1  / |
   *        |     /_|______/___|_____/  |
   * z-axis |    /   |    / |  |    /|  |
   *        |   /    |   /  |  |   / |  |
   *        |   |----|--+---|--/--/  |  |
   *        |   |    |_ |___|_/___|__|_/     ^
   *        |   |   / 2 |   |/ 3  |  |/     / y-axis
   *        |   |  /    |   /     |  /     /
   *        |   | /     |  /      | /     /
   *        |   |_______|_/_______|/     /
   *        --------------------------> /
   *                x-axis
   *
   */

  fn insert(&mut self, pnt : Point) {
//    println!("Attempting to insert {} into {}", pnt, self.origin);
    match self.children {
      Leaf =>
        match self.data {
          None => self.data = Some(pnt),
          Some(oldPoint) => {
//            println!("Leafnode to be split into octree {}", self.origin);
            let mut childNodes = ~[];
            for i in range(0 as uint, 8) {
              let new_x = self.origin.x + self.size * (if i & 4 == 0 { 0.5 } else {-0.5});
              let new_y = self.origin.y + self.size * (if i & 2 == 0 { 0.5 } else {-0.5});
              let new_z = self.origin.z + self.size * (if i & 1 == 0 { 0.5 } else {-0.5});
              let new_octree = ~Octree::new(Point::new(new_x, new_y, new_z), self.size/2.0);
//              println!("made Leafnode {}", new_octree);
              childNodes.push(new_octree);
            }

//TODO easy optimization, just add from here instead of root
//            childNodes[getOctant(self.origin, pnt)].insert(pnt);
//            childNodes[getOctant(self.origin, pnt)].insert(oldPoint);
            self.children = INode(childNodes);
            self.insert(pnt);
          }
        },
      INode(ref mut children) => {
        let index = getOctant(self.origin, pnt);
        children[index].insert(pnt);//.insert(pnt);
      }
    }
  }
  // TODO write, get points w/ limits - mandatory
  // TODO write? just get point? less efficient for sure though
}

fn getOctant(origin: Point, pnt: Point) -> uint {
  let mut octant = 0;
  if pnt.x <= origin.x {
    octant |= 4;
  }
  if pnt.y <= origin.y {
    octant |= 2;
  }
  if pnt.z <= origin.z {
    octant |= 1;
  }
  octant
}

fn main() {
  sphere_test()
}

fn sphere_test() {
  // yes, I know points are gonna be bunched at the poles
  let mut tree = Octree::new_root();
  let mut rng = rand::weak_rng();
  for i in range (0,8000000) {
    // radius of Mars in meters
//    let r: f32      = rng.gen_range(0.0 as f32, 3389500.0);
    let r: f32      = rng.gen_range(495.0 as f32, 500.0);
    let theta: f32  = rng.gen_range(0.0 as f32, 2.0*3.14159);
    let phi: f32    = rng.gen_range(0.0 as f32, 3.14159);
    let x = r*theta.cos()*phi.sin();
    let y = r*theta.sin()*phi.sin();
    let z = r*phi.cos();
    if (i%100000) == 0 {
      println!("point {}: ({},{},{})",i,x,y,z);
    }
    tree.insert(Point::new(x,y,z));
  }


}

fn point_test() {
  let mut tree = Octree::new_root();
  let mut rng = rand::weak_rng();
  for _ in range(0,10000000){
    let x: f32 = rng.gen_range(0.0 as f32, 144000000000.0);
    let y: f32 = rng.gen_range(0.0 as f32, 144000000000.0);
    let z: f32 = rng.gen_range(0.0 as f32, 144000000000.0);
    tree.insert(Point::new(x,y,z));
  }
}
