use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct World {
  // This demonstrates several strategies for having related objects
  // mutate each other. See:
  // https://doc.rust-lang.org/beta/nomicon/borrow-splitting.html

  // Given a mutable reference to a parent object (world), we can simultaneously
  // borrow mutable references to its fields (friend and enemy).
  pub friend: Person,
  pub enemy: Person,

  // But this doesn't work with some structs, e.g. Vec. We can't simultaneously
  // borrow mutable references to multiple elements of a Vec. This is just a
  // limitation of the borrow checker--it understands struct fields but not
  // more complex data structures like Vecs. One solution is RefCell.
  pub frenemies: Vec<RefCell<Person>>,

  // When objects keep references to each other, we can use Rc<RefCell>.
  pub siblings: Vec<Rc<RefCell<Person>>>,
}

struct Person {
  health: u8,
  armor: u8,
  strength: u8,
  sibling: Option<Rc<RefCell<Person>>>
}

impl Person {
  pub fn attack(&self, other: &mut Person) {
    other.receive_attack(self.strength);
  }

  pub fn attack_sibling(&self) -> Result<(), ()> {
    if let Some(ref sibling) = self.sibling {
      self.attack(&mut sibling.borrow_mut());
      Ok(())
    } else {
      Err(())
    }
  }

  pub fn health(&self) -> u8 { self.health }

  pub fn new(armor: u8, strength: u8) -> Person {
    Person {health: 10, armor: armor, strength: strength, sibling: None}
  }

  pub fn receive_attack(&mut self, strength: u8) {
    self.health -= max(0, strength as i16 - self.armor as i16) as u8;
  }

  pub fn set_sibling(&mut self, sibling: &Rc<RefCell<Person>>) {
    self.sibling = Some(sibling.clone());
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::{Ref, RefMut};

  #[test]
  fn example() {
    let mut world = World {
      friend: Person::new(0, 6),
      enemy: Person::new(4, 1),
      frenemies: vec![
        RefCell::new(Person::new(0, 7)),
        RefCell::new(Person::new(3, 0))
      ],
      siblings: vec![
        Rc::new(RefCell::new(Person::new(0, 10))),
        Rc::new(RefCell::new(Person::new(7, 0))),
      ]
    };

    world.siblings[0].borrow_mut().set_sibling(&world.siblings[1]);
    world.siblings[1].borrow_mut().set_sibling(&world.siblings[0]);

    /*{
      let _friend = &mut world.friend;
      let _enemy = &mut world.enemy;

      let _frenemy_0 = world.frenemies[0].borrow_mut();
      let _frenemy_1 = world.frenemies[1].borrow_mut();

      let _sibling_0 = world.siblings[0].borrow_mut();
      let _sibling_1 = world.siblings[1].borrow_mut();
    }*/

    assert_eq!(10, world.enemy.health());
    world.friend.attack(&mut world.enemy);
    assert_eq!(8, world.enemy.health());

    assert_eq!(10, world.frenemies[1].borrow().health());
    let frenemy_0: Ref<Person> = world.frenemies[0].borrow();
    let mut frenemy_1: RefMut<Person> = world.frenemies[1].borrow_mut();
    // RefMut<Person> implements DerefMut. Thus, it will be coerced to &mut Person.
    frenemy_0.attack(&mut frenemy_1);
    assert_eq!(6, frenemy_1.health());

    assert_eq!(10, world.siblings[1].borrow().health());
    let sibling_0 = world.siblings[0].borrow();
    sibling_0.attack_sibling().unwrap();
    assert_eq!(7, world.siblings[1].borrow().health());
  }
}