
use super::event_queue::*;

pub enum RepeatResult {
    Done,
    RescheduleFor(EventTime)
}

pub trait RepeatEvent<T> {
    fn do_event(&mut self, state : &mut T, scheduled: EventTime, actual: EventTime) -> RepeatResult;
}


pub fn do_next_and_repeat<S, EQEntry:RepeatEvent<S>>(state : &mut S, queue : &mut EventQueue<EQEntry>, actual: EventTime) {
    let (scheduled, mut event) = queue.next();
    match event.do_event(state, scheduled, actual) {
        RepeatResult::Done => {},
        RepeatResult::RescheduleFor(new_time) => {
            queue.register_event(event, new_time)
        }
    };
    
            
        
}


