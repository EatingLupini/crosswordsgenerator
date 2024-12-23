
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Dir {
    HOR,
    VER,
}

#[derive(Debug)]
pub struct Solution {
    pub found: bool,
    pub time_elapsed: u128,
    pub visited_nodes: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WordPos {
    pub x: usize,
    pub y: usize,
    pub dir: Dir,
    pub len: usize,
}

impl WordPos {
    pub fn new(x: usize, y: usize, dir: Dir, len: usize) -> WordPos {
        WordPos {
            x,
            y,
            dir,
            len,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    arr: Vec<char>,
}

impl Board {
    pub fn new(w: usize, h: usize) -> Board {
        Board {
            width: w,
            height: h,
            arr: vec![' '; w * h],
        }
    }

    // Get char at given coordinate
    pub fn get(&self, x: usize, y: usize) -> char {
        *self.arr.get(self.width * y + x).unwrap()
    }

    // Get word at given coordinate with dir and len
    pub fn get_word(&self, word_pos: &WordPos) -> String {
        let mut word = String::new();
        match word_pos.dir {
            Dir::HOR => {
                for i in 0..word_pos.len {
                    word.push(*self.arr.get(self.width * word_pos.y + word_pos.x + i).unwrap());
                }
            },
            Dir::VER => {
                for i in 0..word_pos.len {
                    word.push(*self.arr.get(self.width * (word_pos.y + i) + word_pos.x).unwrap());
                }
            },
        }
        word
    }

    // Set char at given coordinate
    #[allow(dead_code)]
    pub fn set(&mut self, x: usize, y: usize, val: char) {
        self.arr[self.width * y + x] = val;
    }

    // Set word at given coordinate with dir and len
    pub fn set_word(&mut self, word_pos: &WordPos, word: &str) {
        let mut word_chars = word.chars();
        match word_pos.dir {
            Dir::HOR => {
                for i in 0..word_pos.len {
                    self.arr[self.width * word_pos.y + word_pos.x + i] = word_chars.next().unwrap();
                }
            },
            Dir::VER => {
                for i in 0..word_pos.len {
                    self.arr[self.width * (word_pos.y + i) + word_pos.x] = word_chars.next().unwrap();
                }
            },
        }
    }

    // Check if cell at the given coordinate is valid (not out of bounds and cell != '#')
    pub fn valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.get(x, y) != '#'
    }

    // Get the elements (x, y, dir, len) representing words to fill in the board
    pub fn get_words_pos(&self) -> Vec<WordPos> {
        let mut elems: Vec<WordPos> = Vec::new();

        let mut len;
        let mut xstart;
        let mut ystart;

        // HOR
        for j in 0..self.height {
            len = 0;
            xstart = 0;
            ystart = j;

            for i in 0..self.width {
                if self.valid(i, j) {
                    len += 1;
                }
                else {
                    if len > 1 {
                        elems.push(WordPos::new(xstart, ystart, Dir::HOR, len));
                    }
                    len = 0;
                    xstart = i+1;
                    ystart = j;
                }
            }
            if len > 1 {
                elems.push(WordPos::new(xstart, ystart, Dir::HOR, len));
            }
        }

        // VER
        for i in 0..self.width {
            len = 0;
            xstart = i;
            ystart = 0;

            for j in 0..self.height {
                if self.valid(i, j) {
                    len += 1;
                }
                else {
                    if len > 1 {
                        elems.push(WordPos::new(xstart, ystart, Dir::VER, len));
                    }
                    len = 0;
                    xstart = i;
                    ystart = j+1;
                }
            }
            if len > 1 {
                elems.push(WordPos::new(xstart, ystart, Dir::VER, len));
            }
        }

        elems
    }

    // Print board
    pub fn print(&self) {
        let mut pretty_matrix = String::new();

        // upper row
        for _ in 0..self.width + self.width - 1 + 2 {
            pretty_matrix.push('-');
        }
        pretty_matrix.push('\n');

        // actual grid
        for j in 0..self.height {
            pretty_matrix.push('|');
            for i in 0..self.width {
                pretty_matrix.push(self.get(i, j));
                pretty_matrix.push('|');
            }
            pretty_matrix.push('\n');

            for _ in 0..self.width + self.width - 1 + 2 {
                pretty_matrix.push('-');
            }
            pretty_matrix.push('\n');
        }

        // print
        println!("{}", pretty_matrix);
    }

}
