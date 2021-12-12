use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;

/// HttpContext filter
pub struct ContentTypeHttp {}

impl ContentTypeHttp {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Context for ContentTypeHttp {}

impl HttpContext for ContentTypeHttp {
    // OnResponse
    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.send_http_response(200, vec![], Some("response from wasm filter".as_bytes()));
        Action::Pause
    }
}
