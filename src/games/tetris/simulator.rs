use super::board::*;
use super::figure::Figure;
use std::time::Duration;

#[derive(Clone)]
pub struct Simulator {
    glass: Glass,
    set_ones: Vec<u8>,
}

impl Simulator {
    const FULL_ROW: u32 = (1 << BOARD_WIDTH) - 1;

    pub fn new(glass: &Glass) -> Self {
        let mut set_ones = vec![0; Self::FULL_ROW as usize + 1];

        for x in 0..=Self::FULL_ROW {
            set_ones[x as usize] = x.count_ones() as u8;
        }

        Self { 
            glass: glass.clone(),
            set_ones,
        }
    }
    
    pub fn valid_moves(&mut self, figure: &Figure) -> Vec<Point> {
        let mut moves = Vec::with_capacity(BOARD_WIDTH);
        
        let heights = self.cols_heights();
        let bottoms = figure.bottoms();

        for x in figure.left()..BOARD_WIDTH_I32 - figure.right() {
            let mut y = 0;
            for t in -figure.left()..=figure.right() {
                y = y.max(heights[(x + t) as usize] + bottoms[(t + 2) as usize]);
            }
            
            if y >= 16 {
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
        let top_row = BOARD_HEIGHT_I32.min(center.y + radius + 1);

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
    
    pub fn evaluate(&self) -> i64 {
        let mut under_mask: u32 = 0;
        let mut ln_mask: u32 = 0;
        let mut rn_mask: u32 = 0;
        let mut min_y = 0;
        let mut holes_count = 0; 

        while min_y < BOARD_HEIGHT as i64 && self.glass.get_row(min_y as usize) == Self::FULL_ROW {
            min_y += 1;
        }

        for y in min_y..BOARD_HEIGHT as i64 {
            let filled = self.glass.get_row(y as usize);
            let line = !filled & Self::FULL_ROW;

            under_mask |= filled;
            ln_mask |= filled << 1;
            rn_mask |= filled >> 1;
            
            holes_count += Self::score_fun(3, y) * self.set_ones[(under_mask & line) as usize] as i64;
            holes_count += Self::score_fun(2, y) * self.set_ones[(ln_mask & line) as usize] as i64;
            holes_count += Self::score_fun(2, y) * self.set_ones[(rn_mask & line) as usize] as i64;
        }

        holes_count
    }

    fn score_fun(p: i64, y: i64) -> i64 {
        let mut result = 1;
        for _ in 0..p {
            result *= BOARD_HEIGHT_I32 as i64 - y;
        }
        result
    }

    fn intersect_figure(&self, figure: &Figure, center: Point) -> bool {
        let radius = 2;
        let bottom_row = 0.max(center.y - radius);
        let top_row = BOARD_HEIGHT_I32.min(center.y + radius + 1);

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
    
    // pub fn completed_lines(&self) -> u32 {
    //     let mut result = 0;
    //
    //     for y in 0..BOARD_HEIGHT {
    //         let row = self.glass.get_row(y);
    //         result += (row == Self::FULL_ROW) as u32;
    //     }
    //
    //     result
    // }
    //
    fn cols_heights(&self) -> Vec<i32> {
        let mut result = vec![0; BOARD_WIDTH];
        for x in 0..BOARD_WIDTH {
            result[x] = (u32::BITS - self.glass.get_col(x).leading_zeros()) as i32;
        }
        result
    }
    //
    // fn count_holes(&self) -> u32 {
    //     let mut result = 0;
    //     for x in 0..BOARD_WIDTH {
    //         let col = self.glass.get_col(x);
    //         result += col.count_zeros() - col.leading_zeros();
    //     }
    //     result
    // }
}
