use std::collections::{HashMap, HashSet};
use std::env;

#[derive(Clone, Debug)]
struct Event {
    id: u32,
    name: String,
    attendees: Vec<String>,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Event ID: {}, Name: {}, Attendees: {:?}", self.id, self.name, self.attendees)
    }
}

struct EventManager {
    events: HashMap<u32, Event>,
    name_to_id: HashMap<String, HashSet<u32>>,
}

impl EventManager {
    fn new() -> EventManager {
        EventManager {
            events: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }

    fn create_event(&mut self, id: u32, name: &str) {
        let new_event = Event {
            id,
            name: name.to_string(),
            attendees: Vec::new(),
        };
        self.name_to_id.entry(name.to_string()).or_insert_with(HashSet::new).insert(id);
        self.events.insert(id, new_event);
    }

    fn get_event(&self, id: u32) -> Option<&Event> {
        self.events.get(&id)
    }

    fn update_event(&mut self, id: u32, new_name: &str) {
        if let Some(event) = self.events.get_mut(&id) {
            let old_name = std::mem::replace(&mut event.name, new_name.to_string());
            self.name_to_id.entry(old_name).and_modify(|e| { e.remove(&id); });
            self.name_to_id.entry(new_name.to_string()).or_insert_with(HashSet::new).insert(id);
        }
    }

    fn delete_event(&mut self, id: u32) {
        if let Some(event) = self.events.remove(&id) {
            self.name_to_id.entry(event.name).and_modify(|e| { e.remove(&id); });
        }
    }

    fn register_to_event(&mut self, event_id: u32, attendee: &str) {
        if let Some(event) = self.events.get_mut(&event_id) {
            event.attendees.push(attendee.to_string());
        }
    }

    fn list_events(&self) -> Vec<&Event> {
        self.events.values().collect()
    }

    fn list_attendees_for_event(&self, event_id: u32) -> Option<Vec<String>> {
        self.events.get(&event_id).map(|event| event.attendees.clone())
    }

    fn find_events_by_name(&self, name: &str) -> Vec<&Event> {
        self.name_to_id.get(name).map_or_else(Vec::new, |ids| {
            ids.iter().filter_map(|id| self.events.get(id)).collect()
        })
    }
}

fn main() {
    dotenv::dotenv().ok();

    let mut event_manager = EventManager::new();
    event_manager.create_event(1, "Rust Conference");
    event_manager.create_event(2, "Rust Conference");

    event_manager.register_to_event(1, "Alice");
    event_manager.register_to_event(1, "Bob");

    match event_manager.get_event(1) {
        Some(event) => println!("Event found before delete: {}", event),
        None => println!("Event not found"),
    };

    event_manager.update_event(1, "RustConf 2023");

    for event in event_manager.find_events_by_name("Rust Conference") {
        println!("Found event by name: {}", event);
    }

    for event in event_manager.find_events_by_name("RustConf 2023") {
        println!("Found updated event by name: {}", event);
    }

    match event_manager.list_attendees_for_event(1) {
        Some(attendees) => println!("Attendees: {:?}", attendees),
        None => println!("No attendees or event not found"),
    }

    event_manager.delete_event(1);
    
    match event_manager.get_event(1) {
        Some(_) => println!("Error: Event was not properly deleted."),
        None => println!("Event successfully deleted."),
    };
}