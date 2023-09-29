use super::board::*;
use super::figure::Figure;
use std::time::Duration;

pub const SBOARD_HEIGHT: usize = 18;
pub const SBOARD_WIDTH: usize = 18;
pub const SBOARD_WIDTH_I32: i32 = 18;
pub const SBOARD_HEIGHT_I32: i32 = 18;

pub struct Simulator {
    glass: Glass,
}

impl Simulator {
    const FULL_ROW: u32 = ((1 << SBOARD_WIDTH) - 1) << (BOARD_WIDTH - SBOARD_WIDTH);

    pub fn new(glass: &Glass) -> Self {
        Self { 
            glass: glass.clone(),
        }
    }
    
    pub fn valid_moves(&mut self, figure: &Figure) -> Vec<Point> {
        let mut moves = Vec::with_capacity(SBOARD_WIDTH);
        
        let heights = self.cols_heights();
        let bottoms = figure.bottoms();

        for x in figure.left()..SBOARD_WIDTH_I32 - figure.right() {
            let mut y = 0;
            for t in -figure.left()..=figure.right() {
                y = y.max(heights[(x + t) as usize] + bottoms[(t + 2) as usize]);
            }
            
            if y >= 15 {
                continue;
            }

            let center = Point::new(x, y);

            if !self.intersect_figure(figure, center) { 
                moves.push(center);
            }
        }

        moves
    }
    
    pub fn toggle_figure(&mut self, figure: &Figure, center: Point) {
        let radius = 2;
        let bottom_row = 0.max(center.y - radius);
        let top_row = SBOARD_HEIGHT_I32.min(center.y + radius + 1);

        for y in bottom_row..top_row {
            let row = self.glass.get_row(y as usize);
            let mut figure_row = figure.get_row(y + radius - center.y);
            figure_row <<= BOARD_WIDTH_I32 - center.x - 1;
            figure_row >>= radius;
            self.glass.set_row(y as usize, row ^ figure_row);
        }
    }

    pub fn visualize(&self, ms: u64) {
        self.glass.visualize();
        std::thread::sleep(Duration::from_millis(ms));
    }
    
    pub fn evaluate(&self) -> f32 {
        const A: f32 = -0.51;
        const B: f32 = 0.70;
        const C: f32 = -0.35;
        const D: f32 = -0.16;

        let cols_heights = self.cols_heights();
        let holes_count = self.count_holes() as f32;
        let aggregate_height = self.cols_heights().iter().fold(0, |acc, h| acc + h) as f32;
        let completed_lines = self.completed_lines();

        let mut bumpiness = 0.0;

        for x in 1..cols_heights.len() {
            bumpiness += (cols_heights[x] as f32 - cols_heights[x - 1] as f32).abs();
        }

        A * aggregate_height + B * completed_lines as f32 + C * holes_count + D * bumpiness
    }

    fn intersect_figure(&self, figure: &Figure, center: Point) -> bool {
        let radius = 2;
        let bottom_row = 0.max(center.y - radius);
        let top_row = SBOARD_HEIGHT_I32.min(center.y + radius + 1);

        for y in bottom_row..top_row {
            let row = self.glass.get_row(y as usize);
            let mut figure_row = figure.get_row(y + radius - center.y);
            figure_row <<= BOARD_WIDTH_I32 - center.x - 1;
            figure_row >>= radius;
            if (row & figure_row) != 0 {
                return true;
            }
        }

        false
    } 
    
    pub fn completed_lines(&self) -> u32 {
        let mut result = 0;

        for y in 0..SBOARD_HEIGHT {
            let row = self.glass.get_row(y);
            result += (row == Self::FULL_ROW) as u32;
        }

        result
    }

    fn cols_heights(&self) -> Vec<i32> {
        let mut result = vec![0; SBOARD_WIDTH];
        for x in 0..SBOARD_WIDTH {
            result[x] = (u32::BITS - self.glass.get_col(x).leading_zeros()) as i32;
        }
        result
    }

    fn count_holes(&self) -> u32 {
        let mut result = 0;
        for x in 0..SBOARD_WIDTH {
            let col = self.glass.get_col(x);
            result += col.count_zeros() - col.leading_zeros();
        }
        result
    }
}
