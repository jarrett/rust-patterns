use std::cell::RefCell;
use std::rc::Rc;

use super::{Event, Events};

type EmitterRef<T> = Rc<RefCell<T>>;

// A wrapper around an Event that remembers the emitter, whose type is T.
#[derive(Debug)]
struct PackagedEvent<T: 'static> {
  event: Box<Event>,
  emitter: EmitterRef<T>
}

impl<T> PackagedEvent<T> {
  fn cast<U, V, F>(&self, f: F) -> Option<V>
    where U: 'static, F: FnOnce(&U) -> V
  {
    self.event.as_any().downcast_ref::<U>().map(f)
  }

  // In a real program, it would be better to put this method on Events.
  fn package(events: &mut Events, emitter: &EmitterRef<T>) -> Vec<PackagedEvent<T>> {
    events.queue.drain(..).map(|event: Box<Event>|
      PackagedEvent {event: event, emitter: emitter.clone()}
    ).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::super::*;
  use super::*;

  #[test]
  fn example() {
    let bar = Rc::new(RefCell::new(
      Bar {events: Events::new()}
    ));

    bar.borrow_mut().events.emit(FooEvent::new("Quux"));

    let packaged: Vec<PackagedEvent<Bar>> = PackagedEvent::package(
      &mut bar.borrow_mut().events, &bar
    );

    // Pretend we're a listener.
    for packaged_event in packaged {
      assert_eq!(
        bar.as_ref() as *const _,
        packaged_event.emitter.as_ref() as *const _
      );
      let message: Option<String> = packaged_event.cast(|e: &FooEvent|
        e.message.clone()
      );
      assert_eq!(Some(String::from("Quux")), message);
    }
  }
}