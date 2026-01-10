use crate::piece::{Color, Piece, Type};

// So it is easier for me to know what exactly a certain variable is
pub type Position = usize;

// The individual tile saving the data
// Bits: 15: off the board, 14: is occupied, 6-5: What it's attacked by, 4: Has moved 3: Color, 2-0: Piece Type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile(u16);
impl Tile {
    pub fn empty(off_the_side: bool) -> Self {
        if off_the_side { Self(0x8000) } else { Self(0) }
    }
    pub fn new(x: u16) -> Self {
        Self(x)
    }
    // return a new Tile loaded from a Piece
    pub fn from_piece(p: &Piece) -> Self {
        let piece_type = p.get_type().to_u16();
        let color = p.get_color().to_u16();
        let has_moved = p.has_moved();

        // Occupied: 0x4000
        Self::new(0x4000 | if has_moved { 0b1_0000 } else { 0 } | color | piece_type)
    }
    pub fn is_on_board(&self) -> bool {
        self.0 & 0x8000 == 0
    }
    pub fn is_occupied(&self) -> bool {
        self.0 & 0x7999 == 0
    }
    pub fn piece_on(&self, p: Position) -> Option<Piece> {
        todo!();
        if self.is_occupied() {
            Some()
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::empty(false)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    // The array that saves the data of each Square
    mailbox: [Tile; 120],
    en_passant: Option<usize>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn tile(&self, p: Position) -> Tile {
        self.mailbox[p]
    }

    pub fn to_ascii(&self) -> String {
        let mut output = String::new();

        for j in 0..12 {
            for i in 0..10 {
                let idx = i + j * 10;
                if !self.mailbox[idx].is_on_board() {
                    output.push('#');
                } else {
                }
                output.push(' ');
            }
            output.push('\n');
        }

        output
    }

    // load the default starting position of chess
    pub fn new() -> Self {
        // Uppercase for White lowercase for Black
        let str = "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr";
        let mut position: Position = 0;
        let mut mailbox = [Tile::default(); 120];

        for _ in 0..21 {
            mailbox[position] = Tile::empty(true);
            position += 1;
        }

        for c in str.chars() {
            if c.is_alphabetic() {
                let piece_type = match c.to_ascii_lowercase() {
                    'r' => Type::Rook,
                    'n' => Type::Knight,
                    'b' => Type::Bishop,
                    'q' => Type::Queen,
                    'k' => Type::King,
                    'p' => Type::Pawn,
                    _ => Type::default(),
                };
                let color = if c.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                mailbox[position] = Tile::from_piece(&Piece::new(piece_type, color));
                position += 1;
            } else if c.is_numeric() {
                position += c.to_string().parse::<usize>().unwrap();
            } else if c == '/' {
                mailbox[position] = Tile::empty(true);
                position += 1;
                mailbox[position] = Tile::empty(true);
                position += 1;
            }
        }
        for _ in 0..21 {
            mailbox[position] = Tile::empty(true);
            position += 1;
        }

        Self {
            mailbox,
            en_passant: None,
        }
    }
}
