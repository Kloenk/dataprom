use super::Plugin;

pub(crate) fn add_defaults(plugins: &mut super::Plugins) {
    if let Some(test) = create_test() {
        plugins.add(test);
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
