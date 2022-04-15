
use std::marker;
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

pub struct ClosureRepeatEvent<S, F>
    where F: Fn(&mut S, EventTime, EventTime) -> RepeatResult
{

    _state : marker::PhantomData<S>, 
    closure: F
        
}

impl<S, F> ClosureRepeatEvent<S,F> where F: Fn(&mut S, EventTime, EventTime) -> RepeatResult {
    pub fn new(f : F) -> ClosureRepeatEvent<S, F> {
        ClosureRepeatEvent { _state : std::marker::PhantomData, closure : f }
    }
}

impl<S, F> RepeatEvent<S> for ClosureRepeatEvent<S,F> where F: Fn(&mut S, EventTime, EventTime) -> RepeatResult {

    fn do_event(&mut self,
                state : &mut S,
                scheduled : EventTime,
                actual : EventTime) -> RepeatResult {
        (self.closure)(state, scheduled, actual)
    }

}


