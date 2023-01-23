mod board;
mod gameplay;


fn main() {
    let mut game = gameplay::Game::initialisation();
    game.main_loop();
}
