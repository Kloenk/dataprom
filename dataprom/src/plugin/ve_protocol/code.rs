use std::i64;
use super::data::Data;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum Response {
    /// Successful execution of the received command.
    /// Payload depends on command.
    /// 
    /// Code: 1
    Done(String),

    /// Unknown command, data is the unknown command.
    /// 
    /// Code: 3
    Unknown(String),

    /// Frame error (payload=0xAAAA), unable to
    /// enter bootloader (payload=0).
    /// 
    /// Code: 4
    Error(String),

    /// The version number is directly interpreted from the hex representation,
    /// e.g. 0x0101 is version 1.01. The two most significant bits indicate the
    /// firmware type:
    /// b00: bootloader
    /// b01: application
    /// b10: tester
    /// b11: release candidate
    /// In case of release candidate the lowest two bits of the highest nibble
    /// together with type indicate the release candidate number. E.g. 0xD101
    /// represents release candidate D of version 1.01.
    /// Note that there can only be 4 release candidates per version.
    /// 
    /// Code: 5
    Ping {
        version: String,
        firmware: FirmwareType,
    },

    /// uint16 - id: of the value being returned
    /// uint8 - flags: defined below
    /// type depends on id - value
    /// 
    /// Code: 7
    Get (Data),

    /// uint16 - id: of the value being returned
    /// uint8 - flags: defined below
    /// type depends on id - value
    /// 
    /// Code: 8
    Set (Data),
}

impl Response {
    pub fn parse(response: &str) -> Self {
        let t: Vec<char> = response.chars().collect();
        match t[0] {
            '1' => Response::Done(t[1..].iter().collect()),
            '3' => Response::Unknown(t[1..].iter().collect()),
            '4' => Response::Error(t[1..].iter().collect()),
            '5' => {
                warn!("not yet implemented: Ping message");
                Response::Ping{ version: "0.0".to_string(), firmware: FirmwareType::Application}
            },
            '7' => {
                let v: String = t[1..].iter().collect();
                let v = super::data::Data::parse(&v);
                info!("data: {:?}", v);
                Response::Get(v)
            },
            _ => Response::Unknown(response.to_string())    // TODO: reconsider own type for parsing errors?
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FirmwareType {
    /// bootloaded
    /// 
    /// Code: b00
    BootLoader,

    /// application
    /// 
    /// Code: b01
    Application,

    /// tester
    /// 
    /// Code: b10
    Tester,

    /// release candidate
    /// In case of release candidate the lowest two bits of the highest nibble
    /// together with type indicate the release candidate number. E.g. 0xD101
    /// represents release candidate D of version 1.01.
    /// Note that there can only be 4 release candidates per version.
    /// 
    /// Code: b11
    ReleaseCandidate(String),
}
