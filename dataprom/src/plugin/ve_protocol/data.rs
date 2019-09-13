use std::i64;

#[derive(Debug)]
pub(crate) enum Data {
    /// Battery maximum current
    /// *0xEDF0*
    /// # Data
    /// - scale: 0.1
    /// - type: u16
    /// - unit: A
    BatteryMaximumCurrent(Flags, f32),

    Unknown(String),
}

impl Data {
    pub(crate) fn parse(input: &str) -> Self {
        let t: Vec<char> = input.chars().collect();

        let addr = parse_u16(&t[0..4]).unwrap_or(0);
        info!("got address {} to parse", addr);

        match addr {
            0xEDF0 => { // 0xEDF0; Battery maximum current; scale: 0.1; type: u16; unit: A
                let flags = Flags::parse(&t[4..5]);

                let v = parse_u16(&t[6..10]).unwrap_or(0);
                let v: f32 = (v as f32) / 10.0;

                trace!("got battery maximum current: {}A", v);
                Data::BatteryMaximumCurrent(flags, v)
            }, 
            _ => Data::Unknown(input.to_string())
        }

        /*if input.starts_with("edf0") { // 0xEDF0; Battery maximum current; scale: 0.1; type: u16; unit: A
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
        }*/
    }
}

#[derive(Debug)]
pub(crate) struct Flags {

}

impl Flags {
    pub(crate) fn parse(high_nible: &[char]) -> Self {
        eprintln!("Flags are still unimplemented");
        Self { }
    }
}

pub(crate) fn parse_u16(input: &[char]) -> Result<u16, String> {    // TODO: implement error type
    if input.len() != 4 {
        return Err("invalid lenght".to_string());
    }
    let addr_low: String = input[0..2].iter().collect(); // FIX detection of not available addresses
    let addr_low = i64::from_str_radix(&addr_low, 16).unwrap_or(0); // FIXME: error return
    let addr_high: String = input[2..4].iter().collect();
    let addr_high = i64::from_str_radix(&addr_high, 16).unwrap_or(0);   //FIXME: error return
    let addr = addr_low + (addr_high * 256);
    let addr = addr as u16;
    Ok(addr)
}