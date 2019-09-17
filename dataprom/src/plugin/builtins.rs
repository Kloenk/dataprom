use super::Plugin;

pub(crate) fn add_defaults(plugins: &mut super::Plugins) {
    if let Some(test) = create_test() {
        plugins.add(test);
    }
    if let Some(ve_protocol) = create_ve_protocol() {
        plugins.add(ve_protocol)
    }
}

#[cfg(not(feature = "data_raw"))]
fn create_test() -> Option<Box<dyn Plugin>> {
    None
}

#[cfg(feature = "data_raw")]
fn create_test() -> Option<Box<dyn Plugin>> {
    info!("data_raw enabled");
    Some(Box::new(super::data_raw::DataRaw::new("data_raw")))
}

#[cfg(not(feature = "ve_protocol"))]
fn create_ve_protocol() -> Option<Box<dyn Plugin>> {
    None
}

#[cfg(feature = "ve_protocol")]
fn create_ve_protocol() -> Option<Box<dyn Plugin>> {
    info!("ve_protocol enabled");
    Some(Box::new(super::ve_protocol::VeProtocol::new()))
}