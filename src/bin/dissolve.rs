extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::error::Error;
use std::io::Write;
use console::Term;



fn main() -> Result<(), Box<dyn Error>>{

    let mut rng = thread_rng();

    
    let mut term = Term::stdout();
    let (rows, cols) = term.size();
    println!("{} {}", rows, cols);

    loop {
        let r: u16 = rng.gen_range(0..rows);
        let c: u16 = rng.gen_range(0..cols);

        term.move_cursor_to(c.into(),r.into())?;
        term.write(b"?")?;
        
        // println!("{} {}", r, c);
    }
    
    Ok(())
}
