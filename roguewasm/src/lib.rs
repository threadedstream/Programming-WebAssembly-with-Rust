#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(module = "./index")]
    fn stats_updated(stats: JsValue);

    pub type Display;

    #[wasm_bindgen(method, structural, js_namespace = ROT)]
    fn draw(this: &Display, x: i32, y: i32, ch: &str);

    #[wasm_bindgen(method, structural, js_name = draw, js_namespace=ROT)]
    fn draw_color(this: &Display, x: i32, y: i32, ch: &str, color: &str);
}

#[wasm_bindgen]
pub struct Engine {
    display: Display,
    points: HashMap<GridPoint, String>,
    prize_location: Option<GridPoint>
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(display: Display) -> Self {
        Engine {
            display: display,
            points: HashMap::new(),
            prize_location: None,
        }
    }

    pub fn on_dig(&mut self, x: i32, y: i32, val: i32) {
        if val == 0 {
            let pt = GridPoint { x, y };
            self.points.insert(pt, ".".to_owned());
        }
    }

    pub fn draw_map(&self) {
        for (k, v) in &self.points {
            self.display.draw(k.x, k.y, &v);
        }
    }
}

#[derive(Serialize)]
pub struct Stats {
    pub hitpoints: i32,
    pub max_hitpoints: i32,
    pub moves: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
struct GridPoint {
    pub x: i32, 
    pub y: i32,
}

#[wasm_bindgen]
pub struct PlayerCore {
    loc: GridPoint,
    moves: i32,
    display: Display,
    hp: i32,
    max_hp: i32,
    icon: String,
    color: String,
}

#[wasm_bindgen]
impl PlayerCore {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32, icon: &str, color: &str, display: Display) -> Self {
        PlayerCore {
            loc: GridPoint { x, y },
            moves: 0,
            display: display,
            hp: 100,
            max_hp: 100,
            icon: icon.to_owned(),
            color: color.to_owned(),
        }
    }

    pub fn x(&self) -> i32 {
        self.loc.x
    }

    pub fn y(&self) -> i32 {
        self.loc.y
    }

    pub fn draw(&self) {
        &self.display.draw_color(self.loc.x, self.loc.y, &self.icon, &self.color);
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.loc = GridPoint {x, y};
        self.draw();

        self.moves += 1;
        self.emit_stats();
    }

    pub fn emit_stats(&self) {
        let stats = Stats {
            hitpoints: self.hp,
            max_hitpoints: self.max_hp,
            moves: self.moves,
        };
        stats_updated(JsValue::from_serde(&stats).unwrap());
    }

    pub fn take_damage(&mut self, hits: i32) -> i32 {
        self.hp = self.hp - hits;
        self.emit_stats();
        self.hp
    }
}