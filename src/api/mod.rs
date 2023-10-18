use rocket::Build;

pub fn init() -> rocket::Rocket<Build> {
    info!("Starting up");

    rocket::build()
}
