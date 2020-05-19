use rand::seq::SliceRandom;
use std::env;

fn main(){
    let mut seed = rand::thread_rng();
    let box1: Vec<String> = env::args().collect();

    if box1.len() == 1 {
        println!("NO CAN'T, THIS VALUE BE EMPTY.");
    
    } else {
        let poker = String::from(box1[1].clone()); 
        let blackbox = &mut poker.into_bytes();

        blackbox.shuffle(&mut seed);
        let lambda = String::from_utf8(blackbox.to_vec()).unwrap();
        println!("{}", lambda);
    }
}