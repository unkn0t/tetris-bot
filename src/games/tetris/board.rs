use std::str::FromStr;

use serde::Deserialize;
use serde::de::{Deserializer, Error};

pub const BOARD_WIDTH: usize = 18;
pub const BOARD_HEIGHT: usize = 18;
pub const BOARD_WIDTH_I32: i32 = 18;
pub const BOARD_HEIGHT_I32: i32 = 18;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LevelProgress {
    pub total: i32,
    pub current: i32,
    pub last_passed: i32,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FigureType {
    I, O, L, J, S, Z, T,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Glass {
    layers: [u32; BOARD_HEIGHT],
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    current_figure_type: FigureType,
    future_figures: Vec<FigureType>,
    layers: Vec<Glass>,
    current_figure_point: Point,
    _level_progress: LevelProgress,
}

impl<'de> Deserialize<'de> for Glass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        let data: &str = Deserialize::deserialize(deserializer)?;
        let glass = data.parse().map_err(|err| D::Error::custom(err))?;
        Ok(glass)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

impl Glass {
    pub fn empty() -> Self {
        Self { layers: [0; BOARD_HEIGHT] }
    }

    pub fn get_row(&self, y: usize) -> u32 {
        self.layers[y]
    }

    pub fn set_row(&mut self, y: usize, val: u32) {
        self.layers[y] = val;
    }

    pub fn get_col(&self, x: usize) -> u32 {
        let mut col = 0;

        for y in 0..BOARD_HEIGHT {
            col |= ((self.layers[y] >> (BOARD_WIDTH - x - 1)) & 1) << y;
        }
        
        col
    }
        
    pub fn visualize(&self) {
        let mut result = String::new();

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if ((self.get_row(BOARD_HEIGHT - y - 1) >> BOARD_WIDTH - x - 1) & 1) == 1 {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }

        println!("{result}");
    }
}

impl Board {
    pub fn current_figure_type(&self) -> FigureType {
        self.current_figure_type
    }

    pub fn future_figures_types(&self) -> &[FigureType] {
        &self.future_figures
    }

    pub fn glass(&self) -> &Glass {
        &self.layers[0]
    }

    pub fn current_figure_point(&self) -> Point {
        self.current_figure_point
    }
}

impl From<String> for Board {
    fn from(text: String) -> Self {
        let text = text.trim_start_matches("board=");
        serde_json::from_str(&text).unwrap()
    }
}

impl FromStr for Glass {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        if data.len() != BOARD_WIDTH * BOARD_HEIGHT {
            return Err("Board sizes are incorrect!".into());
        }

        let mut layers = [0; BOARD_HEIGHT];
        let mut chars = data.chars();

        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if chars.next().unwrap() != '.' {
                   layers[BOARD_HEIGHT - row - 1] |= 1 << (BOARD_WIDTH - col - 1); 
                }
            }
        }

        Ok(Self { layers }) 
    }
}
