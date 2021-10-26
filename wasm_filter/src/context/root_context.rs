use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;

use super::http_context::ResponseStatusHttp;

pub struct ResponseStatusRoot {}

/// RootContext filter
impl ResponseStatusRoot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Context for ResponseStatusRoot {}

impl RootContext for ResponseStatusRoot {
    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(ResponseStatusHttp::new()))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
