use crate::config::{AppConfig, api_config, update_config};
use arc_swap::access::Access;
use rocket::{
	Build, Config, Request, Rocket, catch, catchers, launch, post, routes,
	serde::json::Json,
};

#[post("/config/update", format = "json", data = "<config>")]
async fn config(config: Json<AppConfig>) {
	update_config(config.into_inner());
}

#[catch(404)]
fn not_found(req: &Request) -> String {
	format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
pub fn config_listener() -> Rocket<Build> {
	let api_config = api_config().load();
	let config = Config::figment()
		.merge(("address", &api_config.address))
		.merge(("port", api_config.port))
		.merge(("workers", api_config.workers))
		.merge(("ident", &api_config.ident));

	rocket::build()
		.configure(config)
		.register("/", catchers![not_found])
		.mount("/api", routes![config])
}
