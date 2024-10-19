use crate::player::Player;

/// we won't actually check the board for draw condition since that's pain in the ass,
/// instead we'll just consider it draw if we have reached this turns and no winner is yet decided
const MAX_TURNS: u8 = 9;

pub enum MoveError {
    InvalidPlayer,
    InvalidPosition,
    PositionOccupied,
}

pub enum ValidateResult {
    NextTurn,
    Winner(Player),
    Draw,
}

pub struct Move {
    player: Player,
    position: (usize, usize),
}

pub struct Board {
    cells: [[Option<Player>; 3]; 3],
    turns: u8,
}

macro_rules! validate_rules {
    (
        $_self:ident @ $(
            [$rows1:literal, $cells1:literal] | [$rows2:literal, $cells2:literal] | [
                $rows3:literal,
                $cells3:literal
            ]
        ),*
    ) => {
        $(

            if
             $_self.cells[$rows1][$cells1].is_some_and(|p|
                p.eq(&$_self.cells[$rows2][$cells2].or(Some(Player::default())).unwrap())
            ) &&
            $_self.cells[$rows2][$cells2].is_some_and(|p|
                p.eq(&$_self.cells[$rows3][$cells3].or(Some(Player::default())).unwrap())
            )
        {
            return ValidateResult::Winner($_self.cells[$rows1][$cells1].expect("player expected"));
        }



        )*
    };
}
impl Board {
    ///  basically try to find a winner
    pub fn validate(&mut self) -> ValidateResult {
        self.turns += 1;
        
        // stack cache
        let turns = self.turns;

        let mut _self = self;

        validate_rules!(
            _self @ 
            
            //               |            |
            //               |            |
            //               |            |
            //        x      |     x      |     x
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |

            [0, 0] | [0, 1] | [0, 2],

            //               |            |
            //               |            |
            //               |            |
            //        x      |            |     
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //        x      |            |
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //       x       |            |
            //               |            |
            //               |            |
            //               |            |
            [0, 0] | [1, 0] | [2, 0],

            //               |            |
            //               |            |
            //               |            |
            //        x      |            |     
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //               |     x      |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |   x
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            [0, 0] | [1, 1] | [2, 2],

            //               |            |
            //               |            |
            //               |            |
            //               |     x      |     
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |      x     |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |       x    |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |

            [0, 1] | [1, 1] | [2, 1],

            //               |            |
            //               |            |
            //               |            |
            //               |            |     x
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |     x
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |     x
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |

            [0, 2] | [1, 2] | [2, 2],

            //               |            |
            //               |            |
            //               |            |
            //               |            |     
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //         x     |  x         | x
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            [1, 0] | [1, 1] | [1, 2],

            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //        x      |     x      |     x
            //               |            |
            //               |            |
            //               |            |
            [2, 0] | [2, 1] | [2, 2],

            //               |            |
            //               |            |
            //               |            |
            //               |            |
            //               |            |    x
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //               |       x    |
            //               |            |
            //               |            |
            //  _____________|____________|_______________
            //               |            |
            //               |            |
            //               |            |
            //        x      |            |     
            //               |            |
            //               |            |
            //               |            |
            [2, 0] | [1, 1] | [0, 2]
            
        );


        if turns == MAX_TURNS {
            return ValidateResult::Draw;
        }

        ValidateResult::NextTurn
    }
}
