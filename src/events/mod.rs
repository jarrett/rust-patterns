use std::any::Any;
use std::fmt::Debug;

mod pointer;
mod uuid;

trait AsAny : Any {
  fn as_any(&self) -> &Any;
}

trait Event : Debug + AsAny {}

// An event manager. Should be owned by an emitter or listener.
#[derive(Debug)]
struct Events {
  queue: Vec<Box<Event>>
}

#[derive(Debug)]
struct FooEvent {
  message: String
}

impl AsAny for FooEvent {
  fn as_any(&self) -> &Any { self }
}

#[derive(Debug)]
struct Bar {
  events: Events
}

impl Events {
  fn emit<U: 'static + Event>(&mut self, event: U) {
    self.queue.push(Box::new(event));
  }

  fn new() -> Events {
    Events {
      queue: Vec::new()
    }
  }
}

impl FooEvent {
  fn new(message: &str) -> FooEvent {
    FooEvent {
      message: String::from(message)
    }
  }
}

impl Event for FooEvent {}