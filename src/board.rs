use crate::piece::{Color, Piece, Type};

// to have an index from 0-63 for the mailbox 0-119
const MAILBOX64: [Position; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68, 71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88, 91, 92, 93, 94, 95, 96, 97, 98,
];

// So it is easier for me to know what exactly a certain variable is
pub type Position = usize;

// The individual tile saving the data
// Bits: 15: off the board, 14: is occupied, 12-5: position 4: Has moved 3: Color, 2-0: Piece Type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile(u16);
impl Tile {
    pub fn empty(off_the_side: bool, position: Position) -> Self {
        if off_the_side {
            Self(0x8000 | (position * 2_usize.pow(6)) as u16)
        } else {
            Self(0)
        }
    }
    fn new(x: u16, position: usize) -> Self {
        Self(x | (position * 2_usize.pow(6)) as u16)
    }
    // return a new Tile loaded from a Piece
    pub fn from_piece(p: &Piece) -> Self {
        let piece_type = p.get_type().to_u16();
        let color = p.get_color().to_u16();
        let has_moved = p.has_moved();

        // Occupied: 0x4000
        Self::new(
            0x4000 | if has_moved { 0b1_0000 } else { 0 } | color | piece_type,
            p.get_position(),
        )
    }
    pub fn is_on_board(&self) -> bool {
        self.0 & 0x8000 == 0
    }
    pub fn is_occupied(&self) -> bool {
        self.0 & 0x4000 != 0
    }
    pub fn piece_on(&self) -> Option<Piece> {
        if self.is_occupied() {
            Some(Piece::new_detail(
                self.get_type(),
                self.get_color(),
                self.get_has_moved(),
                self.get_pos(),
            ))
        } else {
            None
        }
    }
    pub fn get_type(&self) -> Type {
        Type::from_u16(self.0 & 0b111)
    }
    pub fn get_color(&self) -> Color {
        Color::from_u16(self.0 & 0b1000)
    }
    pub fn get_has_moved(&self) -> bool {
        self.0 & 0b10000 != 0
    }
    fn get_pos(&self) -> usize {
        (self.0 & 0b1111111_00000) as usize
    }
    pub fn remove_piece(&mut self) {
        self.0 &= 0b1011111111110000
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    // The array that saves the data of each Square
    mailbox: [Tile; 120],
    side_to_move: Color,
    en_passant: Option<Position>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }
    pub fn make_move(&mut self, from: Position, to: Position) {
        if let Some(p) = self.tile_120(from).piece_on() {
            self.mailbox[to] = Tile::from_piece(&p);
            self.mailbox[from].remove_piece();
        }
        if self.side_to_move == Color::White {
            self.side_to_move = Color::Black
        } else {
            self.side_to_move = Color::White
        }
    }
    pub fn get_moves(&self, position: Position, moves: &mut Vec<usize>) {
        let p_type = self.tile_120(position).get_type();
        let p_color = self.tile_120(position).get_color();

        if p_type != Type::Pawn && p_type != Type::King && p_type != Type::Knight {
            for offset in p_type.get_move_offsets() {
                let next_p = (position as i32 + offset) as usize;
                let next_t = self.tile_120(next_p);

                if !next_t.is_on_board() {
                    continue;
                }
                if next_t.is_occupied() && next_t.get_color() == p_color {
                    continue;
                }
                if next_t.is_occupied() && next_t.get_color() != p_color {
                    moves.push(next_p);
                    continue;
                }
                self.get_recursive_moves(next_p, p_type, p_color, moves);
            }
        } else if p_type == Type::Knight || p_type == Type::King {
            for offset in p_type.get_move_offsets() {
                let next_p = (position as i32 + offset) as Position;
                let next_t = self.tile_120(next_p);

                if !next_t.is_on_board() {
                    continue;
                }
                if next_t.is_occupied() && next_t.get_color() == p_color {
                    continue;
                }
                moves.push(next_p);
            }
        }
    }
    fn get_recursive_moves(
        &self,
        position: Position,
        p_type: Type,
        p_color: Color,
        moves: &mut Vec<usize>,
    ) {
        for offset in p_type.get_move_offsets() {
            let next_p = (position as i32 + offset) as Position;
            let next_t = self.tile_120(next_p);

            if !next_t.is_on_board() {
                continue;
            }
            if next_t.is_occupied() && next_t.get_color() == p_color {
                continue;
            }
            if next_t.is_occupied() && next_t.get_color() != p_color {
                moves.push(next_p);
                continue;
            }
            self.get_recursive_moves(next_p, p_type, p_color, moves);
        }
    }
    pub fn tile_120(&self, p: Position) -> Tile {
        self.mailbox[p]
    }

    pub fn tile(&self, p: Position) -> Tile {
        self.mailbox[MAILBOX64[p]]
    }

    pub fn to_ascii(&self) -> String {
        let mut output = String::new();

        for j in 0..12 {
            for i in 0..10 {
                let idx = i + j * 10;
                if !self.mailbox[idx].is_on_board() {
                    output.push('+');
                } else {
                    if let Some(p) = self.mailbox[idx].piece_on() {
                        let c = p.to_ascii();
                        output.push(c);
                    } else {
                        output.push('·')
                    }
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
        let mut mailbox = [Tile::empty(false, 0); 120];

        for _ in 0..21 {
            mailbox[position] = Tile::empty(true, position);
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
                mailbox[position] = Tile::from_piece(&Piece::new(piece_type, color, position));
                position += 1;
            } else if c.is_numeric() {
                position += c.to_string().parse::<usize>().unwrap();
            } else if c == '/' {
                mailbox[position] = Tile::empty(true, position);
                position += 1;
                mailbox[position] = Tile::empty(true, position);
                position += 1;
            }
        }
        for _ in 0..21 {
            mailbox[position] = Tile::empty(true, position);
            position += 1;
        }

        Self {
            mailbox,
            side_to_move: Color::White,
            en_passant: None,
        }
    }
}
