use itertools::Itertools;
pub struct Player {
    pub values1: Vec<u8>,
    pub values2: Vec<u8>
}
impl Player {
    pub fn combination(vector: &Vec<u8>) -> bool{
        let mut interrupter = false;
        let vector = vector;
        if vector.len() >= 3{
            let combination_result = vector.into_iter()
                .combinations(3).collect_vec();
            for i in combination_result{
                if matching(i) {
                    interrupter = true;
                }
            }
        }
        interrupter
    }
}
fn matching(mut vector: Vec<&u8>) -> bool{
    vector.sort();
    match vector.as_slice() {
        &[1,2,3] |
        &[4,5,6] |
        &[7,8,9] |
        &[1,4,7] |
        &[2,5,8] |
        &[3,6,9] |
        &[1,5,9] |
        &[3,5,7] => true,
        _ => false
    }
}
