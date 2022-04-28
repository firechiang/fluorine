pub mod listener;

//use std::cmp::Ordering;
use std::collections::HashMap;

/**
 * event
 */
pub trait EventObject<T> {

    // Returns a unique value
    fn name(&self) -> &str;

    fn body(&self) -> T;
}

pub struct EventUser {

}

/**
 * listener
 */
pub trait EventListener {

    fn on_event(&self,event: Box<dyn EventObject<EventUser>>);
}

/**
 * equals
 */
// impl Eq for Box<dyn EventListener> {}
//
// impl PartialEq<Self> for Box<dyn EventListener> {
//     fn eq(&self, other: &Self) -> bool {
//         self.event_name() == other.event_name()
//     }
// }
//
// impl PartialOrd<Self> for Box<dyn EventListener> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(&other))
//     }
// }
//
// impl Ord for Box<dyn EventListener> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.event_name().cmp(&other.event_name())
//     }
// }


/**
 * context
 */
pub struct EventContext {

    listeners: HashMap<&'static str,Box<dyn EventListener>>,
}

impl EventContext {

    pub fn new() -> EventContext {
        let listeners:HashMap<&'static str,Box<dyn EventListener>> = HashMap::new();
        EventContext {
            listeners
        }
    }

    pub fn add_listener(&mut self, event_name: &'static str, listener: Box<dyn EventListener>) -> Option<Box<dyn EventListener>> {
        self.listeners.insert(event_name,listener)
    }

    pub fn event(&self,event: Box<dyn EventObject<EventUser>>) {
        let obj = self.listeners.get(event.name()).unwrap();
        obj.on_event(event);
        //EventListener::on_event(&**obj, event);
    }
}