use log::error;
use miette_helper::{or_wrap_err, unwrap_or_print, wrap_err};

#[test]
fn main() {
	let test: Result<u32, std::io::Error> = Ok(5);
	let a = wrap_err!(test, "test");
	unwrap_or_print!(a, error);
	let test2 = Some(4);
	let b = or_wrap_err!(test2, "unwrap test failed");
	unwrap_or_print!(b, error);
}
