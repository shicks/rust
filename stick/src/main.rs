#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap,HashSet};
use std::fmt;
use std::vec::Vec;

// struct Pairs<A, T> {
//     orig: T,
//     outer: T,
//     inner: T,
//     last: Some<A>,
//     read: usize,
// }

// impl<A, T> Iterator for Pairs<A, T> where T: Clone + Iterator<Item = A> {
//     type Item = (A, A);

//     #[inline]
//     fn next(&mut self) -> Option<(A, A)> {
//         match self.last {
//             None => return None,
//             _ => (),
//         }
//         match self.inner.next() {
//             Some(x) => {
//                 self.read = self.read + 1;
//                 Some((self.last, x))
//             },
//             None => {
//                 self.last = self.outer.next();
//                 self.inner = self.orig.clone();
//                 self.next()
//             },
//         }
//     }

//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let (lower, upper) = self.orig.size_hint();
//         let lower = lower * lower - read;
//         let upper = upper.and_then(|x| x * x - read);
//         (lower, upper)
//     }
// }

// fn pairs<T: Clone + Iterator>(iter: &T) -> Iterator<(T::Item, T::Item)> {
//     Pairs{
//         orig: iter,
//         outer: iter.clone(),
//         inner: iter.clone(),
//         last: None,
//         read: 0,
//     }
// }


#[derive(Debug,PartialEq,Eq,Clone,Hash)]
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

    fn index(&self) -> usize {
        self.y * 4 + self.x
    }

    fn third(&self, other: &Pos) -> Option<Pos> {
        if self == other {
            return None;
        }
        let dx = self.x as i8 - other.x as i8;
        let dy = self.y as i8 - other.y as i8;
        if dx.abs() > 1 || dy.abs() > 1 {
            return None;
        }
        let x = self.x as i8 + dx;
        let y = self.y as i8 + dy;
        if x < 0 || x > 3 || y < 0 || y > 3 {
            return None;
        }
        //println!("third({}, {}) => ({}, {})", self, other, x, y);
        Some(Pos::new(x as u8, y as u8))
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug,Clone)]
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

    fn add(&self, board: &mut Vec<char>, turn: char) {
        board[self.x.y * 16 + self.x.x * 2] = 'X';
        board[self.o.y * 16 + self.o.x * 2] = 'O';
        // TODO(sdh): add turn number instead of = or ‖
        if self.x.x == self.o.x { // vertical
            board[(self.x.y + self.o.y) * 8 + self.x.x * 2] = turn; // '‖';
        } else {
            board[self.x.y * 16 + self.x.x + self.o.x] = turn; // '=';
        }
    }

    // fn intersects(&self, other: &Piece) -> bool {
    //     self.x == other.x || self.x == other.o ||
    //         self.o == other.x || self.o == other.o
    // }
}

lazy_static! {
    static ref ALL_POSITIONS: Vec<Pos> = {
        let mut all: Vec<Pos> = vec![];
        for x in 0..4 {
            for y in 0..4 {
                all.push(Pos::new(x, y));
            }
        }
        all
    };
}

lazy_static! {
    static ref ALL_PIECES: Vec<Piece> = {
        let mut all: Vec<Piece> = vec![];
        let pos: &Vec<Pos> = &ALL_POSITIONS;
        for x in pos {
            for o in pos {
                if x.connected(o) {
                    all.push(Piece::new(x.clone(), o.clone()));
                }
            }
        }
        all
    };
}

type Permutation = [usize;16];

const PERMUTATIONS: [Permutation;8] = [
    [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],
    [3,7,11,15,2,6,10,14,1,5,9,13,0,4,8,12],
    [15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0],
    [12,8,4,0,13,9,5,1,14,10,6,2,15,11,7,3],
    [3,2,1,0,7,6,5,4,11,10,9,8,15,14,13,12],
    [15,11,7,3,14,10,6,2,13,9,5,1,12,8,4,0],
    [12,13,14,15,8,9,10,11,4,5,6,7,0,1,2,3],
    [0,4,8,12,1,5,9,13,2,6,10,14,3,7,11,15],
    ];

#[derive(Debug,Clone)]
struct Board {
    // TODO: Make Board persistent/immutable?!?
    // TODO: Might be nice to reuse the Pos instances, rather than clone?
    pieces: Vec<Piece>,
    xs: HashSet<Pos>,
    os: HashSet<Pos>,
}

impl Board {
    fn new() -> Board {
        Board{pieces: vec![], xs: HashSet::new(), os: HashSet::new()}
    }

    fn empty(&self, p: &Pos) -> bool {
        !self.xs.contains(p) && !self.os.contains(p)
    }

    /// Adds the given piece to the board.  Returns true move was legal.
    fn add(&mut self, p: Piece) -> bool {
        if !self.empty(&p.x) || !self.empty(&p.o) {
            return false;
        }
        self.xs.insert(p.x.clone());
        self.os.insert(p.o.clone());
        self.pieces.push(p);
        true
    }

    /// Returns the result, which encodes both the winner (in
    /// the sign: positive for player 1, negative for player
    /// 2) and the length of the game (1 + number of blank
    /// spaces in the magnitude).  If the game is a draw or
    /// not yet decided, the result is zero.  Player 1 wants
    /// to maximize this, while player 2 wants to minimize it.
    fn score(&self) -> i8 {
        let x_wins = three_in_a_row(&self.xs);
        let o_wins = three_in_a_row(&self.os);
        let rest = 9 - self.pieces.len() as i8;
        if x_wins && o_wins {
            if rest % 2 == 0 {
                rest
            } else {
                -rest
            }
        } else if x_wins {
            rest
        } else if o_wins {
            -rest
        } else {
            0
        }
    }

    fn state(&self) -> String {
        let mut pos = vec![' ';16];
        for piece in &self.pieces {
            pos[piece.x.index()] = 'x';
            pos[piece.o.index()] = 'o';
        }
        let mut result: Vec<String> = PERMUTATIONS.iter().map(|permutation| {
            let mut s = String::with_capacity(16);
            for i in 0..16 {
                s.push(pos[permutation[i]])
            }
            s
        }).collect();
        result.sort();
        // TODO(sdh): Store which permutation this was, so that we
        // can map *back* to the original as needed.  This would need
        // to be returned separate to the string itself.
        result[0].clone()
    }
}

fn three_in_a_row(set: &HashSet<Pos>) -> bool {
    for p1 in set {
        for p2 in set {
            match p1.third(p2) {
                Some(p3) => {
                    if set.contains(&p3) {
                        return true;
                    }
                },
                _ => (),
            }
        }
    }
    false
}

static EMPTY_BOARD: &'static str =
    " | | | \n-+-+-+-\n | | | \n-+-+-+-\n | | | \n-+-+-+-\n | | | ";

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board: Vec<char> = EMPTY_BOARD.chars().collect();
        let mut turn = '1';
        for piece in &self.pieces {
            piece.add(&mut board, turn);
            turn = (turn as u8 + 1) as char;
        }
        let mut out: String = String::new();
        for ch in board {
            out.push(ch);
        }
        //write!(f, "{}", String::from_chars(board.as_slice()))
        write!(f, "{}", out)
    }
}

struct Solver(HashMap<String, (Board, i8)>);

impl Solver {
    fn new() -> Self {
        Solver(HashMap::new())
    }

    fn solve(&mut self, b: Board, goal: i8) -> (Board, i8) {
        let pieces: &Vec<Piece> = &ALL_PIECES;
        let mut boards: Vec<Board> = vec![];
        let state = b.state();
        match self.0.get(&state) {
            Some(&(ref b1, s1)) => return (b1.clone(), s1),
            None => (),
        }

        for p in pieces {
            let mut b2 = b.clone();
            if b2.add(p.clone()) {
                let s2 = b2.score();
                if s2 * goal > 0 {
                    // println!("WIN {}\n{}\n", goal, &b2);
                    self.0.insert(state, (b2.clone(), s2));
                    return (b2, s2);
                } else if s2 == 0 {
                    boards.push(b2);
                }
            }
        }

        let mut best_score = -100;
        let mut best_board = b.clone(); // ?!?
        for board in boards {
            let (board, s) = self.solve(board, -goal);
            if s * goal > best_score {
                best_score = s * goal;
                best_board = board;
            }
        }
        self.0.insert(state, (best_board.clone(), best_score * goal));
        (best_board, best_score * goal)
    }
}



fn main() {
    let (board, _) = Solver::new().solve(Board::new(), 1);
    println!("\nOPTIMUM:\n{}", board);
}
