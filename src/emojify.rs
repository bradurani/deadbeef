use shakmaty::*;
use std::fmt;
use std::fmt::Write;

pub trait Emojify {
    fn emojify(&self) -> char;
}

pub trait DisplayEmojify {
    fn write_emoji(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl DisplayEmojify for Board {
    fn write_emoji(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).map(Rank::new).rev() {
            for file in (0..8).map(File::new) {
                let square = Square::from_coords(file, rank);
                f.write_char(self.piece_at(square).map_or('.', |piece| piece.emojify()))?;
                f.write_char(if file < File::H { ' ' } else { '\n' })?
            }
        }
        Ok(())
    }
}

impl Emojify for Piece {
    fn emojify(&self) -> char {
        match self.role {
            Role::Pawn => self.color.fold('♟', '♙'),
            Role::King => self.color.fold('♚', '♔'),
            Role::Queen => self.color.fold('♛', '♕'),
            Role::Bishop => self.color.fold('♝', '♗'),
            Role::Knight => self.color.fold('♞', '♘'),
            Role::Rook => self.color.fold('♜', '♖'),
        }
    }
}
