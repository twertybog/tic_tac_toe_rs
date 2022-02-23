mod board;
mod players;
mod availability_check;
use players::{Player};
use board::{Values};
fn main() {
    let mut players = Player{
        values1: vec![],
        values2: vec![]
    };
    let mut game = Values{
        printable: vec!["[1]".to_string(),"[2]".to_string(),"[3]".to_string(),
                        "[4]".to_string(),"[5]".to_string(),"[6]".to_string(),
                        "[7]".to_string(),"[8]".to_string(),"[9]".to_string()],
        available: vec!["1".to_string(),"2".to_string(),"3".to_string(),
                        "4".to_string(),"5".to_string(),"6".to_string(),
                        "7".to_string(),"8".to_string(),"9".to_string()],
    };
    board::pr_board(&game);
    'run: for i in 0..9{
        let mut parameter: u8;
        if i%2 == 0{
            parameter = availability_check::check(&mut game, "x");
            players.values1.push(parameter);
            board::print(String::from(" x "), &mut game, parameter);
            if Player::combination(&players.values1) {
                println!("Player x is a winner");
                break 'run;
            }
        }
        else if i%2==1{
            parameter = availability_check::check(&mut game, "o");
            players.values2.push(parameter);
            board::print(String::from(" o "), &mut game, parameter);
            if Player::combination(&players.values2) {
                println!("Player o is a winner");
                break 'run;
            }
        }
    }
    println!("The game is over.");
}
