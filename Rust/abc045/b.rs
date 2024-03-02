#[macro_use]
extern crate maplit;

fn main() {
    proconio::input! {
        sa: String,
        sb: String,
        sc: String
    }

    let mut table = hashmap! {
        Player::A => PileOfCards::new(&sa),
        Player::B => PileOfCards::new(&sb),
        Player::C => PileOfCards::new(&sc)
    };

    let mut turn = Player::A;
    loop {
        if let Some(pile) = table.get_mut(&turn) {
            if let Some(player) = pile.next() {
                turn = player;
            } else {
                println!("{}", turn.name());
                return;
            }
        } else {
            panic!("Something is wrong!");
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Player {
    A,
    B,
    C,
}

impl Player {
    fn name(&self) -> char {
        match self {
            Player::A => 'A',
            Player::B => 'B',
            Player::C => 'C',
        }
    }
}

struct PileOfCards {
    bytes: Vec<u8>,
    index: usize,
}

impl PileOfCards {
    fn new(s: &str) -> PileOfCards {
        PileOfCards {
            bytes: Vec::from(s.as_bytes()),
            index: 0,
        }
    }

    fn next(&mut self) -> Option<Player> {
        if let Some(byte) = self.bytes.get(self.index) {
            self.index += 1;
            match byte {
                b'a' => Some(Player::A),
                b'b' => Some(Player::B),
                b'c' => Some(Player::C),
                _ => None,
            }
        } else {
            None
        }
    }
}
