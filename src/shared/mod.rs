pub mod error;
pub mod infrastructure;
pub mod utils;

#[cfg(test)]
pub fn init_test_env() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
    });
}