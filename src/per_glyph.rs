use super::EventTime;

struct PersistentGlyph {
    glyph : char,
    row : usize,
    col : usize,
    start : EventTime,
    end : EventTime
}

struct TerminalScreen {
    glyphs : Vec<Vec<Vec<PersistentGlyph>>>
}

impl TerminalScreen {
    pub fn new(rows : usize, cols : usize) -> TerminalScreen {
        TerminalScreen {
            glyphs : vec![vec![&Vec::new(); cols]; rows]
        }
    }
}
