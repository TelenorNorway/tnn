mod ops;

mod api;
pub use api::*;

tnn::extension!(
	dependencies = {
		tnn = "1",
		my_cool_ext = optional "^1"
	},
	ops = [
		ops::greet,
		ops::hello,
	]
);
