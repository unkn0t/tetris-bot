use crate::engine;
use super::figure::Figure;
use super::commands::Command;
use super::board::*;
use super::simulator::Simulator;

use rayon::prelude::*;

const SIMULATION_DEPTH: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    None,
    Left,
    Right,
    Flip,
}

pub struct Solver {
    next_figures: [FigureType; SIMULATION_DEPTH],
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
            next_figures: [FigureType::I; SIMULATION_DEPTH],
            center: Point::new(0, 0),
        }
    }

    fn find_best_move(&self, glass: &Glass) -> (i32, Rotation) {
        let mut best_move = (0, Rotation::None);
        let mut best_evaluation = i64::MIN;

        let figure_type = self.next_figures[0];
        let figure = Figure::from(figure_type);
        let mut simulator = Simulator::new(glass);
        simulator.unplace_figure(&figure, self.center);
        
        for rotation in Self::ROTATIONS[figure_type as usize] {
            let mut figure = figure.clone();
            figure.rotate(*rotation);

            let mut sim_clone = simulator.clone();
            let valid_moves = sim_clone.valid_moves(&figure);
            if let Some((evaluation, mov)) = valid_moves.into_par_iter().map_with(sim_clone, |simulator, mov| {
                simulator.place_figure(&figure, mov);
                let evaluation = self.search(simulator, 1);                    
                simulator.unplace_figure(&figure, mov);
                (evaluation, (mov.x - self.center.x, *rotation))
            }).max() {
                if evaluation > best_evaluation {
                    best_evaluation = evaluation;
                    best_move = mov;
                }
            }
        }

        best_move
    }

    fn search(&self, simulator: &mut Simulator, depth: usize) -> i64 {
        let mut best_evaluation = i64::MIN;

        let figure_type = self.next_figures[depth];
        let figure = Figure::from(figure_type);
        
        for rotation in Self::ROTATIONS[figure_type as usize] {
            let mut figure = figure.clone();
            figure.rotate(*rotation);

            for mov in simulator.valid_moves(&figure) {
                simulator.place_figure(&figure, mov);
                
                let evaluation = if depth == SIMULATION_DEPTH - 1 {
                    simulator.evaluate()
                } else {
                    self.search(simulator, depth + 1)
                };

                if evaluation > best_evaluation {
                    best_evaluation = evaluation;
                }
                    
                simulator.unplace_figure(&figure, mov);
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

        self.next_figures[0] = board.current_figure_type();
        self.center = board.current_figure_point();
        self.next_figures[1..].clone_from_slice(&board.future_figures_types()[..SIMULATION_DEPTH - 1]);
       
        let (offset, rotation) = self.find_best_move(board.glass());
        
        if rotation != Rotation::None {
            commands.add(rotation.into());
        }
        
        let command = if offset < 0 { Command::Left } else { Command::Right }; 
        for _ in 0..offset.abs() {
            commands.add(command);
        }

        for _ in 0..BOARD_HEIGHT {
            commands.add(Command::Down);
        }

        println!("Elapsed time: {}ms", timer.elapsed().as_millis());
    }
}

