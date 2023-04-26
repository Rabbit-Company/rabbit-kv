use std::fs;
use clap::Parser;
use axum::{Router, routing::post};
use std::sync::{Arc, Mutex};

use crate::accounts::Accounts;

pub mod errors;
pub mod caches;
pub mod accounts;
pub mod endpoints;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

	/// Bind the server to specific address
	#[arg(short, long, default_value_t = String::from("0.0.0.0"))]
	address: String,

	/// Bind the server to specific port
	#[arg(short, long, default_value_t = 6380)]
	port: u16,

}

#[tokio::main]
async fn main(){

	let args: Args = Args::parse();
	fs::create_dir_all("/var/lib/rabbitkv/storage").expect("Permission denied. Please run program with root user.");

	let accounts: Arc<Mutex<Accounts>> = Arc::new(Mutex::new(Accounts::new()));
	{
		accounts.lock().unwrap().import().ok();
	}

	let app: Router<_, _> = Router::new()
		.route("/account/create", post(endpoints::create_account))
		.with_state(accounts.clone());

	let address: String = args.address + ":" + &args.port.to_string();
	println!("Rabbit KV listening on {}", &address);
	axum::Server::bind(&address.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}