// Instead of pointers, UUIDs might be a better for events to identify
// the objects that emitted them. This strategy is probably more
// networking-friendly.

// However, it comes at the expense of type safety. We know at compile time
// that an Event<foo> has a valid reference to a Foo. But for an Event with
// nothing more than a UUID, we can't check this property until runtime.

#[cfg(test)]
mod tests {
  //use super::*;

  #[test]
  fn example() {

  }
}