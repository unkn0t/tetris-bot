use super::board::FigureType;
use super::solver::Rotation;

#[derive(Clone)]
pub struct Figure {
    matrix: [u32; 5],
    sides: u32, // top, left, bottom, right 
}

impl From<FigureType> for Figure {
    fn from(ty: FigureType) -> Self {
        let (matrix, sides) = match ty {
            FigureType::I => ([4, 4, 4, 4, 0], 0x01000200),
            FigureType::O => ([0, 6, 6, 0, 0], 0x00000101),
            FigureType::L => ([0, 6, 4, 4, 0], 0x01000101),
            FigureType::J => ([0, 12, 4, 4, 0], 0x01010100),
            FigureType::S => ([0, 0, 12, 6, 0], 0x01010001),
            FigureType::Z => ([0, 0, 6, 12, 0], 0x01010001),
            FigureType::T => ([0, 0, 14, 4, 0], 0x01010001),
        };

        Self { matrix, sides }
    }
}

impl Figure {
    pub fn get_row(&self, y: i32) -> u32 {
        self.matrix[y as usize]
    }
    
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::None => {},
            Rotation::Left => self.rotate_template(Self::left_coord, 8),
            Rotation::Flip => self.rotate_template(Self::flip_coord, 16),
            Rotation::Right => self.rotate_template(Self::right_coord, 24),
        }
    }
    
    pub fn left(&self) -> i32 {
        ((self.sides >> 16) & 255) as i32
    }
    
    pub fn bottoms(&self) -> [i32; 5] {
        let mut bottoms = [0; 5];
        for x in 2-self.left()..=2+self.right() {
            bottoms[x as usize] += ((self.matrix[0] >> (4 - x)) & 1) as i32;
            bottoms[x as usize] += ((self.matrix[1] >> (4 - x)) & 1) as i32;
        }
        bottoms
    }
    
    pub fn tops(&self) -> [i32; 5] {
        let mut tops = [0; 5];
        for x in 2-self.left()..=2+self.right() {
            tops[x as usize] += ((self.matrix[3] >> (4 - x)) & 1) as i32;
            tops[x as usize] += ((self.matrix[4] >> (4 - x)) & 1) as i32;
        }
        tops
    }
    
    pub fn right(&self) -> i32 {
        (self.sides & 255) as i32
    }
    
    fn left_coord(x: usize, y: usize) -> (usize, usize) {
        (y, 4 - x)        
    }

    fn flip_coord(x: usize, y: usize) -> (usize, usize) {
        (4 - x, 4 - y)        
    }

    fn right_coord(x: usize, y: usize) -> (usize, usize) {
        (4 - y, x)        
    }

    fn rotate_template(&mut self, rotate_coord: fn(usize, usize) -> (usize,usize), rot: u32) { 
        let mut temp = [0; 5];

        for y in 0..5 {
            for x in 0..5 {
                let bit = (self.matrix[y] >> x) & 1;
                let (new_x, new_y) = rotate_coord(x, y);
                temp[new_y] |= bit << new_x;
            }
        }

        self.matrix = temp;
        self.sides = self.sides.rotate_right(rot);
    }

    // pub fn visualize(&self) {
    //     let mut result = String::new();
    //
    //     for y in 0..FIGURE_SIZE {
    //         for x in 0..FIGURE_SIZE {
    //             if self.get(x as i32 - 2, 2 - y as i32) == 1 {
    //                 result.push('#');
    //             } else {
    //                 result.push('.');
    //             }
    //         }
    //         result.push('\n');
    //     }
    //
    //     println!("{result}");
    // }    
}

