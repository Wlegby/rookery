use std::vec;

// The chess Piece Types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    #[default]
    Empty,
}
impl Type {
    pub fn to_u16(&self) -> u16 {
        match self {
            Type::Empty => 0,
            Type::Pawn => 1,
            Type::Rook => 2,
            Type::Knight => 3,
            Type::Bishop => 4,
            Type::Queen => 5,
            Type::King => 6,
        }
    }
    pub fn from_u16(x: u16) -> Self {
        match x {
            1 => Type::Pawn,
            2 => Type::Rook,
            3 => Type::Knight,
            4 => Type::Bishop,
            5 => Type::Queen,
            6 => Type::King,
            _ => Type::Empty,
        }
    }
    pub fn get_move_offsets(&self) -> Vec<i32> {
        match self {
            Type::Rook => {
                vec![8, -8, 1, -1]
            }
            Type::Knight => {
                vec![21, -21, 19, -19, 12, -12, 8, -8]
            }
            Type::Bishop => {
                vec![11, -11, 9, -9]
            }
            Type::King => {
                vec![11, -11, 9, -9, 8, -8, 1, -1]
            }
            Type::Queen => {
                vec![11, -11, 9, -9, 8, -8, 1, -1]
            }
            _ => vec![],
        }
    }
}

// The chess Piece Colors
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub enum Color {
    Black,
    White,
    #[default]
    Empty,
}
impl Color {
    pub fn to_u16(&self) -> u16 {
        match self {
            Color::Black => 0,
            // 8 because the color is the fourth bit (8 in decimal)
            Color::White => 8,
            Color::Empty => 0,
        }
    }
    pub fn from_u16(x: u16) -> Self {
        match x {
            8 => Color::White,
            _ => Color::Black,
        }
    }
}

// The chess Pieces
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Piece {
    _type: Type,
    color: Color,
    has_moved: bool,
    position: usize,
}

impl Piece {
    pub fn new(_type: Type, color: Color, position: usize) -> Self {
        Self {
            _type,
            color,
            has_moved: false,
            position,
        }
    }
    pub fn new_detail(_type: Type, color: Color, has_moved: bool, position: usize) -> Self {
        Self {
            _type,
            color,
            has_moved,
            position,
        }
    }
    pub fn get_position(&self) -> usize {
        self.position
    }
    pub fn get_type(&self) -> Type {
        self._type
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn has_moved(&self) -> bool {
        self.has_moved
    }
    pub fn to_ascii(&self) -> char {
        let c = match self._type {
            Type::Pawn => 'p',
            Type::Rook => 'r',
            Type::Knight => 'n',
            Type::Bishop => 'b',
            Type::Queen => 'q',
            Type::King => 'k',
            Type::Empty => ' ',
        };

        if self.color == Color::White {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }
}
