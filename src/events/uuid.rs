// Instead of pointers, UUIDs might be a better for events to identify
// the objects that emitted them. This strategy is probably more
// networking-friendly.

// However, it comes at the expense of type safety. We know at compile time
// that an Event<foo> has a valid reference to a Foo. But for an Event with
// nothing more than a UUID, we can't check this property until runtime.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use uuid::Uuid;
use super::{Event, Events};

#[derive(Debug)]
struct EntityRef<T: 'static> {
  uuid: Uuid,
  phantom: PhantomData<T>
}

// A wrapper around an Event that remembers the emitter, whose type is T.
#[derive(Debug)]
struct PackagedEvent<T: 'static> {
  event: Box<Event>,
  emitter: EntityRef<T>
}

// Example of an emitter.
#[derive(Debug)]
struct Bar {
  uuid: Uuid,
  events: Events
}

impl<T> EntityRef<T> {
  // We can't derive `Clone` without the trait bound `T: Clone`, which
  // we don't necessarily want for all emitters. See:
  // https://github.com/rust-lang/rust/issues/26925
  fn clone(&self) -> Self {
    EntityRef::new(self.uuid)
  }
  
  fn new(uuid: Uuid) -> Self {
    EntityRef  {uuid: uuid, phantom: PhantomData}
  }

  // Unclear whether this should be a method on the registry or
  // the `EntityRef`. Maybe both?
  fn resolve<'a>(
    &self, registry: &'a HashMap<Uuid, Box<Any>>
  ) -> &'a Rc<RefCell<T>> {
    registry.get(&self.uuid).unwrap().downcast_ref::<Rc<RefCell<T>>>().unwrap()
  }
}

impl<T> PackagedEvent<T> {
  fn cast<U, V, F>(&self, f: F) -> Option<V>
    where U: 'static, F: FnOnce(&U) -> V
  {
    self.event.as_any().downcast_ref::<U>().map(f)
  }

  // In a real program, it would be better to put this method on Events.
  fn package(events: &mut Events, emitter: &EntityRef<T>) -> Vec<PackagedEvent<T>> {
    events.queue.drain(..).map(|event: Box<Event>|
      PackagedEvent {event: event, emitter: emitter.clone()}
    ).collect()
  }
}

impl Bar {
  fn new() -> Bar {
    Bar {uuid: Uuid::new_v4(), events: Events::new()}
  }
}

#[cfg(test)]
mod tests {
  use super::super::*;
  use super::*;

  #[test]
  fn example() {
    let mut registry: HashMap<Uuid, Box<Any>> = HashMap::new();
    let bar = Rc::new(RefCell::new(Bar::new()));
    let eref = EntityRef::new(bar.borrow().uuid);
    registry.insert(eref.uuid, Box::new(bar.clone()));

    bar.borrow_mut().events.emit(FooEvent::new("Quux"));

    let packaged: Vec<PackagedEvent<Bar>> = PackagedEvent::package(
      &mut bar.borrow_mut().events, &eref
    );
    
    // Pretend we're a listener.
    for packaged_event in packaged {
      assert_eq!(
        bar.as_ref() as *const _,
        packaged_event.emitter.resolve(&registry).as_ref() as *const _
      );
      let message: Option<String> = packaged_event.cast(|e: &FooEvent|
        e.message.clone()
      );
      assert_eq!(Some(String::from("Quux")), message);
    }
  }
}