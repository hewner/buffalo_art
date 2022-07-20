extern crate rand;

use std::error::Error;
use std::io::Write;
use console::Term;

use buffalo_art::EventTime;
use buffalo_art::repeat_event::*;
use buffalo_art::event_queue::*;

use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>>{

    let mut events = EventQueue::new();

    let mut term = Term::stdout();
    let (rows, cols) = term.size();
    println!("{} {}", rows, cols);

    let closure = |term : &mut console::Term, scheduled : EventTime , _| {

        let new_pos = (scheduled * 10.) as u16;
        
        if new_pos >= cols {
            return RepeatResult::Done;
        }

        term.move_cursor_to(new_pos.into(), 0).unwrap();
        term.write(b"?").unwrap();
        RepeatResult::RescheduleFor(scheduled + 0.1)

    };


    let line = ClosureRepeatEvent::new( closure );
    
    events.register_event(line, 0.);

    let mut time = 0.;

    let tenth = time::Duration::from_millis(100);

    loop {

        if events.is_empty() { break; } 

        do_next_and_repeat(&mut term, &mut events, time);
        
        thread::sleep(tenth);
        time += 0.1;
        

        
        // println!("{} {}", r, c);
    }
    
    Ok(())
}
