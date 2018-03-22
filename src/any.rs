use std::any::Any;

// This trait is needed because you can't upcast from a `Foo` trait object
// to an `Any` trait object even though `Foo : Any`. (Which in turn flows)
// from how Rust builds vtables for trait objects.) More info:
// https://users.rust-lang.org/t/upcasting-support-for-arbitrary-types-implementing-a-trait-any-vs-mopa/3897/13
trait AsAny : Any {
  fn as_any(&self) -> &Any;
}

trait Foo : AsAny {}

struct Bar;

impl Foo for Bar {}

impl AsAny for Bar {
  fn as_any(&self) -> &Any { self }
}

#[cfg(test)]
mod tests {
  use std::boxed::Box;
  use super::*;

  #[test]
  fn example() {
    {
      let bar = Bar;
      let any = &bar as &Any;
      any.downcast_ref::<Bar>().unwrap();
    }

    {
      let bar = Bar;
      let any: Box<Any> = Box::new(bar);
      any.downcast::<Bar>().unwrap();
    }

    {
      let bar = Bar;
      let foo: &Foo = &bar as &Foo;
      let any: &Any = foo.as_any();
      any.downcast_ref::<Bar>().unwrap();
    }

    {
      let bar = Bar;
      let foo: Box<Foo> = Box::new(bar);
      let any: &Any = foo.as_any();
      any.downcast_ref::<Bar>().unwrap();
    }

    // The following examples won't compile.
    //
    //`downcast_ref` is defined in `impl Any`
    // rather than `trait Any`, so it's defined only on the trait object,
    // not on the trait's implementors.
    // {
    //   let bar = Bar;
    //   bar.downcast_ref::<Bar>().unwrap();
    // }
    //
    // As noted above, we can't upcast from a `Foo` trait object to
    // an `Any` trait object.
    // {
    //   let bar = Bar;
    //   let foo: &Foo = &bar as &Foo;
    //   let any: &Any = foo as &Any;
    //   any.downcast_ref::<Bar>().unwrap();
    // }
  }
}