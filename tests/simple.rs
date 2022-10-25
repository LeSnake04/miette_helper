use miette_helper::{or_wrap_err, wrap_err};

#[test]
fn miette() {
	let test: Result<u32, std::io::Error> = Ok(5);
	assert!(wrap_err!(test, "test").is_ok());
	let test2: Option<u32> = None;
	assert!(or_wrap_err!(test2, "unwrap test failed").is_err());
}
