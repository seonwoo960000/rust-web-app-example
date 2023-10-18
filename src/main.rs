#[macro_use]
extern crate rocket;

#[launch]
async fn launch() -> _ {
    rust_demo::logger::init();
    rust_demo::api::init()
}
