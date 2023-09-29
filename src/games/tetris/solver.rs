use crate::engine;
use super::figure::Figure;
use super::commands::Command;
use super::board::*;
use super::simulator::Simulator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation {
    None,
    Left,
    Right,
    Flip,
}

pub struct Solver {
    simulator: Simulator,
    next_figures: [FigureType; 3],
    center: Point,
}   

impl From<Rotation> for Command {
    fn from(rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => Command::Down,
            Rotation::Left => Command::RotateLeft,
            Rotation::Right => Command::RotateRight,
            Rotation::Flip => Command::Flip,
        }
    }
}

impl Solver {
    const ROTATIONS: [&'static [Rotation]; 7] = [
        &[Rotation::None, Rotation::Left],
        &[Rotation::None],
        &[Rotation::None, Rotation::Left, Rotation::Flip, Rotation::Right],
        &[Rotation::None, Rotation::Left, Rotation::Flip, Rotation::Right],
        &[Rotation::None, Rotation::Left],
        &[Rotation::None, Rotation::Left], 
        &[Rotation::None, Rotation::Left, Rotation::Flip, Rotation::Right],
    ];
   
    pub fn new() -> Self {
        Self { 
            simulator: Simulator::new(&Glass::empty()), 
            next_figures: [FigureType::I; 3],
            center: Point::new(0, 0),
        }
    }

    fn find_best_move(&mut self) -> (i32, Rotation) {
        let mut best_move = (0, Rotation::None);
        let mut best_evaluation = f32::MIN;

        let figure_type = self.next_figures[0];
        let figure = Figure::from(figure_type);
        self.simulator.toggle_figure(&figure, self.center);

        for rotation in Self::ROTATIONS[figure_type as usize] {
            let mut figure = figure.clone();
            figure.rotate(*rotation);

            for mov in self.simulator.valid_moves(&figure) {
                self.simulator.toggle_figure(&figure, mov);
                
                let evaluation = self.search(1);

                if evaluation > best_evaluation {
                    best_evaluation = evaluation;
                    best_move = (mov.x - self.center.x, *rotation);
                }
                    
                self.simulator.toggle_figure(&figure, mov);
            }
        }

        best_move
    }

    fn search(&mut self, depth: usize) -> f32 {
        let mut best_evaluation = f32::MIN;

        let figure_type = self.next_figures[depth];
        let figure = Figure::from(figure_type);
        
        for rotation in Self::ROTATIONS[figure_type as usize] {
            let mut figure = figure.clone();
            figure.rotate(*rotation);

            for mov in self.simulator.valid_moves(&figure) {
                self.simulator.toggle_figure(&figure, mov);
                
                let evaluation = if depth == 2 {
                    self.simulator.evaluate()
                } else {
                    self.search(depth + 1)
                };

                if evaluation > best_evaluation {
                    best_evaluation = evaluation;
                }
                    
                self.simulator.toggle_figure(&figure, mov);
            }
        }
        
        best_evaluation
    }
}

impl engine::Solver<Command, Board> for Solver {
    fn start(&mut self, commands: &mut engine::Commands<Command>) {
        commands.add(Command::Clear)
    }

    fn solve(&mut self, commands: &mut engine::Commands<Command>, board: &Board) {
        let timer = std::time::Instant::now();

        self.simulator = Simulator::new(board.glass());
        self.next_figures[0] = board.current_figure_type();
        self.center = board.current_figure_point();
        self.next_figures[1..].clone_from_slice(&board.future_figures_types()[..2]);
       
        let factor = self.simulator.completed_lines() + self.simulator.semicompleted_lines() / 2;
        if factor >= 4 && board.current_figure_type() == FigureType::I {
            for _ in 0..BOARD_WIDTH {
                commands.add(Command::Right);
            }
        } else {
            let (offset, rotation) = self.find_best_move();
            
            if rotation != Rotation::None {
                commands.add(rotation.into());
            }
            
            let command = if offset < 0 { Command::Left } else { Command::Right }; 
            for _ in 0..offset.abs() {
                commands.add(command);
            }
        }

        for _ in 0..BOARD_HEIGHT {
            commands.add(Command::Down);
        }

        println!("Elapsed time: {}ms", timer.elapsed().as_millis());
    }
}

