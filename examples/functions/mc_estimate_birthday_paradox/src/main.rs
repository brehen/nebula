use rand::prelude::*;
use shared::{run_function, FunctionType};
use std::collections::HashSet;

fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(birthday_sharing, func_type);
}

pub fn birthday_sharing(n_people: u32) -> String {
    let mut rng = thread_rng();
    let trials = 1_000_000;
    let mut success = 0;
    for _ in 0..trials {
        let mut birthdays: HashSet<i32> = HashSet::new();
        for _ in 0..n_people {
            let birthday: i32 = rng.gen_range(0..365);
            if birthdays.contains(&birthday) {
                success += 1;
                break;
            } else {
                birthdays.insert(birthday);
            }
        }
    }
    format!(
        "Probability of {} people sharing a birthday is {}",
        n_people,
        success as f32 / trials as f32
    )
}
