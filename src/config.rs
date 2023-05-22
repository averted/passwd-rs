use std::env::Args;

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

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn max_special(&self) -> u8 {
        self.max_special
    }
}
