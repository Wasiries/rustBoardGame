use crate::board;
pub struct Player {
    name: String,
    letter: char,
    position: PlayerPosition
}


pub enum PlayerPosition {
    FirstPlayer,
    SecondPlayer
}


pub struct Game {
    desk: board::Board,
    playerList: Vec<Player>,
    action_counter: usize,
    who_is_going: Player
}


pub struct Move {
    player: Player,
    row: usize,
    column: usize,
    action_counter: usize,
    desk: Vec<Vec<char>>,
}


impl Player {
    pub fn enter_name() -> String{
        let mut output = String::new();
        std::io::stdin().read_line(&mut output).expect("error");
        let output: String = output.trim().parse().expect("error");
        return output;
    }


    pub fn enter_letter() -> char {
        let mut output = String::new();
        std::io::stdin().read_line(&mut output).expect("error");
        let output: char = output.trim().parse().expect("error");
        return output;
    }


    pub fn creating_player(position: PlayerPosition) -> Player {
        let name = Player::enter_name();
        let letter = Player::enter_letter();
        let output = Player {
            name: name,
            letter: letter,
            position: position,
        };
        return output;
    }
}


impl Move {
    pub fn making_move(&self) -> board::Board {
        let mut action = self.desk;
        action[self.row][self.column] = self.player.letter;
        return action;
    }


    pub fn print_move_info(&self) {
        println!("[ {} ]: player {} made his move", self.action_counter, self.player.name);
        println!("[ {} ]: his letter was placed in {} row and {} column", self.action_counter, self.row, self.column);
    }


    pub fn enter_row() -> usize {
        let mut row = String::new();
        std::io::stdin().read_line(&mut row);
        let row: usize = row.trim().parse().expect("error");
        return row;
    }


    pub fn enter_column() -> usize {
        let mut column = String::new();
        std::io::stdin().read_line(&mut column);
        let column: usize = column.trim().parse().expect("error");
        return column;
    }
}


impl Game {
    pub fn is_this_win(&self) -> bool {
        if (self.desk[0][0] == self.desk[0][1] == self.desk[0][2]) || 
        (self.desk[1][0] == self.desk[1][1] == self.desk[1][2]) ||
        (self.desk[2][0] == self.desk[2][1] == self.desk[2][2]) ||
        (self.desk[0][0] == self.desk[1][0] == self.desk[2][0]) ||
        (self.desk[0][1] == self.desk[1][1] == self.desk[2][1]) ||
        (self.desk[0][2] == self.desk[1][2] == self.desk[2][2]) ||
        (self.desk[0][0] == self.desk[1][1] == self.desk[2][2]) ||
        (self.desk[0][2] == self.desk[1][1] == self.desk[2][0]) {
            return true;
        } else {
            return false;
        }
    }


    pub fn board_creation() -> board::Board {
        let mut desk = board::Board::create_board();
        return desk;
    }


    pub fn initialisation() -> Game {
        let mut desk = Game::board_creation();
        let mut action_counter: usize = 0;
        let players: Vec<Player> = vec![Player::creating_player(PlayerPosition::FirstPlayer), Player::creating_player(PlayerPosition::SecondPlayer)];
        let mut goes_now = players[0];
        let mut output = Game {
            desk: desk,
            playerList: players,
            action_counter: action_counter,
            who_is_going: goes_now,
        };
        return output;
    }


    pub fn going_now(&self) {
        if self.action_counter % 2 == 0 {
            self.who_is_going = self.playerList[0];
        } else {
            self.who_is_going = self.playerList[1];
        }
    }


    pub fn main_loop(&self) {
        while self.is_this_win() == false {
            let action = Move {
                player: self.who_is_going,
                row: Move::enter_row(),
                column: Move::enter_column(),
                action_counter: self.action_counter,
                desk: self.desk.;
            };
            self.desk = action.making_move();
            action.print_move_info();
            if self.is_this_win() == true {
                println!("player {} won", self.who_is_going.name);
                break;
            } else {
                self.action_counter = self.action_counter + 1;
                self.going_now();
            }
        }
    }
}