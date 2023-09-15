use std::sync::Arc;

use once_cell::sync::Lazy;
use spinners::{Spinner, Spinners};
use tokio::sync::{Mutex, OwnedMutexGuard};

const OK_SYMBOL: &'static str = "\u{001b}[1;92m✔\u{001b}[22;39m";
const ERR_SYMBOL: &'static str = "\u{001b}[1;91m✘\u{001b}[22;39m";

pub enum Action<'a> {
	End,
	StartOrUpdate(&'a str),
	Complete(&'a str),
	Fail(&'a str),
}

struct Status {
	enabled: bool,
	inner: Option<Spinner>,
}

impl Status {
	fn enable(&mut self) {
		self.enabled = true;
	}

	fn disable(&mut self) {
		if let Some(mut spinner) = self.inner.take() {
			spinner.stop();
			print!("\r\x1b[0m\x1b[K");
		}
	}
}

fn new_spinner(message: String) -> Spinner {
	Spinner::new(Spinners::Dots, message)
}

static _STATUS: Lazy<Arc<Mutex<Status>>> = Lazy::new(|| {
	Arc::new(Mutex::new(Status {
		enabled: false,
		inner: None,
	}))
});

async fn get_status() -> OwnedMutexGuard<Status> {
	_STATUS.clone().lock_owned().await
}

pub async fn status_action<'a>(action: Action<'a>) {
	match action {
		Action::End => status_end().await,
		Action::StartOrUpdate(text) => status_start_or_update(text.to_string()).await,
		Action::Complete(text) => status_complete(text.to_string()).await,
		Action::Fail(text) => status_fail(text.to_string()).await,
	}
}

async fn status_end() {
	let mut status = get_status().await;

	if !status.enabled {
		return ();
	}

	if let Some(mut spinner) = status.inner.take() {
		spinner.stop();
		print!("\r\x1b[0m\x1b[K");
	}
}

async fn status_start_or_update(text: String) {
	let mut status = get_status().await;

	if !status.enabled {
		return ();
	}

	if let Some(mut spinner) = status.inner.take() {
		spinner.stop();
		print!("\r\x1b[0m\x1b[K");
	}

	let _ = status.inner.insert(new_spinner(text));
}

async fn status_complete(text: String) {
	let mut status = get_status().await;

	if !status.enabled {
		return ();
	}

	if let Some(mut spinner) = status.inner.take() {
		spinner.stop_and_persist(OK_SYMBOL, text)
	} else {
		new_spinner("".to_string()).stop_and_persist(OK_SYMBOL, text);
	}
}

async fn status_fail(text: String) {
	let mut status = get_status().await;

	if !status.enabled {
		return ();
	}

	if let Some(mut spinner) = status.inner.take() {
		spinner.stop_and_persist(ERR_SYMBOL, text)
	} else {
		new_spinner("".to_string()).stop_and_persist(ERR_SYMBOL, text);
	}
}

pub async fn enable_reporting() {
	get_status().await.enable();
}

pub async fn disable_reporting() {
	get_status().await.disable();
}
