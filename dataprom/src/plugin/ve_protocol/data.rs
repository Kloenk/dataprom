use std::i64;

#[derive(Debug)]
pub(crate) enum Data {
    BatteryMaximumCurrent(Flags, f32),

    Unknown(String),
}

impl Data {
    pub(crate) fn parse(input: &str) -> Self {
        if input.starts_with("f0ed") {
            let t: Vec<char> = input.chars().collect();
            let flags = Flags::parse(&t[4..5]);
            warn!("use flags: {:?}", flags);

            let v: String = t[6..9].iter().collect();
            let v = i64::from_str_radix(&v, 16);
            if v.is_err() {
                warn!("unable to parse {}, {}", input, v.unwrap_err());
                return Data::Unknown(input.to_string());
            }
            let v = v.unwrap();
            let v: f32 = (v as f32) / (10.0);
            return Data::BatteryMaximumCurrent(flags, v);
        }
        Data::Unknown(input.to_string())
    }
}

#[derive(Debug)]
pub(crate) struct Flags {

}

impl Flags {
    pub(crate) fn parse(high_nible: &[char]) -> Self {
        Self { }
    }
}