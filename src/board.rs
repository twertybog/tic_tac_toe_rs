use std::mem::replace;
#[derive(Debug)]
pub struct Values{
    pub printable: Vec<char>,
    pub available: Vec<String>,
}
pub fn print(value: char, game: &mut Values,
             parameter: u8){
    replace(&mut game.printable[(parameter - 1) as usize], value);
    println!("
1|2|3   {}|{}|{}
-+-+-   -+-+-
4|5|6   {}|{}|{}
-+-+-   -+-+-
7|8|9   {}|{}|{}", game.printable[0],game.printable[1],game.printable[2]
             ,game.printable[3],game.printable[4],game.printable[5]
             ,game.printable[6],game.printable[7],game.printable[8]);


}