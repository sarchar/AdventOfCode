use std::fmt;
use std::fs;
use std::io;

const SAMPLE     : bool = true;

struct Piece {
    shape: [[u8; 4]; 4],
    width : u32,
    height: u32,
}

const PIECE_DASH: Piece = Piece {
    shape : [ [ 1, 1, 1, 1 ],
              [ 0, 0, 0, 0 ],
              [ 0, 0, 0, 0 ],
              [ 0, 0, 0, 0 ] ],
    width : 4,
    height: 1,
};

const PIECE_CROSS: Piece = Piece {
    shape : [ [ 0, 1, 0, 0 ], 
              [ 1, 1, 1, 0 ],
              [ 0, 1, 0, 0 ],
              [ 0, 0, 0, 0 ] ],
    width : 3,
    height: 3,
};

const PIECE_L: Piece = Piece {
    shape : [ [ 0, 0, 1, 0 ], 
              [ 0, 0, 1, 0 ],
              [ 1, 1, 1, 0 ],
              [ 0, 0, 0, 0 ] ],
    width : 3,
    height: 3,
};

const PIECE_LINE: Piece = Piece {
    shape : [ [ 1, 0, 0, 0 ], 
              [ 1, 0, 0, 0 ],
              [ 1, 0, 0, 0 ],
              [ 1, 0, 0, 0 ] ],
    width : 1,
    height: 4,
};

const PIECE_BLOCK: Piece = Piece {
    shape : [ [ 1, 1, 0, 0 ], 
              [ 1, 1, 0, 0 ],
              [ 0, 0, 0, 0 ],
              [ 0, 0, 0, 0 ] ],
    width : 2,
    height: 2,
};

// order matters
const PIECES: [&Piece; 5] = [
    &PIECE_DASH,
    &PIECE_CROSS,
    &PIECE_L,
    &PIECE_LINE,
    &PIECE_BLOCK
];

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.shape {
            for dit in row {
                write!(f, "{}", if dit == 1 { "@" } else { "." } )?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

const FIELD_WIDTH: usize = 7;

struct Tetris<'a> {
    playfield       : Vec<[u8; FIELD_WIDTH]>,

    current_block   : Option<&'a Piece>,
    next_block_index: usize,

    block_x         : u32,
    block_y         : u32,
}

impl<'a> Tetris<'a> {
    fn new() -> Tetris<'a> {
        Tetris {
            playfield       : Vec::new(),
            current_block   : None,
            next_block_index: 0,
            block_x         : 0,
            block_y         : 0,
        }
    }

    fn height(&self) -> u32 {
        self.playfield.len() as u32
    }

    fn next_block(&mut self) {
        assert!(matches!(self.current_block, None));

        self.current_block = Some(PIECES[self.next_block_index]);
        self.next_block_index = (self.next_block_index + 1) % PIECES.len();

        // block starts with its left edge 2 spaces from the left wall
        self.block_x = 2;

        // and starts so that its bottom edge is 3 spaces from the highest rock in the playfield
        let block = self.current_block.unwrap();
        self.block_y = (self.height() + 3) + (block.height - 1);

        assert!(self.block_x + block.width < FIELD_WIDTH as u32);

        //println!("starting block at ({}, {}):\n{}", self.block_x, self.block_y, block);
    }

    fn landed(&self) -> bool {
        match self.current_block {
            None => { true },
            _    => { false },
        }
    }

    fn move_left(&mut self) {
        assert!(!matches!(self.current_block, None));
        let block = self.current_block.unwrap();

        // for each row in the piece, make sure the space 1 left the first element is clear
        for (_row_index, row) in block.shape.iter().enumerate() {
            let row_index = _row_index as u32;

            if row_index >= block.height { break; }

            // will panic! if there's something wrong with our shape
            let mut check_x: i32 = 0;
            while row[check_x as usize] != 1 { check_x = check_x + 1; }

            // adjust the check position to account for the block location, minus 1 for the space before it
            check_x = check_x + (self.block_x as i32) - 1;

            //println!("testing ({},{}) for empty space", check_x, row_index);

            // fail if the piece goes beyond the playfield
            if check_x < 0 { return; }

            // check if there's a rock in that space
            let check_y = self.block_y - row_index;

            if check_y >= self.height() { continue; } // no playfield => no rock

            let playfield_row = &self.playfield[check_y as usize];
            if playfield_row[check_x as usize] == 1 { return; }
        }

        // if we get here, nothing failed so it's safe to move
        self.block_x -= 1;
        //println!("moved block left, now = {}, {}", self.block_x, self.block_y);
    }

    fn move_right(&mut self) {
        assert!(!matches!(self.current_block, None));
        let block = self.current_block.unwrap();

        // for each row in the piece, make sure the space 1 past the last element is clear
        for (_row_index, row) in block.shape.iter().enumerate() {
            let row_index = _row_index as u32;

            if row_index >= block.height { break; }

            // will panic! if there's something wrong with our shape
            let mut check_x: u32 = block.width - 1;
            while row[check_x as usize] != 1 { check_x = check_x - 1; }

            // adjust the check position to account for the block location, plus the 1 for the space after it
            check_x = check_x + self.block_x + 1;

            //println!("testing ({},{}) for empty space", check_x, row_index);

            // fail if the piece goes beyond the playfield
            if check_x >= (FIELD_WIDTH as u32) { return; }

            // check if there's a rock in that space
            let check_y = self.block_y - row_index;

            if check_y >= self.height() { continue; } // no playfield => no rock

            let playfield_row = &self.playfield[check_y as usize];
            if playfield_row[check_x as usize] == 1 { return; }
        }

        // if we get here, nothing failed so it's safe to move
        self.block_x += 1;
        //println!("moved block right, now = {}, {}", self.block_x, self.block_y);
    }

    fn fall(&mut self) {
        assert!(!matches!(self.current_block, None));
        let block = self.current_block.unwrap();

        // for each column in the piece, make sure the space 1 below the first element is clear
        // if it is not, extend the playfield if necessary and mark those spots as solid
        for column_index in 0..block.width {
            if column_index >= block.width { break; }

            // will panic! if there's something wrong with our shape
            let mut check_y: i32 = (block.height as i32) - 1;
            while block.shape[check_y as usize][column_index as usize] != 1 { check_y -= 1; }

            // adjust the check position to account for the block location, minus 1 for the space below it
            check_y = (self.block_y as i32) - check_y - 1;
            let check_x = self.block_x + column_index;

            // land if the piece goes beyond the playfield
            if check_y < 0 { self.land(); return; }

            // check if there's a rock in that space
            if (check_y as u32) >= self.height() { continue; } // no playfield => no rock

            let playfield_row = &self.playfield[check_y as usize];
            if playfield_row[check_x as usize] == 1 { self.land(); return; }
        }

        // if we get here, nothing failed so it's safe to move
        self.block_y -= 1;
        //println!("block fell, now = {}, {}", self.block_x, self.block_y);
    }

    fn land(&mut self) {
        assert!(!matches!(self.current_block, None));

        // doesn't matter where the block is, make it into rock
        let block = self.current_block.unwrap();

        while self.height() <= self.block_y {
            self.grow();
        }

        for (_row_index, row) in block.shape.iter().enumerate() {
            if (_row_index as u32) >= block.height { break; }
            for (_column_index, dit) in row.iter().enumerate() {
                if (_column_index as u32) >= block.width { continue; }

                let x = (_column_index as u32) + self.block_x;
                let y = self.block_y - (_row_index as u32);

                if *dit == 1 {
                    self.playfield[y as usize][x as usize] = 1;
                }
            }
        }

        // finally, done with the block
        self.current_block = None;
        //println!("block landed at {},{}", self.block_x, self.block_y);
    }

    fn grow(&mut self) {
        let new_row = [0; FIELD_WIDTH];
        self.playfield.push(new_row);
    }
}

impl<'a> fmt::Display for Tetris<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..self.height()).rev() {
            let playfield_row = &self.playfield[y as usize];

            write!(f, "|")?;
            for dit in playfield_row {
                if *dit == 0 { write!(f, ".")?; }
                else         { write!(f, "#")?; }
            }
            write!(f, "|\n")?;
        }

        write!(f, "+")?;
        for _ in 0..FIELD_WIDTH { write!(f, "-")?; }
        write!(f, "+")?;

        Ok(())
    }
}

fn main() {
    // read in the sensors and their closest beacons
    let gas = fs::read_to_string(if SAMPLE { "input2.txt" } else { "input.txt" })
                    .expect("could not open input").split("\r\n").next().unwrap().trim().to_string();
    let gas_chars = gas.as_bytes();

    let mut tetris = Tetris::new();

    let mut gas_index = 0;
    let mut input_loops = 0;
    let mut last_height = 0;

    // Sorry, there's no actual implementation for part B since it's all done "on paper".
    // Just have to make use of this program by changing the loop count and looking at the
    // height of the tower as cycles repeat, then do some quick math in a calculator.

    //for i in 0..(1723+(3*1725)+1602) { // for figuring out the real input final block height
    //for i in 0..28+(3*35)+22 {         // for figuring out the sample input final block height
    for i in 0..2022 {                   // for part A
        tetris.next_block();

        while !tetris.landed() {
            match gas_chars[gas_index as usize] {
                b'<' => { tetris.move_left(); },
                b'>' => { tetris.move_right(); },
                _ => { panic!("whoopsedaisy"); },
            };

            gas_index = (gas_index + 1) % gas.len();
            tetris.fall();

            if gas_index == 0 {
                input_loops += 1;
                let diff = tetris.height() - last_height;
                println!("during the {}th piece ({}): tower is {} high (added {}) ({} input loops)", 
                         i, if tetris.landed() { "landed" } else { "not landed" },
                         tetris.height(), diff, input_loops);
                last_height = tetris.height();

                // look for a cycle in the pattern, then multiple # of thes the cycle occurs * the
                // height you get from it.
                //
                // looking at the sample input, visually inspecting shows that there's a loop:
                // of heights 10+9+8+14+12 = 53 added every time the sample input loops
                // height when cycle starts = 51  (not part of the cycle)
                // height when cycle ends   = 104 (verifies the cycle adds 53 in height)
                // number of peices fallen when cycle starts = 28 (height is 51)
                // number of pieces fallen when cycle ends   = 63 (35 pieces)
                // number of peices fell before the first cycle starts = 28
                // peices added at each step are +7, +7, +6, +8, +7 = 35 (verified)
                // first cycle starts from loop 3 to 4 (after the first 22 pieces)
                // so for 28 peices, tower is 51 high
                // then, every 35 pieces, tower grows by 53
                //
                // the number of cycle repeats is easy to determine:
                // floor( (1,000,000,000,000 - 28) / 35 ) = num_cycles = 28571428570
                // then we need to figure out the leftover peice count, which is easy:
                //
                // 1,000,000,000,000 - 35 * num_cycles - 28 = leftover_end_peices = 22
                //
                // then simulate those extra 22 peices, so simulate 28+(3*35)+22 to get a height
                // of 237. Notice that a further 27 in height is at the end, thus the total
                // height will be:
                //
                // 51 + (28571428570 * 53) + 27 = 1514285714288
                //
                // now for the real input, simulate a few million to see if a pattern occurs.
                // it's blatently obvious that there's a 1-cycle, it just repeats the same 
                // # of height added for each cycle of the input.
                //
                // before pattern occurs: 1723 peices (2613 height)
                // leftover after pattern: 1602 peices
                //
                // 1723 peices = 2613 height
                // every 1725 peices += 2630 height
                // which there are 579710143 cycles of, bringing us to 999999998398 used peices
                // (1602 remaining)
                //
                // simulating 1723+(3*1725)+1602 peices = height of 12945
                // first 1723+(3*1725) accounts for 2613+(3*2630)=10503, leaving 2442 for the last chunk of 1602 peices
                //
                // so final height is 2613 + (579710143 * 2630) + 2442 = 1524637681145
            }
        }

        //let mut s = "".to_string();
        //io::stdin().read_line(&mut s);

        //println!("------");
    }

    println!("{}", tetris.height());
}
