use axum::{
	routing::{get, post},
	Router,
};
use std::fs;
use std::path::Path;
use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub mod caches;
pub mod state;
pub mod error;
mod endpoints {
	pub mod metrics;
	pub mod v1 {
		pub mod get;
		pub mod set;
		pub mod stats;
		pub mod health;
	}
}

use state::SharedState;
use crate::caches::cache::Cache;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

	/// Bind the server to specific address
	#[arg(short, long, default_value_t = String::from("0.0.0.0"))]
	address: String,

	/// Bind the server to specific port
	#[arg(short, long, default_value_t = 6380)]
	port: u16,

	/// Token used for authentication
	#[arg(short, long, default_value_t = String::from("default_token"))]
	token: String,

	/// Persistant cache
	#[arg(long, default_value_t = false)]
	persistant: bool,

	/// Persistant cache path
	#[arg(long, default_value_t = String::from("./cache"))]
	path: String,

}

#[tokio::main]
async fn main(){
	let args: Args = Args::parse();

	let state: Arc<SharedState> = Arc::new(SharedState { token: args.token, cache: Mutex::new(Cache::new(args.persistant, args.path.clone())) });

	if args.persistant {
		let file = args.path.clone() + "/cache.json";
		let path = Path::new(&file);
		if !path.exists() {
			fs::create_dir_all(&args.path).expect("Failed with creating cache.json file!");
			fs::write(&file, "{}").expect("Failed with creating cache.json file!");
		}

		state.cache.lock().unwrap().load().expect("Failed to load cache!");
	}

	let address: String = args.address + ":" + &args.port.to_string();

	let app: Router = Router::new()
	.route("/metrics", get(endpoints::metrics::handle_get))
	.route("/v1/health", get(endpoints::v1::health::handle_get))
	.route("/v1/set", post(endpoints::v1::set::handle_post))
	.route("/v1/set/:key/:value/:ttl", get(endpoints::v1::set::handle_get))
	.route("/v1/get/:key", get(endpoints::v1::get::handle_get))
	.route("/v1/stats", get(endpoints::v1::stats::handle_get))
	.with_state(state);

	println!("Server is running on {}", &address);

	let listener: TcpListener = TcpListener::bind(&address).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}