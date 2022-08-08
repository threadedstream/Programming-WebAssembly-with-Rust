#[macro_use]
extern crate lazy_static;

mod board;
mod game;

use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
use mut_static::MutStatic;

const PIECEFLAG_WHITE: u8 = 1 << 0;
const PIECEFLAG_BLACK: u8 = 1 << 1;
const PIECEFLAG_CROWNED: u8 = 1 << 2; 

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<GameEngine> = 
         MutStatic::from(GameEngine::new()) ;
}

extern {
    fn notify_piecemoved(fx: i32, fy: i32, tx: i32, ty: i32);
    fn notify_piececrowned(tx: i32, ty: i32);
}

impl Into<i32> for GamePiece {
    fn into(self) -> i32 {
        let mut value = match self.color {
            PieceColor::White => PIECEFLAG_WHITE,
            PieceColor::Black => PIECEFLAG_BLACK,
            _ => 0
        };
        if self.crowned {
            value |= PIECEFLAG_CROWNED;
        }
        value as i32
    }
}

pub extern "C" fn get_piece(x: i32, y: i32) -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    let piece = engine.get_piece(Coordinate(x as usize, y as usize));
    match piece {
        Ok(Some(p)) => p.into(),
        Ok(None) => -1,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();
    let mv = Move::new((fx as usize, fy as usize), (tx as usize, ty as usize));
    let res = engine.move_piece(&mv);
    match res {
        Ok(move_result) => {
            unsafe {
                notify_piecemoved(fx, fy, tx, ty);
            }
            if move_result.crowned {
                unsafe {
                    notify_piececrowned(tx, ty);
                }
            }
            1
        }
        Err(_) => 0
    }
}