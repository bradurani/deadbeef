use _move::Move;
use board::*;
use rand::{Rng, SeedableRng, StdRng};
use std::collections::HashSet;
use std::mem;
use types::*;
use util::lsb;


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bound {
    Exact = 0,
    Lower = 1,
    Upper = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Entry {
    pub score: i32,
    pub best_move: Move,
    pub hash16: u16,
    pub depth: u8,
    pub info: u8, // Upper 2 bits -> bound, lowest bit -> ancient
}

impl Entry {
    const NULL: Entry = Entry {
        score: 0,
        best_move: Move::NULL,
        hash16: 0,
        depth: 0,
        info: 0,
    };

    pub fn is_empty(&self) -> bool {
        *self == Entry::NULL
    }

    pub fn ancient(&self) -> bool {
        self.info & 0b1 != 0
    }

    pub fn bound(&self) -> Bound {
        unsafe { mem::transmute(self.info >> 6) }
    }

    pub fn compare(&self, hash: &Hash) -> bool {
        self.hash16 == hash.sub()
    }
}

pub struct Table {
    pub entries: Vec<Entry>,
}

impl Table {
    pub fn empty(size: usize) -> Self {
        Table {
            entries: vec![Entry::NULL; size],
        }
    }

    pub fn empty_mb(size_mb: usize) -> Self {
        Table::empty(size_mb * 1024 * 1024 / mem::size_of::<Entry>())
    }

    pub fn probe(&self, hash: Hash, depth: u8, alpha: i32, beta: i32) -> (Option<i32>, Move) {
        let entry = &self.entries[hash.val as usize % self.size()];

        if !entry.is_empty() && entry.compare(&hash) {
            // if entry.depth() == depth {
            //     println!("{} {:?} d = {} a = {} b = {}", entry.score, entry.bound(), depth, alpha, beta);
            // }
            if entry.depth >= depth
                && match entry.bound() {
                    Bound::Lower => entry.score >= beta,
                    Bound::Upper => entry.score <= alpha,
                    Bound::Exact => true,
                } {
                return (Some(entry.score), Move::NULL);
            }

            return (None, entry.best_move);
        }
        (None, Move::NULL)
    }

    pub fn best_move(&self, hash: Hash) -> Option<Move> {
        let entry = &self.entries[hash.val as usize % self.size()];

        if !entry.is_empty() && entry.compare(&hash) && entry.best_move != Move::NULL {
            return Some(entry.best_move);
        }
        None
    }

    pub fn record(&mut self, board: &Board, score: i32, best_move: Move, depth: u8, bound: Bound) {
        let size = self.size();
        let entry = &mut self.entries[board.hash.val as usize % size];

        if entry.is_empty() || entry.depth <= depth || entry.ancient() {
            let info = (bound as u8) << 6;
            *entry = Entry {
                score: score,
                best_move: best_move,
                hash16: board.hash.sub(),
                depth: depth,
                info: info,
            };
        }
    }

    pub fn pv(&self, board: &Board) -> Vec<Move> {
        let mut pv = Vec::new();
        let mut visited = HashSet::new();
        self.pv_cycle_track(*board, &mut pv, &mut visited);
        pv
    }

    pub fn pv_cycle_track(
        &self,
        mut board: Board,
        pv: &mut Vec<Move>,
        visited: &mut HashSet<Hash>,
    ) {
        let mv = self.best_move(board.hash);

        if let Some(m) = mv {
            pv.push(m);
            board.make_move(m);

            if visited.insert(board.hash) {
                self.pv_cycle_track(board, pv, visited);
            }
        }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn set_ancient(&mut self) -> usize {
        let mut num = 0;
        for entry in &mut self.entries {
            if !entry.is_empty() {
                num += 1;
                entry.info |= 0b1;
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn add_and_retreive(){
        init();
        let table = Table::empty_mb(10);
        table.record(
    }
}
