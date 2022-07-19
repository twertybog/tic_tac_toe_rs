use std::io;
use crate::cli::board;
use board::Values;

pub fn check(game: &mut Values, player: &str) -> u8{
    let mut parameter:u8 = 0;
    'checking: loop {
        println!("Player {} please enter a cell number", player);
        let mut player_x = String::new();
        io::stdin().read_line(&mut player_x).unwrap();
        let player_x: String = player_x.trim().parse()
            .expect("Something went wrong!");
        for i in 0..game.available.len(){
            if player_x.eq(&game.available[i]){
                parameter = player_x.trim().parse()
                    .expect("Cannot convert to u8");
                game.available.remove(i);
                break 'checking;
            }
        }
    }
    parameter
}