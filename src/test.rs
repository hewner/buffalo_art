pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::repeat_event::*;
    use crate::event_queue::*;

    struct Adder {
        pub val : u32
    }

    struct AdderEvent {
        val_to_add : u32,
        times : u32
            
    }

    impl RepeatEvent<Adder> for AdderEvent {
        fn do_event(&mut self, state : &mut Adder, _ : EventTime, _ : EventTime) -> RepeatResult {
            state.val = state.val + self.val_to_add;
            RepeatResult::Done
        }
    }
    
    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        //assert_eq!(bad_add(1, 2), 3);

        let mut events = EventQueue::new();
        let mut adder  = Adder { val : 0 };
        let mut add_event = AdderEvent { val_to_add : 3, times : 1 };
        events.register_event(add_event, 10.);
        do_next_and_repeat(&mut adder, &mut events, 11.);
        assert_eq!(adder.val, 3);
        
    }
}
