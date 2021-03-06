#[cfg(test)]
mod tests {
    use crate::repeat_event::*;
    use crate::event_queue::*;
    use crate::EventTime;

    #[test]
    fn test_queue_basics() {
        let mut events = EventQueue::new();
        events.register_event('A', 1.);
        events.register_event('C', 2.);
        events.register_event('B', 0.);
        assert_eq!(events.next(),  (0., 'B'));
        assert_eq!(events.next(),  (1., 'A'));
        assert_eq!(events.next(),  (2., 'C'));
        
    }    
    
    #[test]
    fn test_invoke_once() {

        let mut events = EventQueue::new();
        let mut change_me : u32 = 0;
        let closure = |state : &mut u32, _, _| {             
            *state = *state + 1;
            RepeatResult::Done
        };
        
        let add_event = ClosureRepeatEvent::new( closure );
                
        events.register_event(add_event, 10.);
        do_next_and_repeat(&mut change_me, &mut events, 11.);
        assert_eq!(change_me, 1);        
    }

    #[test]
    fn test_invoke_twice() {

        let mut events = EventQueue::new();
        let mut change_me : u32 = 0;
        let closure = |state : &mut u32, _, _| {             
            *state = *state + 1;
            if *state == 2 {
                RepeatResult::Done
            } else {
                RepeatResult::RescheduleFor(100.)
            }
        };
        
        let mut add_event = ClosureRepeatEvent::new( closure );
                
        events.register_event(add_event, 10.);
        do_next_and_repeat(&mut change_me, &mut events, 11.);
        assert_eq!(change_me, 1);
        do_next_and_repeat(&mut change_me, &mut events, 11.);
        assert_eq!(change_me, 2);
        assert_eq!(events.is_empty(), true);
        
    }

    #[test]
    fn test_time_values() {

        let mut events = EventQueue::new();
        let mut change_me : EventTime = 0.;
        let closure = |state : &mut EventTime, scheduled : EventTime, actual : EventTime | {             
            *state = actual - scheduled;
            RepeatResult::Done
        };
        
        let mut add_event = ClosureRepeatEvent::new( closure );
                
        events.register_event(add_event, 10.);
        do_next_and_repeat(&mut change_me, &mut events, 11.);
        assert_eq!(change_me, 1.);        
    }



}
