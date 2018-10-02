use chrono::Local;
use slugify::slugify;

pub fn YmdHMS() -> u64{
    Local::now()
        .format("%Y%m%d%H%M%S")
        .to_string()
        .parse::<u64>()
        .unwrap()
}

pub fn slug(sentence: &str ) -> String{
    slugify!(sentence)
}
