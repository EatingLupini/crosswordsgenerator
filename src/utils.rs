
#[derive(Debug)]
pub enum Dir {
    HOR,
    VER,
}

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
    pub fn _get_word(&self, x: usize, y: usize, dir: Dir, len: usize) -> String {
        let mut word = String::new();
        match dir {
            Dir::HOR => {
                for i in 0..len {
                    word.push(*self.arr.get(self.width * y + x + i).unwrap());
                }
            },
            Dir::VER => {
                for i in 0..len {
                    word.push(*self.arr.get(self.width * (y + i) + x).unwrap());
                }
            },
        }
        word
    }

    // Set char at given coordinate
    pub fn set(&mut self, x: usize, y: usize, val: char) {
        self.arr[self.width * y + x] = val;
    }

    // Set word at given coordinate with dir and len
    pub fn set_word(&mut self, x: usize, y: usize, dir: Dir, len: usize, word: &str) {
        let mut word_chars = word.chars();
        match dir {
            Dir::HOR => {
                for i in 0..len {
                    self.arr[self.width * y + x + i] = word_chars.next().unwrap();
                }
            },
            Dir::VER => {
                for i in 0..len {
                    self.arr[self.width * (y + i) + x] = word_chars.next().unwrap();
                }
            },
        }
    }

    // Check if cell at the given coordinate is valid (not out of bounds and cell != '#')
    pub fn valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.get(x, y) != '#'
    }

    // Get the elements (x, y, dir, len) representing words to fill in the board
    pub fn get_words_tofill(&self) -> Vec<(usize, usize, Dir, usize)> {
        let mut elems: Vec<(usize, usize, Dir, usize)> = Vec::new();

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
                        elems.push((xstart, ystart, Dir::HOR, len));
                    }
                    len = 0;
                    xstart = i+1;
                    ystart = j;
                }
            }
            if len > 1 {
                elems.push((xstart, ystart, Dir::HOR, len));
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
                        elems.push((xstart, ystart, Dir::VER, len));
                    }
                    len = 0;
                    xstart = i;
                    ystart = j+1;
                }
            }
            if len > 1 {
                elems.push((xstart, ystart, Dir::VER, len));
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
