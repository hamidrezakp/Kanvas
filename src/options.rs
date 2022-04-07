#[derive(Eq, PartialEq)]
pub struct Options {
    pub width: usize,
    pub height: usize,
    pub colors: &'static str,
    pub refresh_period_miliseconds: u64,
    pub cooldown_duration_seconds: i64,
}

pub const OPTIONS: Options = Options {
    width: 100,
    height: 100,
    colors: include_str!("../colors.json"),
    refresh_period_miliseconds: 1000,
    cooldown_duration_seconds: 10,
};
