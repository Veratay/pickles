use std::time::Instant;

use colored::*;
use rand::Rng;

#[derive(Copy,Clone,Debug)]
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
    heights:u32, //each nibble is the height of 
    white_pixels_left:u8,
    purple_pixels_left:u8,
    green_pixels_left:u8,
    yellow_pixels_left:u8
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
            white_pixels_left:30,
            purple_pixels_left:5,
            green_pixels_left:5,
            yellow_pixels_left:5
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
    #[inline]
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
            10 => self.r10,
            _ => 0
        }
    }

    fn apply_move(&mut self, m:Move) {
        insert(self, m.column, m.color);
    }
}

enum Pickle {
    White,
    Purple,
    Green,
    Yellow
}

impl Pickle {
    fn to_u32(&self) -> u32 {
        match self {
            Pickle::White => 1,
            Pickle::Purple => 3,
            Pickle::Green => 5,
            Pickle::Yellow => 9
        }
    }
}

#[derive(Debug,Clone, Copy)]
struct Move {
    column:u32,
    color:u32
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
    let mut r:u32 = pickle.to_u32();
    r = r << (x*4);
    r
}

//PROCESSING

pub fn elim_mosaic_breaking_moves(x:u32,y:u32,board:&Board) -> u32 {
    let mut r0 = board.get(y+1);
    let mut r1 = board.get(y);
    let mut r2 = board.get(y-1);
    let mut r3 = board.get(y-2);

    //shift on a unsigned integer is a logical shift, so bits will be deleted.
    let mask0:u32 = 0x0000FFFFu32;
    let mask1:u32 = 0x000FFFFFu32;
    let mask2:u32 = 0x00000FFFu32;
    
    if x>2 { 
        r0 = r0 >> (x-2)*4;
        r1 = r1 >> (x-2)*4;
        r2 = r2 >> (x-2)*4;
        r3 = r3 >> (x-2)*4;
    } else if x<2 { 
        r0 = r0 << (2-x)*4;
        r1 = r1 << (2-x)*4;
        r2 = r2 << (2-x)*4;
        r3 = r3 << (2-x)*4;
    }

    r3 = r3 >> 4;

    if y%2==0 {
        r0 = r0 >> 4;
        r2 = r2 >> 4;
    }

    let working:u64 = (mask0 & r0) as u64 | (((mask1 & r1) as u64) << 4*4) | (((mask0 & r2) as u64) << 9*4) | (((mask2 & r3) as u64) << 13*4);

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
    
    // x x x x
    //x x 0 x x
    // x x x x
    //  x x x

    let w1_and_0 = 0xEE0_E0E0_0E000_0000;
    let w1_color = 0x000_0E00_00000_0000;

    let w2_and_0 = 0x000_EE00_E0000_E000;
    let w2_color = 0x000_0000_0E000_0000;
    
    let w3_and_0 = 0x0EE_0E0E_000E0_0000;
    let w3_color = 0x000_00E0_00000_0000;

    let w4_and_0 = 0x000_00EE_0000E_000E;
    let w4_color = 0x000_0000_000E0_0000;

    //x,y is touching two colored tiles
    let mut touching = 0;
    if w1_color & working != 0 { touching += 1; }
    if w2_color & working != 0 { touching += 1; }
    if w3_color & working != 0 { touching += 1; }
    if w4_color & working != 0 { touching += 1; }

    if touching > 1 {
        return 0x1;
    }

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

    //x,y is bordering a singular colored one that itself is not surrounded by any colored pixels. therefore, this m-osaic should be continued.
    if w1_color & working != 0 || w2_color & working != 0 || w3_color & working != 0 || w4_color & working != 0  {
        return 0xE;
    }

    return 0xF;

}

fn enforce_pixels_left(board:&Board, potential_move:u32) -> u32 {
    let mut mask = if board.white_pixels_left != 0 { 0x1 } else { 0x0 };
    if board.purple_pixels_left != 0 { mask += 0x2 };
    if board.green_pixels_left != 0 { mask += 0x4 };
    if board.yellow_pixels_left != 0 { mask += 0x8 };
    // println!("{}",mask);
    return potential_move & mask;
}

fn is_placeable(board:&Board, x:u32,y:u32) -> bool {
    if y==0 { return x!=6;}
    let below = board.get(y-1);
    if y%2==1 {
        if x==0 { return true; }
        if x==1 { return below & 0x1 != 0; }
        return (below >> ((x-1)*4)) & 0x1 != 0;
    } else {
        return ((below >> ((x+1)*4)) & 0x1 != 0) && x!=6;
    }
}

static mut POSITIONS:u32 = 0;
fn calc_score(board:&Board) -> f64 {
    let mut score = 0;
    for y in 0..=10 {
        for x in if y%2==0 { 0..=5 } else { 0..=6 } {
            if (0xF << (x*4)) & board.get(y) != 0 {
                score += 3;
            }
            
            let and_0_1:u64 = 0x00_EE0_E0E0_E00E_EEE;
            let mask_1:u64 =  0x00_000_0E00_0EE0_000;
            let and_0_2:u64 = 0x00_EEE_E00E_0E0E_0EE;
            let mask_2:u64=   0x00_000_0EE0_00E0_000;

            let mask_1_color_1:u64 = 0x00_000_0E00_0000_000;
            let mask_1_color_2:u64 = 0x00_000_0000_0E00_000;
            let mask_1_color_3:u64 = 0x00_000_0000_00E0_000;

            let mask_2_color_1:u64 =  0x00_000_0E00_0000_000;
            let mask_2_color_2:u64 =  0x00_000_00E0_0000_000;
            let mask_2_color_3:u64 =  0x00_000_0000_00E0_000;

            let working = if y%2 == 0 {
                ((board.get(y-1) >> (x*4)) & 0xFFF) as u64 |
                ((right_signed(board.get(y) , (x-1)*4) & 0xFFFF) as u64) << (4*3) |
                (((board.get(y+1) >> ((x)*4)) & 0xFFFF) as u64) << (4*7) |
                (((board.get(y+2) >> (x*4)) & 0xFFF) as u64) << (4*11)
            } else { 
                ((right_signed(board.get(y-1) , (x-1)*4)) & 0xFFF) as u64 |
                ((right_signed(board.get(y) , (x-1)*4) & 0xFFFF) as u64) << (4*3) |
                ((right_signed(board.get(y+1) , ((x-2)*4)) & 0xFFFF) as u64) << (4*7) |
                ((right_signed(board.get(y+2) , ((x-1)*4)) & 0xFFF) as u64) << (4*11)
            };

            if and_0_1 & working == 0 && mask_1 & working > 0x00_000_0200_0220_000  {
                let color1 = (mask_1_color_1 & working) >> (9*4);
                let color2 = (mask_1_color_2 & working) >> (5*4);
                let color3 = (mask_1_color_3 & working) >> (4*4);

                if (color1==color2 && color2==color3) || (0xE-color1-color2-color3) == 0 {
                    // print_board(board);
                    // println!("x:{},y:{}, working:{:016x}",x,y,working);
                    score += 10;
                }
            }

            if and_0_2 & working == 0 && mask_2 & working > 0x00_000_0220_0020_000 {
                let color1 = (mask_2_color_1 & working) >> (9*4);
                let color2 = (mask_2_color_2 & working) >> (8*4);
                let color3 = (mask_2_color_3 & working) >> (4*4);

                if (color1==color2 && color2==color3) || (0xE-color1-color2-color3) == 0 {
                    // print_board(board);
                    // println!("x:{},y:{}, working:{:016x}",x,y,working);
                    score += 10;
                }
            }
        }
    }
    unsafe { POSITIONS += 1; }
    return score as f64;
}

#[inline(always)]
fn right_signed(src:u32, rhs:i32) -> u32 {
    if rhs >= 0 {
        return src >> rhs;
    } else {
        return src << rhs.abs();
    }
}

fn find_moves(board:&Board, depth:u32) -> Vec<(Move,f64)> {
    let mut moves:Vec<(Move,f64)> = Vec::new();

    for x in 0..=6 {
        let y = (board.heights & (0xF << x*4)) >> x*4;
        let pixel = enforce_pixels_left(&board, if is_placeable(&board, x, y) { elim_mosaic_breaking_moves(x, y, &board) } else { 0});
        
        if pixel == 0 { continue; }
        for (mask,res) in [(2,3),(4,5),(8,9),(1,1)] {
            let mut moved_board = board.clone();
            if mask & pixel != 0 { insert(&mut moved_board, x, res); } else { continue; }
            let v = eval_self_move(moved_board,depth-1);
            moves.push((Move {column:x,color:res},v));
        }
    }

    return moves;
}

fn eval_self_move(board:Board, depth:u32) -> f64 {
    let mut max:f64 = -1.0;
    if depth == 0 {
        return calc_score(&board);
    }

    let mut mosaic_breaking:[u32; 7] = [0,0,0,0,0,0,0];

    for (res,x) in mosaic_breaking.iter_mut().zip([0,1,2,3,4,5,6]) {
        let y = (board.heights & (0xF << x*4)) >> x*4;
        if is_placeable(&board, x, y) { 
            let moves = enforce_pixels_left(&board, if is_placeable(&board, x, y) { elim_mosaic_breaking_moves(x, y, &board) } else { 0});
            if moves == 0x3 || moves == 0x5 || moves == 0x9 {
                let mut moved_board = board.clone(); 
                insert(&mut moved_board, x, moves);
                return if false { eval_partner_move(moved_board,depth-1) } else { eval_self_move(moved_board,depth-1)+5.0 };
            }
            *res = moves;
        }
    }

    for x in 0..=6 {
        let y = (board.heights & (0xF << x*4)) >> x*4;
        let pixel = mosaic_breaking[x as usize];
        if pixel == 0 { continue; }
        for (mask,res) in [(2,3),(4,5),(8,9),(1,1)] {
            let mut moved_board = board.clone();
            if mask & pixel != 0 { insert(&mut moved_board, x, res); } else { continue; };
            let v = if false { eval_partner_move(moved_board,depth-1) } else { eval_self_move(moved_board,depth-1) };
            max = max.max(v);
        }
    }

    if max == -1.0 {
        return calc_score(&board);
    }

    return max;
}

fn eval_partner_move(board:Board, depth:u32) -> f64 {
    if depth == 0 {
        return calc_score(&board);
    }

    let mut average = -1.0;
    let mut num_average = 0;
    for x in 0..=6 {
        let y = (board.heights & (0xF << x*4)) >> x*4;
        if is_placeable(&board, x, y) && enforce_pixels_left(&board, 0x1) != 0 {
            let mut moved_board = board.clone();
            insert(&mut moved_board, x, 0x1);
            let v = if depth%2==1 { eval_self_move(moved_board, depth-1) } else { eval_partner_move(board, depth-1)};
            if num_average==0 { average = v; continue; }

            average *= num_average as f64/(num_average+1) as f64;
            average += v/(num_average+1) as f64;
            num_average += 1;
        }
    }

    if average == -1.0 {
        return calc_score(&board);
    }

    return average;
}

fn insert(board:&mut Board, column:u32, pixel:u32) {
    let mask = 0xF << column*4;
    let height = (board.heights & mask) >> column*4;
    let row = board.get_mut(height);
    *row = *row | (pixel << column*4);
    board.heights += (if column == 6 {0x2} else { 0x1 }) << column*4;
    match pixel {
        0x1 => board.white_pixels_left -= 1,
        0x3 => board.purple_pixels_left -= 1,
        0x5 => board.green_pixels_left -= 1,
        0x9 => board.yellow_pixels_left -= 1,
        _ => ()
    }
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

fn print_moves(board:&Board) {
    for r in 0..11 {
        let padding = " ".repeat(PRINT_PADDING);
        let mut line = if r%2==0 { " ".repeat(PRINT_PADDING/2+2) } else { String::new() };
        let clen = if r%2==0 { 6 } else { 7 };        
        for c in 0..clen {
            line = format!("{}{}{}",line,padding.clone(),color_pixel_move(get_column(board.get(10-r), c)));
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

fn color_pixel_move(pixel:String) -> String {
    return format!("{}{}{}{}",
        if pixel.chars().nth(0) == Some('1') { String::from("1").yellow() } else { String::from("0").white() },
        if pixel.chars().nth(1) == Some('1') { String::from("1").green() } else { String::from("0").white()},
        if pixel.chars().nth(2) == Some('1') { String::from("1").purple() } else { String::from("0").white()},
        if pixel.chars().nth(3) == Some('1') { String::from("1").on_white().black() } else { String::from("0").white()}
    );
    
    let mut res = String::new();
    for (i,x) in pixel.chars().enumerate() {
        if x =='1' {
            let add = String::from("1");
            res.push_str(& match i {
                0 => add.yellow(),
                1 => add.green(),
                2 => add.purple(),
                _ => add.white()
            });
        } else {
            res.push_str("0");
        }
    }
    return res;
}


pub fn main() {
    let mut rng = rand::thread_rng();
    let pickles = vec![];
    let mut board = make_board(pickles);

    for x in 0..45 {
        if true {
            unsafe { POSITIONS = 0; }
            let instant = Instant::now();
            let moves = find_moves(&board, 7);
            unsafe { println!("Analyzed {} positions in {} ms",POSITIONS,instant.elapsed().as_millis()); }
            println!("{:#?}",moves);
            let m = moves.iter().fold((Move {column:0,color:0},0.0), |acc,x| if x.1 > acc.1 { *x } else { acc });
            board.apply_move(m.0);

        } else {
            let mut m = Move {
                column: if ((board.heights & (0xF << (6*4))) >> (6*4)) % 2 == 0 { rng.gen_range(0..=6) } else {  rng.gen_range(0..=5) },
                color:0x1
            };

            while ( !is_placeable(&board,m.column,(board.heights & (0xF << m.column*4)) >> m.column*4) ) {
                m = Move {
                    column: if ((board.heights & (0xF << (6*4))) >> (6*4)) % 2 == 0 { rng.gen_range(0..=6) } else {  rng.gen_range(0..=5) },
                    color:0x1
                };
            }
            board.apply_move(m);
        }
        print_board(&board);
        println!("score = {}", calc_score(&board));
        println!("{}","=".repeat(15*4));
    }

    println!("{:#?}",find_moves(&board, 5));
}
