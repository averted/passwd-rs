use rand::Rng;
use std::env::Args;

struct Blacklist {
    special_count: u8,
    invalid: [u8; 8],
    special: [u8; 23],
}

#[rustfmt::skip]
impl Blacklist {
    fn new() -> Self {
        Self {
            special_count: 0,
            invalid: [34, 39, 47, 60, 62, 92, 96, 124],
            special: [
                33, 35, 36, 37, 38,
                40, 41, 42, 43, 44,
                45, 46, 47, 58, 59,
                61, 63, 64, 91, 93,
                94, 95, 126,
            ],
        }
    }

    fn cmp_invalid(&self, value: u8) -> bool {
        self.invalid.contains(&value)
    }

    fn cmp_special(&self, value: u8) -> bool {
        self.special.contains(&value)
    }

    fn bump_special(&mut self) {
        self.special_count += 1;
    }
}

pub struct Config {
    len: u8,
    max_special: u8,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        let mut len = 24;
        args.next();

        if let Some(value) = args.next() {
            match value.parse::<u8>() {
                Ok(v) => len = v,
                Err(_) => return Err("Invalid length value"),
            };
        }

        let max_special = (len as f32 * 0.15).floor() as u8;

        Ok(Self { len, max_special })
    }
}

pub fn generate(config: Config) -> String {
    let mut blacklist = Blacklist::new();
    let mut result: String = String::new();

    for _ in 0..config.len {
        loop {
            let rng: u8 = rand::thread_rng().gen_range(33..126);

            if blacklist.cmp_invalid(rng) {
                continue;
            }

            if blacklist.cmp_special(rng) {
                if blacklist.special_count >= config.max_special {
                    continue;
                }

                blacklist.bump_special()
            }

            result.push(rng as char);
            break;
        }
    }

    result
}
