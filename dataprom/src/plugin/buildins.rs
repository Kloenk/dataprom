use super::Plugin;

pub(crate) fn add_defaults(plugins: &mut super::Plugins) {
    if let Some(test) = create_test() {
        plugins.add(test);
    }
}

#[cfg(not(feature = "test"))]
fn create_test() -> Option<Box<dyn Plugin>> {
    None
}

#[cfg(feature = "test")]
fn create_test() -> Option<Box<dyn Plugin>> {
    info!("test enabled");
    Some(Box::new(super::test::Test::new("test")))
}