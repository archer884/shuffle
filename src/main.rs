extern crate nshuffle;
extern crate rand;

pub fn main() {
    let mut rng = rand::weak_rng();
    let names: Vec<_> = std::env::args().skip(1).collect();
    let snames = nshuffle::shuffle(&mut rng, &names);

    for i in 0..names.len() {
        println!("{}: {}", names[i], snames[i]);
    }
}
