use super::EventTime;

pub struct PersistentGlyph {
    glyph : char,
    row : usize,
    col : usize,
    start : EventTime,
    end : EventTime
}

pub struct TerminalScreen {
    glyphs : Vec<Vec<Vec<PersistentGlyph>>>
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
            glyphs : row_list
        }
    }

    pub fn add_glyph(&mut self, glyph : PersistentGlyph) {
        self.glyphs[glyph.row][glyph.col].push(glyph);
    }

}
