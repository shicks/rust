use std::fmt;
use std::vec::Vec;

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: u8, y: u8) -> Pos {
        if x > 3 || y > 3 {
            panic!("Invalid point: ({}, {})", x, y);
        }
        Pos{x: x as usize, y: y as usize}
    }

    fn connected(self: &Pos, other: &Pos) -> bool {
        if self.x == other.x {
            (self.y as i8 - other.y as i8).abs() == 1
        } else if self.y == other.y {
            (self.x as i8 - other.x as i8).abs() == 1
        } else {
            false
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Piece {
    x: Pos,
    o: Pos,
}

impl Piece {
    fn new(x: Pos, o: Pos) -> Piece {
        if !x.connected(&o) {
            panic!("Invalid piece [{}, {}]", x, o);
        }
        Piece{x: x, o: o}
    }

    fn add(&self, board: &mut Vec<char>) {
        board[self.x.y * 16 + self.x.x * 2] = 'X';
        board[self.o.y * 16 + self.o.x * 2] = 'O';
        if self.x.x == self.o.x { // vertical
            board[(self.x.y + self.o.y) * 8 + self.x.x * 2] = '‖';
        } else {
            board[self.x.y * 16 + self.x.x + self.o.x] = '=';
        }
    }
}

#[derive(Debug)]
struct Board {
    pieces: Vec<Piece>,
}

impl Board {
    fn new() -> Board {
        Board{pieces: vec![]}
    }

    fn add(&mut self, p: Piece) {
        self.pieces.push(p);
    }
}

static EMPTY_BOARD: &'static str =
    " | | | \n-+-+-+-\n | | | \n-+-+-+-\n | | | \n-+-+-+-\n | | | ";

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board: Vec<char> = EMPTY_BOARD.chars().collect();
        for piece in &self.pieces {
            piece.add(&mut board);
        }
        let mut out: String = String::new();
        for ch in board {
            out.push(ch);
        }
        //write!(f, "{}", String::from_chars(board.as_slice()))
        write!(f, "{}", out)
    }
}

fn main() {
    let mut board = Board::new();
    board.add(Piece::new(Pos::new(0, 0), Pos::new(0, 1)));
    board.add(Piece::new(Pos::new(0, 2), Pos::new(1, 2)));
    println!("{}", board);
}
