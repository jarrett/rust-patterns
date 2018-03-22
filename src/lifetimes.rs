struct Animal {
  name: String
}

impl Animal {
  fn borrow_name<'a>(&'a self) -> &'a String {
    &self.name
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn borrow_struct_field() {
    {
      let animal = Animal {name: String::from("Foo")};
      let _: &String = animal.borrow_name();
    }
  }
}