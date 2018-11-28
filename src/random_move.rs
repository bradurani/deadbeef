use game::*;
use node::Node;
use rand::Rng;
use shakmaty::*;
use utils::*;

pub trait RandomMove {
    fn random_move<R: Rng>(&self, rng: &mut R) -> Move;
    fn make_random_move<R: Rng>(&mut self, rng: &mut R);
}

impl RandomMove for Chess {
    fn random_move<R: Rng>(&self, rng: &mut R) -> Move {
        choose_random(rng, &self.allowed_actions()).clone()
    }

    fn make_random_move<R: Rng>(&mut self, rng: &mut R) {
        let action = self.random_move(rng);
        self.make_move(&action);
    }
}

impl RandomMove for Node {
    fn random_move<R: Rng>(&self, rng: &mut R) -> Move {
        self.position.random_move(rng)
    }
    fn make_random_move<R: Rng>(&mut self, rng: &mut R) {
        let action = self.random_move(rng);
        self.make_move(&action);
    }
}
