trait Emojify {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Emojify for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).map(Rank::new).rev() {
            for file in (0..8).map(File::new) {
                let square = Square::from_coords(file, rank);
                f.write_char(
                    self.piece_at(square)
                        .map_or('.', |piece| piece.emoji_char()),
                )?;
                f.write_char(if file < File::H { ' ' } else { '\n' })?;
            }
        }

        Ok(())
    }
}

impl Emojify for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.role {
            Role::Pawn => f.write_char(self.color.fold('♟', '♙'))?,
            Role::King => f.write_char(self.color.fold('♚', '♔'))?,
            Role::Queen => f.write_char(self.color.fold('♛', '♕'))?,
            Role::Bishop => f.write_char(self.color.fold('♝', '♗'))?,
            Role::Knight => f.write_char(self.color.fold('♞', '♘'))?,
            Role::Rook => f.write_char(self.color.fold('♜', '♖'))?,
        }
    }
}
