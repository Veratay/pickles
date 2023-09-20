use std::ops::{Index,IndexMut};




pub struct Board {
    r0:u32, //every nibble in the u32 corresponds to the xth from the left:
    r1:u32, //0000: Nothing (0x0)
    r2:u32, //xxx1: Something (0x1)
    r3:u32, //0001: white (0x1)
    r4:u32, //0011: purple (0x3)
    r5:u32, //0101: green (0x5)
    r6:u32, //1001: yellow (0x9)
    r7:u32,
    r8:u32,
    r9:u32,
    r10:u32,
    heights:u32 //each nibble is the height of 
}

impl Board {
    fn new() -> Self {
        Self {
            r0:0,
            r1:0,
            r2:0,
            r3:0,
            r4:0,
            r5:0,
            r6:0,
            r7:0,
            r8:0,
            r9:0,
            r10:0,
            heights:0,
        }
    }
}

//dont worry ab out of bounds
impl IndexMut for Board {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            4 => &mut self.r4,
            5 => &mut self.r5,
            6 => &mut self.r6,
            7 => &mut self.r7,
            8 => &mut self.r8,
            9 => &mut self.r9,
            _ => &mut self.r10,
        }
    }
}

impl Index for Board {
    fn index(&self, index: Idx) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            3 => &self.r3,
            4 => &self.r4,
            5 => &self.r5,
            6 => &self.r6,
            7 => &self.r7,
            8 => &self.r8,
            9 => &self.r9,
            _ => &self.r10,
        }
    }
}

type Moves = Board; //Moves are represented the same way as Boards, except moves start initialized where all the allowed moves are one. 
//Then, the bits that correspond to each move are flipped.
//Ex- a move in the first cell would start as 1111.
//If white placement wasnt allowed, it would become 1110. 
//If all color placements werent allowed, but white was, it would be 0001.
//No moves in that cell is represented by 0000.
//height field is available pieces
//basically a union but not a union

enum Pickle {
    White,
    Purple,
    Green,
    Yellow
}

fn make_board(pickles:Vec<(u8,u8,Pickle)>) -> Board {
    let mut r = Board::new();
    for (x,y,pickle) in pickles {
        
        let row = match y {
            0=> &mut r.r0,
            1=> &mut r.r1,
            2=> &mut r.r2,
            3=> &mut r.r3,
            4=> &mut r.r4,
            5=> &mut r.r5,
            6=> &mut r.r6,
            7=> &mut r.r7,
            8=> &mut r.r8,
            9=> &mut r.r9,
            _=> &mut r.r10
        };
        *row = *row & make_row(x,pickle);
    }
    r
}

fn make_row(x:u8,pickle:Pickle) -> u32 {
    let mut r:u32 = match pickle {
        Pickle::White => 1,
        Pickle::Purple => 3,
        Pickle::Green => 5,
        Pickle::Yellow => 9
    };
    r = r << x;
    r.reverse_bits();
    r
}

pub fn elim_illegal_moves(board:&Board,moves:&mut Moves) {
    
}

pub fn elim_mosaic_breaking_moves(board:&Board, moves:&mut Moves) {

    for r in 0..11 {
        //first 8 nibbles is one of the row. Second is for the row above/below it.
        //merged into 64bits bc uh idk
        let row:u64 = (board[r] as u64).rotate_left(32) | (if r<10 { board[r+1] } else { 0 });
        let row_lower:u32 = if r>0 { board[r-1] } else { 0 };

        let moves_row:&mut u32 = moves[r];
        let moves_lower:&mut u32 = if r>0 { moves[r-1] } else { 0 };
        let moves_upper:&mut u32 = if r<10 { board[r+1] } else { 0 };
        let moves_upper_upper:&mut u32 = if r<9 { board[r+2] } else { 0 };
        for _ in 1..=2 {
            /*
            first N is at idx 0, when it shifts there needs to be an N where the star is.
            in the hex in this case C means the color bit, so for purple 2, green 4, and yellow 88
            E* = wraparound
            assumes board is valid(no hanging)
            N N    upper-upperso count ones only has to do one thing
            N C N  != E0E00000 & 0C000000 upper
            *C C N != 00E0000E*& CC000000 row
            N N N  != EEE00000 lower
            */

            /*
            if row is at one of the odd numbers then its like this and theres quite a bit of overflow
            N N
                C N
            C C N
                N N
            */

            //if no surrounding spots are colored. it is okay for the E to be at the end, because the largest row is 7 long
            
            let mut color_mask:u64 =     0xFF000000_0F000000;
            let mut valid_purple:u64 =   0x22000000_02000000;
            let mut valid_green:u64 =    0x44000000_04000000;
            let mut valid_yellow:u64 =   0x88000000_08000000;
            let mut not_kernel:u64 =     0x00E00000_E0E0000E; //<-overflow
            let mut not_kernel_lower:u32=0xEEE00000;

            let mut valid_1:u64 =        0x24000000_08000000;
            let mut valid_2:u64 =        0x42000000_08000000;
            let mut valid_3:u64 =        0x82000000_04000000;
            let mut valid_4:u64 =        0x84000000_02000000;
            let mut valid_5:u64 =        0x48000000_02000000;
            let mut valid_6:u64 =        0x28000000_04000000;

            let mut breaking_lower:u32 = 0x111FFFFF;
            let mut breaking:u32 =       0xFF1FFFF1;
            let mut breaking_upper:u32 = 0x1F1FFFFF;
            let mut breaking_upper_upper:u32 = 0x11FFFFFF;

            if (r%2==1) {
                color_mask = 0xFF000000_F0000000;
                valid_purple=0x22000000_20000000;
                valid_green= 0x44000000_40000000;
                valid_yellow=0x80000000_80000000;
                not_kernel = 0x00E0000E_0E00000E;
                not_kernel_lower= 0xEE000000_0000000E;

                valid_1 =        0x24000000_80000000;
                valid_2 =        0x42000000_80000000;
                valid_3 =        0x82000000_40000000;
                valid_4 =        0x84000000_20000000;
                valid_5 =        0x48000000_20000000;
                valid_6 =        0x28000000_40000000;

                breaking_lower = 0x11FFFFF1;
                breaking =       0xFF1FFFF1;
                breaking_upper = 0xF1FFFFF1;
                breaking_upper_upper = 0x11FFFFF1;
            }

            for i in 1..=4 {
                let working = color_mask & row;

                //checks if there is mosaic
                if((row_lower & not_kernel_lower) == 0 && (not_kernel & row) == 0 && (
                    working==valid_purple || working==valid_green || working==valid_yellow
                    || working==valid_1 || working==valid_2 || working==valid_3 
                    || working==valid_4 || working==valid_5 || working==valid_6
                )) {
                    //bans moves that will break that mosaic
                    *moves_lower = *moves_lower & breaking_lower;
                    *moves_row = *moves_row & breaking; //again, okay to set the last pixel to whatever because it isint being used.
                    *moves_upper = *moves_upper & breaking_upper;
                    *moves_upper_upper = *moves_upper_upper & breaking_upper_upper;
                }

                //logical shift instead of arithmetic bc the thingy needs to wrap around.
                color_mask.rotate_right(1);
                valid_purple.rotate_right(1);
                valid_green.rotate_right(1);
                valid_yellow.rotate_right(1);
                not_kernel.rotate_right(1);
                not_kernel_lower.rotate_right(1);

                valid_1.rotate_right(1);
                valid_2.rotate_right(1);
                valid_3.rotate_right(1);
                valid_4.rotate_right(1);
                valid_5.rotate_right(1);
                valid_6.rotate_right(1);

                breaking_lower.rotate_right(1);
                breaking.rotate_right(1);
                breaking_upper.rotate_right(1);
                breaking_upper_upper.rotate_right(1);
            }

            row = (board[r] as u64).rotate_left(32) | (if r>0 { board[r-1] } else { 0 });
            row_lower = if r<10 { board[r+1] } else { 0 };

            moves_row = moves[r];
            moves_lower = if r<10 { moves[r+1] } else { 0 };
            moves_upper = if r>0 { board[r-1] } else { 0 };
            moves_upper_upper = if r>1 { board[r-2] } else { 0 };
        }        
    }
}

fn main() {
    println!("Hello, world!");
}
