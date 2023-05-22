mod ascii;
pub mod config;

use ascii::Ascii;
use config::Config;
use rand::Rng;

pub fn generate(config: Config) -> String {
    let mut result = String::new();
    let mut special_count: u8 = 0;

    for _ in 0..config.len() {
        loop {
            let rng: u8 = rand::thread_rng().gen_range(33..126);

            if Ascii::cmp_invalid(rng) {
                continue;
            }

            if Ascii::cmp_special(rng) {
                if special_count >= config.max_special() {
                    continue;
                }

                special_count += 1;
            }

            result.push(rng as char);
            break;
        }
    }

    result
}
