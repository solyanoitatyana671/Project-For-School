import rand::prelude::*;
let rng = rand::thread_rng();
let random_number = rng.gen_range(0..=10);
println!("{}", random_number)