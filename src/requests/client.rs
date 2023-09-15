use std::sync::Arc;

use once_cell::sync::Lazy;
use reqwest::{redirect, Client};
use tokio::sync::{Mutex, OwnedMutexGuard};

static _CLIENT: Lazy<Arc<Mutex<Client>>> = Lazy::new(|| {
	Arc::new(Mutex::new(
		Client::builder()
			.cookie_store(true)
			.redirect(redirect::Policy::none())
			.build()
			.expect("Could not create http client"),
	))
});

pub async fn get_client() -> OwnedMutexGuard<Client> {
	_CLIENT.clone().lock_owned().await
}
