use axum::{
	routing::{get, post},
	Router,
};
use std::{fs, sync::atomic::AtomicU64};
use std::path::Path;
use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::tcp::{authenticate, handle_client};

pub mod tcp;
pub mod utils;
pub mod caches;
pub mod state;
pub mod error;
pub mod types;
mod endpoints {
	pub mod ws;
	pub mod metrics;
	pub mod v1 {
		pub mod get;
		pub mod set;
		pub mod del;
		pub mod list;
		pub mod exists;
		pub mod incr;
		pub mod decr;
		pub mod ping;
		pub mod save;
		pub mod clean;
		pub mod flush;
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

	/// Persistant cache path
	#[arg(long, default_value_t = String::from("./cache"))]
	path: String,

	/// Preserve items relative order
	#[arg(long, default_value_t = false)]
	preserve_order: bool,

}

#[tokio::main]
async fn main(){
	let args: Args = Args::parse();

	let state: Arc<SharedState> = Arc::new(SharedState { token: args.token.clone(), ws_connections: AtomicU64::new(0), cache: Mutex::new(Cache::new(args.path.clone(), args.preserve_order)) });

	let file = args.path.clone() + "/cache.json";
	let path = Path::new(&file);
	if !path.exists() {
		fs::create_dir_all(&args.path).expect("Failed with creating cache.json file!");
		fs::write(&file, "{}").expect("Failed with creating cache.json file!");
	}
	state.clone().cache.lock().unwrap().load().ok();

	let address: String = args.address.clone() + ":" + &args.port.to_string();
	let tcp_address: String = args.address.clone() + ":" + &(args.port + 1).to_string();

	let app: Router = Router::new()
	.route("/ws/:token", get(endpoints::ws::handle_get))
	.route("/metrics", get(endpoints::metrics::handle_get))
	.route("/v1/health", get(endpoints::v1::health::handle_get))
	.route("/v1/set", post(endpoints::v1::set::handle_post))
	.route("/v1/set/:key/:value/:ttl", get(endpoints::v1::set::handle_get))
	.route("/v1/del", post(endpoints::v1::del::handle_post))
	.route("/v1/del/:key", get(endpoints::v1::del::handle_get))
	.route("/v1/list", post(endpoints::v1::list::handle_post))
	.route("/v1/list/:prefix/:limit/:cursor", get(endpoints::v1::list::handle_get))
	.route("/v1/incr", post(endpoints::v1::incr::handle_post))
	.route("/v1/incr/:key/:value/:ttl", get(endpoints::v1::incr::handle_get))
	.route("/v1/decr", post(endpoints::v1::decr::handle_post))
	.route("/v1/decr/:key/:value/:ttl", get(endpoints::v1::decr::handle_get))
	.route("/v1/get", post(endpoints::v1::get::handle_post))
	.route("/v1/get/:key", get(endpoints::v1::get::handle_get))
	.route("/v1/exists", post(endpoints::v1::exists::handle_post))
	.route("/v1/exists/:key", get(endpoints::v1::exists::handle_get))
	.route("/v1/save", get(endpoints::v1::save::handle_get))
	.route("/v1/clean", get(endpoints::v1::clean::handle_get))
	.route("/v1/flush", get(endpoints::v1::flush::handle_get))
	.route("/v1/stats", get(endpoints::v1::stats::handle_get))
	.route("/v1/ping", get(endpoints::v1::ping::handle_get))
	.with_state(state.clone());

	tokio::spawn(async move {
		let listener: TcpListener = TcpListener::bind(&address).await.expect("Failed to bind HTTP listener");
		println!("HTTP Server is running on {}", &address);
		axum::serve(listener, app).await.unwrap();
	});

	let tcp_listener: TcpListener = TcpListener::bind(&tcp_address).await.expect("Failed to bind TCP listener");
	println!("TCP Server is running on {}", &tcp_address);

	loop {
		match tcp_listener.accept().await {
			Ok((mut stream, addr)) => {
				println!("New TCP connection: {}", addr);
				let token_clone: String = args.token.clone();
				let state_clone: Arc<SharedState> = state.clone();

				tokio::spawn(async move {
					if authenticate(&mut stream, &token_clone).await {
						handle_client(&mut stream, state_clone).await;
					} else {
						println!("Authentication failed for {}", addr);
					}
				});
			}
			Err(e) => {
				eprintln!("Error accepting TCP connection: {}", e);
			}
		}
	}
}