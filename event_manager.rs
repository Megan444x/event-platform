use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
enum EventError {
    EventNotFound,
    DuplicateEvent,
    AttendeeError(String),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventError::EventNotFound => write!(f, "Event not found."),
            EventError::DuplicateEvent => write!(f, "Event with the same ID already exists."),
            EventError::AttendeeError(msg) => write!(f, "Error managing attendee: {}", msg),
        }
    }
}

#[derive(Clone, Debug)]
struct Event {
    id: u32,
    name: String,
    attendees: Vec<String>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    fn create_event(&mut self, id: u32, name: &str) -> Result<(), EventError> {
        if self.events.contains_key(&id) {
            return Err(EventError::DuplicateEvent);
        }
        let new_event = Event {
            id,
            name: name.to_string(),
            attendees: Vec::new(),
        };
        self.name_to_id.entry(name.to_string()).or_insert_with(HashSet::new).insert(id);
        self.events.insert(id, new_event);
        Ok(())
    }

    fn get_event(&self, id: u32) -> Result<&Event, EventError> {
        self.events.get(&id).ok_or(EventError::EventNotFound)
    }

    fn update_event(&mut self, id: u32, new_name: &str) -> Result<(), EventError> {
        match self.events.get_mut(&id) {
            Some(event) => {
                let old_name = std::mem::replace(&mut event.name, new_name.to_string());
                self.name_to_id.entry(old_name).and_modify(|e| { e.remove(&id); });
                self.name_to_id.entry(new_name.to_string()).or_insert_with(HashSet::new).insert(id);
                Ok(())
            },
            None => Err(EventError::EventNotFound),
        }
    }

    fn delete_event(&mut self, id: u32) -> Result<(), EventError> {
        match self.events.remove(&id) {
            Some(event) => {
                self.name_to_id.entry(event.name).and_modify(|e| { e.remove(&id); });
                Ok(())
            },
            None => Err(EventError::EventNotFound),
        }
    }

    fn register_to_event(&mut self, event_id: u32, attendee: &str) -> Result<(), EventError> {
        match self.events.get_mut(&event_id) {
            Some(event) => {
                if !event.attendes.contains(&attendee.to_string()) {
                    event.attendees.push(attendee.to_string());
                    Ok(())
                } else {
                    Err(EventError::AttendeeError("Attendee already registered.".to_owned()))
                }
            },
            None => Err(EventError::EventNotFound),
        }
    }

    fn list_events(&self) -> Vec<&Event> {
        self.events.values().collect()
    }

    fn list_attendees_for_event(&self, event_id: u32) -> Result<Vec<String>, EventError> {
        self.events.get(&event_id).map(|event| event.attendees.clone()).ok_or(EventError::EventNotFound)
    }

    fn find_events_by_name(&self, name: &str) -> Vec<&Event> {
        self.name_to_id.get(name).map_or_else(Vec::new, |ids| {
            ids.iter().filter_map(|id| self.events.get(id)).collect()
        })
    }
}

fn main() {
    let mut event_manager = EventManager::new();

    // Handle creation errors
    if let Err(e) = event_manager.create_event(1, "Rust Conference") {
        println!("Error creating event: {}", e);
    }

    // Similarly, handle other operations with potential for errors
    match event_num {
        Some(event) => println!("Event found: {}", event),
        None => println!("Event not found."),
    }
}