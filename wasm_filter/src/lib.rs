use proxy_wasm::{traits::RootContext, types::LogLevel};
use crate::context::root_context::ContentTypeRoot;

mod context;

#[no_mangle]
#[cfg(not(test))]
pub fn _start() {
    start();
}

pub fn start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(ContentTypeRoot::new()) });
}
