use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::fmt;

mod event_management {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Event {
        pub id: u32,
        pub name: String,
        pub participants: Vec<String>,
    }

    #[derive(Debug)]
    pub enum EventManagementError {
        DuplicateEvent,
        EventNotFound,
        DatabaseError,
    }

    impl fmt::Display for EventManagementError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                EventManagementError::DuplicateEvent => write!(f, "Event ID already exists"),
                EventManagementError::EventNotFound => write!(f, "Event not found"),
                EventManagementError::DatabaseError => write!(f, "An internal database error occurred"),
            }
        }
    }

    lazy_static! {
        pub static ref EVENT_DATABASE: Mutex<HashMap<u32, Event>> = Mutex::new(HashMap::new());
    }

    pub fn add_event(id: u32, name: &str) -> Result<(), EventManagementError> {
        let mut events = EVENT_DATABASE.lock().map_err(|_| EventManagementError::DatabaseError)?;
        if events.contains_key(&id) {
            Err(EventManagementError::DuplicateEvent)
        } else {
            events.insert(id, Event { id, name: name.to_string(), participants: vec![] });
            Ok(())
        }
    }

    pub fn add_participant_to_event(event_id: u32, participant_name: &str) -> Result<(), EventManagementError> {
        let mut events = EVENT_DATABASE.lock().map_err(|_| EventManagementError::DatabaseError)?;
        if let Some(event) = events.get_mut(&event_id) {
            event.participants.push(participant_name.to_string());
            Ok(())
        } else {
            Err(EventManagementError::EventNotFound)
        }
    }

    pub fn change_event_name(event_id: u32, new_name: &str) -> Result<(), EventManagement--Error> {
        let mut events = EVENT_DATABASE.lock().map_err(|_| EventManagementError::DatabaseError)?;
        if let Some(event) = events.get_mut(&event_id) {
            event.name = new_name.to_string();
            Ok(())
        } else {
            Err(EventManagementError::EventNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use event_management::EventManagementError;

    #[test]
    fn test_add_event() {
        let result = event_management::add_event(1, "Tech Conference");
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_participant_to_event() {
        event_management::add_event(2, "Music Festival").unwrap();
        let result = event_management::add_participant_to_event(2, "Alice");
        assert!(result.is_ok());
    }

    #[test]
    fn test_change_event_name_success() {
        event_management::add_event(3, "Art Show").unwrap();
        let result = event_management::change_event_name(3, "Annual Art Show");
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_event_creation_error() {
        event_management::add_event(4, "Science Fair").unwrap();
        let result = event_management::add_event(4, "Science Fair 2.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_participant_to_nonexistent_event_error() {
        let result = event_management::add_participant_to_event(999, "Bob");
        assert!(result.is_err());
    }
}