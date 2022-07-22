use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::EventTime;

struct EventEntry<T> {
    event_time : EventTime,
    event : T,
}

impl<T> Ord for EventEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.event_time.partial_cmp(&self.event_time).unwrap()
    }
}

impl<T> PartialOrd for EventEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for EventEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.event_time == other.event_time
    }
}

impl<T> Eq for EventEntry<T> {}

pub struct EventQueue<T> {
    queue : BinaryHeap<EventEntry<T>>,
}

impl<T> EventQueue<T> {
    pub fn register_event(&mut self, event:T, event_time:EventTime) {
        self.queue.push(EventEntry { event_time : event_time, event : event } ) 
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    // "waiting" i.e. has no happened events
    pub fn is_waiting(&self, now : EventTime) -> bool {
        self.queue.peek().unwrap().event_time >= now
    }

    pub fn next(&mut self) -> (EventTime
                               , T) {
        let result = self.queue.pop().unwrap();
        (result.event_time, result.event)
    }

    pub fn new() -> EventQueue<T> {
        EventQueue { queue : BinaryHeap::new() }
    }
}



