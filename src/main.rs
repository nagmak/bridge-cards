mod bridge;
 
use bridge::Game;
 
fn main() {
    println!("Welcome to Bridge!");
 
    let mut bridge = Game::new();
 
    bridge.play_game();
}