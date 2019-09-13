use super::Plugin;
use super::{DataIn, DataOut};

mod code;

mod data;

pub(crate) struct VeProtocol {

}

impl VeProtocol {
    pub(crate) fn new() -> Self {
        Self {

        }
    }
}

impl Plugin for VeProtocol {
    fn name(&self) -> String {
        "ve_protocol".to_string()
    }
    fn init(&mut self) {

    }
    fn parse(&self, data_in: DataIn) -> Vec<DataOut> {
        let mut ret_vec = Vec::new();
        let data_str: String = data_in.data.trim_matches(':').trim().to_ascii_lowercase();
        let data_str: Vec<&str> = data_str.split('\n').collect();
        for v in data_str {
            let v = v.trim_matches(':').trim();
            info!("parse {}", v);
            let data_resp = code::Response::parse(v);
            info!("data: {:?}", data_resp);
        }
        

        ret_vec
    }
}