use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::White,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) { 
        // setting up white pieces
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
        .iter()
        .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
        .map(|(a,b)| (*a as usize, *b as usize))
        .for_each(|(x, y)| {
            self.board[x][y] = Some(GamePiece::new(PieceColor::White));
        });

        // setting up black pieces
        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
        .iter()
        .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
        .map(|(a, b)| (*a as usize, *b as usize))
        .for_each(|(x, y)| {
            self.board[x][y] = Some(GamePiece::new(PieceColor::Black));
        });
    } 

    fn is_valid_coordinate(&self, loc: &Coordinate) -> bool {
        let Coordinate(x, y) = *loc;
        x >= 0 && x <= 7 && y >= 0 && y <= 7
    }

    pub fn get_piece(&self, loc: Coordinate) -> Result<Option<GamePiece>, ()> {
        if !self.is_valid_coordinate(&loc) {
            return Err(());
        }
        Ok(self.board[loc.0][loc.1])
    }

    fn midpiece_coordinate(&mut self, fx: usize, fy: usize, tx: usize, ty: usize) -> Option<Coordinate> {
        None
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();

        if !legal_moves.contains(mv) {
            return Err(());
        }

        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        let piece = self.board[fx][fy].unwrap();
        let midpiece_coordinate = self.midpiece_coordinate(fx, fy, tx, ty);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None;
        }
        
        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;

        let crowned = if self.should_crown(piece, mv.to) {
            self.crown_piece(mv.to);
            true 
        } else {
            false
        };

        self.advance_turn();
        
        Ok(MoveResult{
            mv: mv.clone(),
            crowned: crowned,
        })
    }
    
    fn advance_turn(&self)  {
    }

    fn should_crown(&self, piece: GamePiece, loc: Coordinate) -> bool {
        let Coordinate(x, y) = loc;
        match piece.color {
            PieceColor::White => y == 7 && (x == 0 || x == 2 || x == 4 || x == 6),
            PieceColor::Black => y == 0 && (x == 1 || x == 3 || x == 5 || x == 7),
        }
    }

    fn crown_piece(&mut self, loc: Coordinate) {
        let piece = self.get_piece(loc);
        match piece {
            Ok(Some(mut p)) => {
                p.crowned = true;
                let Coordinate(x, y) = loc;
                self.board[x][y] = Some(p);
            },
            Ok(None) => (),  
            Err(_) => ()
        }
    }
    
    fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }
        moves 
    }

    // trace path from "to" to "from"
    // if there's a piece, then one may bravely state that it's a valid jump
    fn valid_jump(&self, piece: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        // what does valid jump really mean
        // x , x - 2 -> 2
        // y , y - 2 -> 2 
        let Coordinate(fx, fy) = *from;
        let Coordinate(tx, ty) = *to;
        let mut target_x = 0;
        let mut target_y = 0;
        let x_dir = tx - fx;
        let y_dir = ty - fy;
        if x_dir > 0 {
            target_x = fx + 1;
        } else {
            target_x = fx - 1;
        }

        if y_dir > 0 {
            target_y = fy + 1;
        } else {
            target_y = fy - 1;
        }
        if let Some(_) = self.board[target_x][target_y] {
            true
        } else {
            false
        }
    }

    fn valid_move(&self, _piece: &GamePiece, _from: &Coordinate, to: &Coordinate) -> bool {
        let Coordinate(tx, ty) = *to;   
        if tx <= 7 && ty <= 7 {
            if let Some(_) = self.board[tx][ty] {
                return false;
            } else {
                return true;
            }
        }
        false
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(piece) = self.board[x][y] {
            let mut jumps= loc 
            .possible_jumps()
            .filter(|t| self.valid_jump(&piece, &loc, &t))
            .map(|ref t| Move {
                from: loc.clone(),
                to: t.clone(),
            }).collect::<Vec<Move>>();

            let mut moves = loc
            .possible_moves()
            .filter(|t| self.valid_move(&piece, &loc, &t))
            .map(|ref t| Move {
                from: loc.clone(),
                to: t.clone(),
            }).collect();

            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }
}