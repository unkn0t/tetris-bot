mod engine;
mod games;

use games::tetris;

const PLAYER_URL: &str = "wss://dojorena.io/codenjoy-contest/ws?user=dojorena4637&code=3214792745498937324"; 

fn main() {
    pretty_env_logger::init();

    let url = url::Url::parse(PLAYER_URL).unwrap();
    let mut web_client = engine::WebClient::connect(url, tetris::Solver::new());
    web_client.run();
}

