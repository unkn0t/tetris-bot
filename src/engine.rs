use std::{net::TcpStream, marker::PhantomData};

use tungstenite::{WebSocket, stream::MaybeTlsStream, Message, Error};
use url::Url;
use serde::Serialize;

pub struct WebClient<C, B, S> 
{
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    solver: S,
    commands: Commands<C>,
    phantom_data: PhantomData<B>,
}

pub struct Commands<C> {
    buffer: Vec<C>,
}

pub trait Solver<C, B> {
    fn start(&mut self, commands: &mut Commands<C>);

    fn solve(&mut self, commands: &mut Commands<C>, board: &B);
}

impl<C, B, S> WebClient<C, B, S>
    where
        C: Serialize,
        B: From<String>,
        S: Solver<C, B> 
{
    pub fn connect(url: Url, solver: S) -> Self {
        let (socket, _) = tungstenite::connect(url).expect("Can't connect");
        log::info!("Connected to the server.");

        Self {
            socket,
            solver,
            commands: Commands::empty(),
            phantom_data: PhantomData,
        }
    }

    pub fn run(&mut self) {
        let mut is_first_message = true;

        loop {
            match self.socket.read() {
                Ok(Message::Text(text)) => if is_first_message {
                    self.on_first_message();
                    is_first_message = false;
                } else {
                    self.on_message(text);
                }
                Err(Error::ConnectionClosed) => {
                    self.on_close();
                    break;
                }
                Err(e) => {
                    log::error!("Error while reading: {e}");
                    break;
                }
                _ => {}
            }
        }
    }
    
    fn on_first_message(&mut self) { 
        self.solver.start(&mut self.commands); 
        let answer = self.commands.execute();
        self.socket.write(Message::Text(answer)).unwrap();
        self.socket.flush().unwrap();
    }

    fn on_message(&mut self, text: String) {
        let board = B::from(text);
        self.solver.solve(&mut self.commands, &board);
        let answer = self.commands.execute();
        self.socket.write(Message::Text(answer)).unwrap();
        self.socket.flush().unwrap();
    }

    fn on_close(&self) {
        log::info!("Connection closed.");
    }   
}

impl<C: Serialize> Commands<C> {
    pub fn empty() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn add(&mut self, command: C) {
        self.buffer.push(command); 
    }

    pub fn execute(&mut self) -> String {
        let len = self.buffer.len(); 
        let executables: Vec<_> = self.buffer.drain(0..len).collect(); 
        serde_json::to_string(&executables).unwrap()
    }
}

