extern crate rand;

use std::error::Error;

use buffalo_art::EventTime;
use buffalo_art::per_glyph::*;

use std::{thread, time};

fn pglyph(glyph : char, col : usize, start : EventTime, end : EventTime, term : &mut TerminalScreen) {
    let g = PersistentGlyph {
        glyph : glyph,
        row : 0,
        col : col,
        start : start,
        end : end
    };
    term.add_glyph(g);
    
}

fn main() -> Result<(), Box<dyn Error>>{

    let mut screen = TerminalScreen::new(10,10);
    pglyph('a', 0, 0.2, 1.0, &mut screen);
    pglyph('b', 1, 1., 2.0, &mut screen);
    pglyph('c', 2, 2., 3.0, &mut screen);
    pglyph('X', 0, 0., 3.0, &mut screen);
    pglyph('Y', 0, 0.5, 2.0, &mut screen);


    
    let mut time = 0.;
    let tenth = time::Duration::from_millis(100);

    loop {

        let draw_result = screen.draw_next(time) ?;
        match draw_result {
            DrawResult::Done => {
                //println!("done!");
                break }
            DrawResult::Continuing => { continue }
            DrawResult::Waiting =>
            {
                //println!("sleeping!");
                thread::sleep(tenth);
                time += 0.1;

            }
        }
        

    }
    
    Ok(())
}
