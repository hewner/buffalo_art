use super::EventTime;
use super::event_queue::*;
use std::error::Error;
use console::Term;

pub struct PersistentGlyph {
    pub glyph : char,
    pub row : usize,
    pub col : usize,
    pub start : EventTime,
    pub end : EventTime
}

enum GlyphCommand {
    Draw(PersistentGlyph),
    Clean(usize, usize)
}

// TODO this should probably be in its own file
pub struct TerminalScreen {
    drawn_glyphs : Vec<Vec<Vec<PersistentGlyph>>>,
    future_glyphs : EventQueue<GlyphCommand>,
    term : Term
}

pub enum DrawResult {
    Continuing,
    Waiting,
    Done
}

impl TerminalScreen {
    pub fn new(rows : usize, cols : usize) -> TerminalScreen {

        let mut row_list = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut col_list = Vec::with_capacity(cols);
            for _ in 0..cols {
                col_list.push(Vec::new());
            }
            row_list.push(col_list);
        }
        
        TerminalScreen {
            drawn_glyphs : row_list,
            future_glyphs : EventQueue::new(),
            term : Term::stdout()
        }
    }

    pub fn add_glyph(&mut self, glyph : PersistentGlyph) {
        //self.glyphs[glyph.row][glyph.col].push(glyph);
        let start = glyph.start;
        self.future_glyphs.register_event(GlyphCommand::Draw(glyph), start);
    }

    fn draw_glyph(&mut self, glyph: char, row: usize, col: usize) -> Result<(),Box<dyn Error>> {

        use std::io::Write;

        //println!("drawing {} at ({},{})", glyph, row, col);
        
        self.term.move_cursor_to(col, row)?;
        let mut b = [0; 4];


        let result = glyph.encode_utf8(&mut b);
        self.term.write(result.as_bytes()) ?;
        Ok(())
    }

    fn clean_glyph(&mut self, row: usize, col: usize, now : EventTime) -> char {
        let mut vec = &mut self.drawn_glyphs[row][col];
        for i in (0..vec.len()).rev() {
            if vec[i].end < now { vec.remove(i); }
        };
        if vec.is_empty() { ' ' } else { vec.last().unwrap().glyph } 
        
    }
    
    pub fn draw_next(&mut self, now : EventTime) -> Result<DrawResult,Box<dyn Error>> {

        if self.future_glyphs.is_empty() { return Ok(DrawResult::Done) } ;
        if self.future_glyphs.is_waiting(now) { return Ok(DrawResult::Waiting) };
        match self.future_glyphs.next() {
            (_, GlyphCommand::Draw(glyph)) => {
                self.draw_glyph(glyph.glyph, glyph.row, glyph.col) ?;
                self.future_glyphs.register_event(GlyphCommand::Clean(glyph.row, glyph.col), glyph.end);
                self.drawn_glyphs[glyph.row][glyph.col].push(glyph);
            }
            (_, GlyphCommand::Clean(row, col)) => {
                let new_char = self.clean_glyph(row, col, now);
                self.draw_glyph(new_char, row, col) ?;
            }
        }
        Ok(DrawResult::Continuing)
        
    }

}
