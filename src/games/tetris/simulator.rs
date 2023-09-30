use super::board::*;
use super::figure::Figure;
use std::time::Duration;

static mut SET_ONES: [u8; 1 << BOARD_WIDTH] = [0; 1 << BOARD_WIDTH];

#[derive(Clone)]
pub struct Simulator {
    glass: Glass,
    heights: [i32; BOARD_WIDTH],
}

impl Simulator {
    const FULL_ROW: u32 = (1 << BOARD_WIDTH) - 1;

    pub fn new(glass: &Glass) -> Self {
        for x in 0..=Self::FULL_ROW {
            unsafe { SET_ONES[x as usize] = x.count_ones() as u8; }
        }

        Self { 
            glass: glass.clone(),
            heights: [0; BOARD_WIDTH],
        }
    }
    
    pub fn valid_moves(&mut self, figure: &Figure) -> Vec<Point> {
        let mut moves = Vec::with_capacity(BOARD_WIDTH);
        
        // let heights = self.cols_heights();
        let bottoms = figure.bottoms();

        for x in figure.left()..BOARD_WIDTH_I32 - figure.right() {
            let mut y = 0;
            for t in -figure.left()..=figure.right() {
                y = y.max(self.heights[(x + t) as usize] + bottoms[(t + 2) as usize]);
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
    
    fn toggle_figure(&mut self, figure: &Figure, center: Point) {
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

    pub fn place_figure(&mut self, figure: &Figure, center: Point) {
        self.toggle_figure(figure, center);
        
        let tops = figure.tops();
            
        for t in -figure.left()..=figure.right() {
            self.heights[(center.x + t) as usize] = center.y + tops[(t + 2) as usize];
        }
    }

    pub fn unplace_figure(&mut self, figure: &Figure, center: Point) {
        self.toggle_figure(figure, center);
        self.recalc_heights();
    }

    pub fn visualize(&self, ms: u64) {
        self.glass.visualize();
        std::thread::sleep(Duration::from_millis(ms));
    }
    
    pub fn evaluate(&self) -> i64 {
        let mut under_mask: u32 = 0;
        let mut ln_mask: u32 = 0;
        let mut rn_mask: u32 = 0;
        let mut max_y: i64 = BOARD_HEIGHT as i64 - 1;
        let mut holes_count = 0; 

        while max_y >= 0 as i64 && self.glass.get_row(max_y as usize) == 0 {
            max_y -= 1;
        }

        for y in (0..=max_y).rev() {
            let filled = self.glass.get_row(y as usize);
            let line = !filled & Self::FULL_ROW;

            under_mask |= filled;
            ln_mask |= filled << 1;
            rn_mask |= filled >> 1;
           
            let side_factor = (y + 2) * (y + 2);
            let inside_factor = side_factor * side_factor;
            unsafe {
                holes_count += inside_factor * SET_ONES[(under_mask & line) as usize] as i64;
                holes_count += side_factor * SET_ONES[(ln_mask & line) as usize] as i64;
                holes_count += side_factor * SET_ONES[(rn_mask & line) as usize] as i64;
            }
        }

        -holes_count
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
    
    fn recalc_heights(&mut self) {
        for x in 0..BOARD_WIDTH {
            self.heights[x] = (u32::BITS - self.glass.get_col(x).leading_zeros()) as i32;
        }
    }
}
