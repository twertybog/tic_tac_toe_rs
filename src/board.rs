use std::mem::replace;
pub struct Values{
    pub printable: Vec<String>,
    pub available: Vec<String>,
}
pub fn print(value: String, game: &mut Values,
             parameter: u8){
    replace(&mut game.printable[(parameter - 1) as usize], value);
    pr_board(&game);
}
pub fn pr_board(game: &Values){
    println!("\
{}|{}|{}
---+---+---
{}|{}|{}
---+---+---
{}|{}|{}", game.printable[0],game.printable[1],game.printable[2]
             ,game.printable[3],game.printable[4],game.printable[5]
             ,game.printable[6],game.printable[7],game.printable[8]);
}
