use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;

use super::http_context::ContentTypeHttp;

pub struct ContentTypeRoot {}

/// RootContext filter
impl ContentTypeRoot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Context for ContentTypeRoot {}

impl RootContext for ContentTypeRoot {
    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(ContentTypeHttp::new()))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
