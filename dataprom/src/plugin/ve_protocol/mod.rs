use super::Plugin;
use super::{DataIn, DataOut};

mod code;

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
    fn parse(&self, data: DataIn) -> Vec<DataOut> {
        let mut ret_vec = Vec::new();
        let data_str: String = data.data.trim_matches(':').trim().to_ascii_lowercase();
        let data_resp = code::Response::parse(&data_str);
        info!("data: {:?}", data_resp);

        ret_vec
    }
}