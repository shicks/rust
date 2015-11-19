use std::collections::{HashMap,HashSet};
use std::hash::Hash;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

macro_rules! do_option {
    ($x:expr) => (match $x {
        None => { return None; },
        Some(x) => x
    })
}

/// Iterates over multiple ranges at once
struct Vecs<'a> {
    start: usize,
    ends: &'a Vec<usize>,
    pos: usize,
}

impl<'a> Iterator for Vecs<'a> {
    type Item = Vec<usize>;

    #[inline]
    fn next(&mut self) -> Option<Vec<usize>> {
        let mut out = vec![];  // TODO - preinitialize capacity?
        let mut num = self.pos;
        self.pos += 1;
        for end in self.ends {
            let div = end - self.start;
            if div == 0 {
                return None;
            }
            out.push(self.start + (num % div));
            num /= div;
        }
        if num > 0 {
            None
        } else {
            Some(out)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut total = 1;
        for end in self.ends {
            total *= end - self.start;
        }
        let left = total - self.pos;
        (left, Some(left))
    }
}

impl<'a> Vecs<'a> {
    fn start(start: usize, ends: &'a Vec<usize>) -> Vecs<'a> {
        Vecs{start: start, ends: ends, pos: 0}
    }
}

#[derive(Clone,PartialEq,Eq,Hash)]
struct Counts {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Counts {
    fn size(&self) -> usize {
        let mut c: usize = 0;
        for row in &self.rows {
            c += *row;
        }
        c
    }

    fn non_trivial(&self) -> bool {
        for row in &self.rows {
            if *row == 0 || *row == self.cols.len() { return false; }
        }
        for col in &self.cols {
            if *col == 0 || *col == self.rows.len() { return false; }
        }
        true
    }
}

impl Display for Counts {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in &self.rows {
            s.push_str(&format!("{} ", row));
        }
        s.push('|');
        for col in &self.cols {
            s.push_str(&format!(" {}", col));
        }
        write!(f, "{}", s)
    }
}

struct Pixel {
    values: Vec<bool>,
    width: usize,
    height: usize,
}

impl Index<(usize, usize)> for Pixel {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &bool {
        let (x, y) = index;
        &self.values[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Pixel {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut bool {
        let (x, y) = index;
        &mut self.values[y * self.width + x]
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push('+');
        for _ in 0..self.width { s.push('-'); }
        s.push_str("+\n");
        for y in 0..self.height {
            s.push('|');
            for x in 0..self.width {
                s.push(if self[(x, y)] { '*' } else { ' ' });
            }
            s.push_str("|\n");
        }
        s.push('+');
        for _ in 0..self.width { s.push('-'); }
        s.push('+');
        write!(f, "{}", s)
    }
}

impl Pixel {
    fn new(width: usize, height: usize) -> Pixel {
        Pixel{values: vec![false; width * height], width: width, height: height}
    }

    fn count<F>(&self, max: usize, getter: F) -> Option<usize> 
        where F : Fn(usize) -> bool {
        let mut count: isize = 0;
        let mut result = Some(0);
        for z in (0..max) {
            match (getter(z), count) {
                (true, -1) => return None, // multiple groups
                (true, _) => {
                    count += 1;
                    result = Some(count as usize);
                },
                (false, 0) | (false, -1) => (),
                (false, _) => count = -1,
            }
        }
        result
    }

    fn row(&self, y: usize) -> Option<usize> {
        self.count(self.width, |x| { self[(x, y)] })
    }

    fn col(&self, x: usize) -> Option<usize> {
        self.count(self.height, |y| { self[(x, y)] })
    }

    fn counts(&self) -> Option<Counts> {
        let mut rows = vec![];
        let mut cols = vec![];
        for x in 0..self.width {
            cols.push(do_option!(self.col(x)));
        }
        for y in 0..self.height {
            rows.push(do_option!(self.row(y)));
        }
        Some(Counts{rows: rows, cols: cols})
    }
}

struct UniqueMap<K, V> where K: Eq+Hash {
    doubles: HashSet<K>,
    data: HashMap<K, V>,
}

impl<K: Clone+Eq+Hash, V> UniqueMap<K, V> {
    fn new() -> UniqueMap<K, V> {
        UniqueMap{doubles: HashSet::new(), data: HashMap::new()}
    }

    fn put(&mut self, key: K, value: V) {
        if self.data.contains_key(&key) {
            self.data.remove(&key);
            self.doubles.insert(key);
        } else if !self.doubles.contains(&key) {
            self.data.insert(key, value);
        }
    }

    fn drain_unique(&mut self) -> Vec<(K, V)> {
        let mut keys: Vec<K> = vec![];
        {
            for key in self.data.keys() {
                keys.push(key.clone());
            }
        }
        keys.iter().map(|k| (k.clone(), self.data.remove(k).unwrap())).collect()
    }
}

fn find() -> Pixel {
    for size in (2..) {
        let mut map: UniqueMap<Counts, Pixel> = UniqueMap::new();
        // TODO(sdh): if we're smarter about generating these, then
        // we can save lots of work...

        let limits = vec![size; size];
        for counts in Vecs::start(1, &limits) {
            let max_starts: Vec<usize> =
                counts.iter().map(|count| { size - count + 1 }).collect();
            for starts in Vecs::start(0, &max_starts) {
                // Build up the board
                let mut pix = Pixel::new(size, size);
                for row in 0..size {
                    for col in 0..counts[row] {
                        pix[(col + starts[row], row)] = true;
                    }
                }
                match pix.counts() {
                    Some(c) => {
                        if c.non_trivial() { map.put(c, pix) };
                    },
                    None => (),
                }
            }
        }

        if !map.data.is_empty() {
            let mut best_count = 0;
            let mut best: Option<Pixel> = None;
            for (k, v) in map.drain_unique() {
                if best.is_none() || k.size() < best_count {
                    best_count = k.size();
                    best = Some(v);
                }
            }
            return best.unwrap();
        }
        
    }
    panic!("Impossible")
}


fn main() {
    let p = find();
    println!("{}", p);
    match p.counts() {
        Some(c) => println!("{}", c),
        None => println!("non-unique"),
    }
}
