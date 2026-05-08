/* ---------------------------------------------------------
 * 📑 LABEL: TestController (test_controller.rs)
 * --------------------------------------------------------- */

use crate::app::view;
use rustbasic_core::requests::Request;
use rustbasic_core::axum::response::IntoResponse;
use rustbasic_core::minijinja::context;

pub struct TestController;

impl TestController {
    pub async fn index(req: Request) -> impl IntoResponse {
        view(&req, "test.rb.html", context! {
            title => "TestController"
        })
    }
}
