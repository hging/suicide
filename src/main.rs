fn main() {
	let mut maybe_hope = Some(());

	while let Some(hope) = maybe_hope.take() {
		match hope {
			() => maybe_hope = Some(()),
		}
	}

	println!("Goodbye, world!");
}
