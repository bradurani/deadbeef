extern crate deadbeef;
extern crate shakmaty;

use helpers::*;

mod helpers;

#[test]
fn avoids_queen_or_knight_trap() {
    // 0xDEADBEEF vs Sunfish
    assert_move(
        "r2qr1k1/pp3ppp/3b1n2/n2PpQ2/2p5/2N1PN2/PP3PPP/R1B2RK1 w - - 1 14",
        "e3e4",
    );
}

#[test]
fn finds_queen_or_knight_trap() {
    // 0xDEADBEEF vs Sunfish
    assert_move(
        "r2qr1k1/pp3ppp/3b1n2/n2PpQ2/2p5/2N1PN2/PP3PPP/R1BR2K1 b - - 2 14",
        "e5e4",
    );
}

#[test]
fn sac_knight_to_avoid_queen_trap() {
    assert_move(
        "rn1qkb1r/pp3ppp/2p1bn2/1B1pQ3/3Pp3/2N1P3/PPP2PPP/R1B1K1NR w KQkq -",
        "c3d5",
    );
}
