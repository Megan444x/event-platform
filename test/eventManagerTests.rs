use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

mod event_management {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq)]
    pub struct Event {
        pub id: u32,
        pub name: String,
        pub participants: Vec<String>,
    }

    lazy_static! {
        pub static ref EVENTS_DB: Mutex<HashMap<u32, Event>> = Mutex::new(HashMap::new());
    }
    
    pub fn create_event(id: u32, name: &str) -> Result<(), String> {
        let mut db = EVENTS_DB.lock().unwrap();
        if db.contains_key(&id) {
            Err("Event ID already exists".to_string())
        } else {
            db.insert(id, Event { id, name: name.to_string(), participants: vec![] });
            Ok(())
        }
    }

    pub fn register_participant(event_id: u32, participant_name: &str) -> Result<(), String> {
        let mut db = EVENTS_DB.lock().unwrap();
        if let Some(event) = db.get_mut(&event_id) {
            event.participants.push(participant_name.to_string());
            Ok(())
        } else {
            Err("Event not found".to_string())
        }
    }
    
    pub fn update_event_name(event_id: u32, new_name: &str) -> Result<(), String> {
        let mut db = EVENTS_DB.lock().unwrap();
        if let Some(event) = db.get_mut(&event_id) {
            event.name = new_name.to_string();
            Ok(())
        } else {
            Err("Event not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let result = event_management::create_event(1, "Tech Conference");
        assert!(result.is_ok());
    }

    #[test]
    fn test_event_registration() {
        event_management::create_event(2, "Music Festival").unwrap();
        let result = event_management::register_participant(2, "Alice");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_event_update() {
        event_management::create_event(3, "Art Show").unwrap();
        let result = event_management::update_event_name(3, "Annual Art Show");
        assert!(result.is_ok());
    }

    #[test]
    fn test_event_repeated_creation_error() {
        event_management::create_event(4, "Science Fair").unwrap();
        let result = event_management::create_event(4, "Science Fair 2.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_registration_for_nonexistent_event() {
        let result = event_management::register_participant(999, "Bob");
        assert!(result.is_err());
    }
}