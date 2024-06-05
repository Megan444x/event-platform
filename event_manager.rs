use std::collections::HashMap;
use std::env;

#[derive(Clone, Debug)]
struct Event {
    id: u32,
    name: String,
    attendees: Vec<String>,
}

struct EventManager {
    events: HashMap<u32, Event>,
}

impl EventManager {
    fn new() -> EventManager {
        EventManager {
            events: HashMap::new(),
        }
    }

    fn create_event(&mut self, id: u32, name: &str) {
        let new_event = Event {
            id,
            name: name.to_string(),
            attendees: Vec::new(),
        };
        self.events.insert(id, new_event);
    }

    fn get_event(&self, id: u32) -> Option<&Event> {
        self.events.get(&id)
    }

    fn update_event(&mut self, id: u32, new_name: &str) {
        if let Some(event) = self.events.get_mut(&id) {
            event.name = new_name.to_string();
        }
    }

    fn delete_event(&mut self, id: u32) {
        self.events.remove(&id);
    }

    fn register_to_event(&mut self, event_id: u32, attendee: &str) {
        if let Some(event) = self.events.get_mut(&event_id) {
            event.attendees.push(attendee.to_string());
        }
    }

    fn list_events(&self) -> Vec<&Event> {
        self.events.values().collect()
    }
}

fn main() {
    dotenv::dotenv().ok();

    let mut event_manager = EventManager::new();
    event_manager.create_event(1, "Rust Conference");

    event_manager.register_to_event(1, "Alice");
    event_manager.register_to_event(1, "Bob");

    match event_manager.get_event(1) {
        Some(event) => println!("Event found: {:?}", event),
        None => println!("Event not found"),
    };

    event_manager.update_event(1, "RustConf 2023");

    event_manager.delete_event(1);
}