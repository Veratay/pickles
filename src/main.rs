use colored::*;

#[derive(Copy,Clone)]
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

    fn ones() -> Moves {
        Self {
            r0:0xFFFFFFFF,
            r1:0xFFFFFFFF,
            r2:0xFFFFFFFF,
            r3:0xFFFFFFFF,
            r4:0xFFFFFFFF,
            r5:0xFFFFFFFF,
            r6:0xFFFFFFFF,
            r7:0xFFFFFFFF,
            r8:0xFFFFFFFF,
            r9:0xFFFFFFFF,
            r10:0xFFFFFFFF,
            heights:0xFFFFFFFF,
        }
    }

    fn get_mut(&mut self, index:u32) -> &mut u32 {
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

    fn get(&self, index:u32) -> u32 {
         match index {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.r8,
            9 => self.r9,
            _ => self.r10,
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

fn make_board(pickles:Vec<(u32,u32,Pickle)>) -> Board {
    let mut r = Board::new();
    for (x,y,pickle) in pickles {
        
        let row = r.get_mut(y);
        *row = *row | make_row(x,pickle);
    }
    r
}

fn make_row(x:u32,pickle:Pickle) -> u32 {
    let mut r:u32 = match pickle {
        Pickle::White => 1,
        Pickle::Purple => 3,
        Pickle::Green => 5,
        Pickle::Yellow => 9
    };
    r = r << (x*4);
    r
}

// pub fn elim_illegal_moves(board:&Board,moves:&mut Moves) {
    
// }

// const mosaic_colors:[u32; 9] = [0x333,0x555,0x999,0x359,0x593,0x935, 0x395,0x953,0x539];
// const mosaic_colors_2:[u8; 6] = [0x33,0x55,0x99,0x59,0x93,0x39,0x95,0x53]; 

pub fn elim_mosaic_breaking_moves(x:u32,y:u32,board:&Board) -> u32 {
    let mut r0 = if y<10 { board.get(y+1) } else { 0 };
    let mut r1 = board.get(y);
    let mut r2 = if y>0 { board.get(y-1) } else { 0 };
    let mut r3 = if y>1 { board.get(y-2) } else { 0 };

    println!("r0: {:#016x}",r0);
    println!("r1: {:#016x}",r1);
    println!("r2: {:#016x}",r2);
    println!("r3: {:#016x}",r3);

    //shift on a unsigned integer is a logical shift, so bits will be deleted.
    let mask0:u32 = 0x0000FFFFu32;
    let mask1:u32 = 0x000FFFFFu32;
    let mask2:u32 = 0x00000FFFu32;
    
    if x>2 { 
        r0 = r0 >> (x-2);
        r1 = r1 >> (x-2);
        r2 = r2 >> (x-2);
        r3 = r3 >> (x-2);
    } else if x<2 { 
        r0 = r0 << (2-x);
        r1 = r1 << (2-x);
        r2 = r2 << (2-x);
        r3 = r3 << (2-x);
    }

    r3 = r3 >> 4;

    if y%2==0 {
        r0 = r0 >> 4;
        r2 = r2 >> 4;
    }

    println!("r0: {:#016x}",r0);
    println!("r1: {:#016x}",r1);
    println!("r2: {:#016x}",r2);
    println!("r3: {:#016x}",r3);

    let working:u64 = (mask0 & r0) as u64 | (((mask1 & r1) as u64) << 4*4) | (((mask0 & r2) as u64) << 9*4) | (((mask2 & r3) as u64) << 13*4);

    println!("Working: {:#018x}",working);

    //x,y is the last pixel to complete a mosaic
    let v1_and_0:u64 =    0xEEE_E00E_0E0E0_0000;
    let v1_color_1:u64 =  0x000_0E00_00000_0000;
    let v1_color_2:u64=   0x000_00E0_00000_0000;

    let v2_and_0:u64 =    0x0EE_0E0E_0E00E_000E;
    let v2_color_1:u64 =  0x000_0000_000E0_0000;
    let v2_color_2:u64=   0x000_00E0_00000_0000;

    let v3_and_0:u64 =    0xEE0_E0E0_E00E0_E000;
    let v3_color_1:u64 =  0x000_0E00_00000_0000;
    let v3_color_2:u64 =  0x000_0000_0E000_0000;

    if v1_and_0 & working == 0 && v1_color_1 & working != 0 && v1_color_2 & working != 0 {
        let color_1 = (working & v1_color_1) >> 11*4;
        let color_2 = (working & v1_color_2) >> 10*4;
        println!("COLORS: {}, {}",color_1,color_2);
        let missing_color = if color_1 == color_2 { color_1 } else { 0xE - color_1 - color_2 };
        
        //make completing the mosaic the only legal move
        return missing_color as u32;
    }

    if v2_and_0 & working == 0 && v2_color_1 & working != 0 && v2_color_2 & working != 0 {
        let color_1 = (working & v2_color_1) >> 5*4;
        let color_2 = (working & v2_color_2) >> 10*4;
        let missing_color = if color_1 == color_2 { color_1 } else { 0xE - color_1 - color_2 };
        
        //make completing the mosaic the only legal move
        return missing_color as u32;
    }

    if v3_and_0 & working == 0 && v3_color_1 & working != 0 && v3_color_2 & working != 0 {
        let color_1 = (working & v3_color_1) >> 11*4;
        let color_2 = (working & v3_color_2) >> 7*4;
        let missing_color = if color_1 == color_2 { color_1 } else { 0xE - color_1 - color_2 };
        
        //make completing the mosaic the only legal move
        return missing_color as u32;
    }
    
    let w1_and_0 = 0xEE0_E000_0E000_0000;
    let w1_color = 0x000_0E00_00000_0000;

    let w2_and_0 = 0x000_EE00_E0000_E000;
    let w2_color = 0x000_0000_0E000_0000;
    
    let w3_and_0 = 0x0EE_0E0E_000EE_0000;
    let w3_color = 0x000_00E0_00000_0000;

    let w4_and_0 = 0x000_00EE_0000E_000E;
    let w4_color = 0x000_0000_000E0_0000;

    //x,y is touching two colored tiles
    let mut touching = 0;
    if w1_color & working != 0 { touching += 1; }
    if w2_color & working != 0 { touching += 1; }
    if w3_color & working != 0 { touching += 1; }
    if w4_color & working != 0 { touching += 1; }
    
    println!("TOUCHING={}",touching);

    if touching > 1 {
        return 0x1;
    }

    println!("w1={:#018x} {:#018x}",w1_and_0 & working,w1_color & working);

    //x,y is bordering a colored tile connected to another one already
    if (w1_and_0 & working != 0 && w1_color & working != 0) ||
    (w2_and_0 & working != 0 && w2_color & working != 0) ||
    (w3_and_0 & working != 0 && w3_color & working != 0) ||
    (w4_and_0 & working != 0 && w4_color & working != 0) {
       return 0x1;
    }

    //x,y has a colored tile under it but the 3rd space for mosaic is covered by white
    let o2 =    0x000_0000_0F000_0000;
    let o2_eq = 0x000_0000_01000_0000;
    let o4 =    0x000_0000_000F0_0000;
    let o4_eq = 0x000_0000_00010_0000;
    if (w1_color & working != 0 && o2 & working == o2_eq) || (w3_color & working != 0 && o4 & working == o4_eq) {
        return 0x1;
    }

    return 0xF;

}

//PRINTING STUFF

const PRINT_PADDING:usize = 4;
fn print_board(board:&Board) {
    for r in 0..11 {
        let padding = " ".repeat(PRINT_PADDING);
        let mut line = if r%2==0 { " ".repeat(PRINT_PADDING/2+2) } else { String::new() };
        let clen = if r%2==0 { 6 } else { 7 };        
        for c in 0..clen {
            line = format!("{}{}{}",line,padding.clone(),color_pixel(get_column(board.get(10-r), c)));
        }
        println!("{}",line);
    }
}

fn print_moves(moves:&Moves) {
    for r in 0..11 {
        let padding = " ".repeat(PRINT_PADDING);
        let mut line = if r%2==0 { " ".repeat(PRINT_PADDING/2+2) } else { String::new() };
        let clen = if r%2==0 { 6 } else { 7 };        
        for c in 0..clen {
            line = format!("{}{}{}",line,padding.clone(),get_column(moves.get(10-r), c));
        }
        println!("{}",line);
    }
}

fn get_column(row:u32,x:u32) -> String {
    let mask:u32 = 0x0000000F;
    let res = (row >> (x*4)) & mask;
    return format!("{:04b}",res);
}

fn color_pixel(pixel:String) -> ColoredString {
    match pixel.as_str() {
        "0001" => pixel.black().on_white(),
        "0011" => pixel.white().on_purple(),
        "0101" => pixel.black().on_green(),
        "1001" => pixel.black().on_yellow(),
        "0000" => pixel.white(),
        _ => panic!("{}",pixel)
    }
}

fn main() {
    println!("Hello, world!");
    // let pickles = vec![(1,1,Pickle::White),(2,1,Pickle::White),(3,1,Pickle::White),
    // (4,2,Pickle::Purple),(1,2,Pickle::Purple),(2,2,Pickle::Purple),(3,2,Pickle::Purple),
    // (0,3,Pickle::Green),(1,3,Pickle::Green),(3,3,Pickle::Green),(4,3,Pickle::Green),
    // (1,4,Pickle::Yellow),(4,4,Pickle::Yellow)];

    let pickles = vec![(1,2,Pickle::White),(2,1,Pickle::Green),(1,1,Pickle::White)];
    let board = make_board(pickles);
    let mut moves = Moves::ones();

    let result = elim_mosaic_breaking_moves(2,2,&board);
    
    print_board(&board);
    println!("{}","=".repeat(15*PRINT_PADDING));
    println!("result is: {:04b}",result);
}
