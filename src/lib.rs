use std::{any::Any};

struct EventWriter {
    events: Vec<Box<dyn Any>>
}

impl EventWriter {
    fn write<T: Event>(&mut self, new_event: T) {
        self.events.push(Box::new(new_event));
    }
}

trait Event: Any { }

trait EventHandler<T: Event> {
    fn read(&mut self, events_writer: &mut EventWriter) {
        for event in events_writer.events.iter_mut() {
            if let Some(event) = event.downcast_mut::<T>() {
                self.on_event(event);
            }
        }
    }

    fn on_event(&mut self, event: &T);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Head {
        sub: Test,
        events: EventWriter
    }

    impl Head {
        fn run(&mut self) {
            self.sub.run(&mut self.events)
        }
    }

    struct Test {
        values: i32,
        sub: SubTest
    }

    impl Test {
        fn run(&mut self, events: &mut EventWriter) {
            self.read(events)
        }
    }

    impl EventHandler<TestEvent> for Test { 
        fn on_event(&mut self, event: &TestEvent) {
            self.values += event.0
        }
    }

    struct SubTest {

    }

    struct TestEvent(i32);

    impl Event for TestEvent { }

    #[test]
    fn it_works() {


        let mut test = Head {
            events: EventWriter { events: Vec::new() },
            sub: Test {
                values: 0,
                sub: SubTest { }
            }
        };

        test.events.write(TestEvent(10));
        test.run();

        assert_eq!(test.sub.values, 10);
    }
}
