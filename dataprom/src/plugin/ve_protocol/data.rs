use std::i64;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum Data {
    /// Product Id
    /// *0x0100*
    /// # Data
    /// - scale: -
    /// - type: u16
    /// - unit: -
    ProductId(Flags, u16),

    /// Group Id
    /// *0x0104*
    /// # Data
    /// - scale: -
    /// - type: u8
    /// - unit: -
    GroupId(Flags, u8),

    /// Device instance
    /// *0x0105*
    /// # Data
    /// - scale: -
    /// - type: u8
    /// - unit: -
    DeviceInstance(Flags, u8),

    /// Device class
    /// *0x0106*
    /// # Data
    /// - scale: -
    /// - type: u16
    /// - unit: -
    DeviceClass(Flags, u16),

    /// Serial number
    /// *0x010A*
    /// # Data
    /// - scale: -
    /// - type: String
    /// - unit: -
    SerialNumber(Flags, String),

    /// Model name
    /// *0x010B*
    /// # Data
    /// - scale: -
    /// - type: String
    /// - unit: -
    ModelName(Flags, String),

    /// Capabilities
    /// *0x0140*
    /// # Data
    /// - scale: -
    /// - type: u32
    /// - unit: -
    Capabilities(Flags, u32),

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
        info!("got address {} to parse: {}, len({})", addr, input, input.len());

        match addr {
            0x0100 => { // Product Id
                Data::Unknown(input.to_string())
            },
            0x0104 => { // Group Id
                Data::Unknown(input.to_string())
            },
            0xEDF0 => { // Battery maximum current
                if let Some(ret) = Self::check_len(input, 12) {
                    return ret;
                }
                let flags = Flags::parse(&t[4..5]);

                let v = parse_u16(&t[6..10]).unwrap_or(0);
                let v: f32 = (v as f32) / 10.0;

                trace!("got battery maximum current: {}A", v);
                Data::BatteryMaximumCurrent(flags, v)
            }, 
            _ => Data::Unknown(input.to_string())
        }
    }
    fn check_len(input: &str, len: usize) -> Option<Self> {
        if input.len() != len {
            warn!("invalid len: {} ({})", input.len(), input);
            return Some(Data::Unknown(input.to_string()));
        }
        None
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