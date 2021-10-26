use log::debug;
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;

/// HttpContext filter
pub struct ResponseStatusHttp {}

impl ResponseStatusHttp {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

// Response related methods
impl ResponseStatusHttp {
    fn status(&self) {
        debug!("-----------------------------------------------");
        debug!("headers: {:?}", self.get_http_response_headers());
        debug!("code: {:?}", self.get_property(vec!["response", "code"]));
        debug!("code_details: {:?}", self.get_property(vec!["response", "code_details"]));
        debug!("-----------------------------------------------");
    }
}

impl Context for ResponseStatusHttp {}

impl HttpContext for ResponseStatusHttp {
    // OnResponse
    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.status();
        Action::Continue
    }
}
