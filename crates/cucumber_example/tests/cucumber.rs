use cucumber::{writer, given, when, then, World as _};

#[derive(cucumber::World, Default, Debug)]
struct World {
	user: Option<String>,
	capacity: usize,
}

#[given(expr = "{word} is hungry")]
async fn iam_hungry(w: &mut World, user: String) {
	w.user = Some(user);
}

#[when(regex = r"^(?:he|she|they|it) eats? (\d+) cucumbers?$")]
async fn eat_cucumbers(w: &mut World, count: usize) {
	w.capacity += count;
	assert_eq!(w.capacity, 3, "{} ate too much", w.user.as_ref().unwrap());
}

#[then(regex = r"^(?:he|she|they|it) is full")]
async fn is_full(w: &mut World) {
	assert_eq!(w.capacity, 3, "{} isn't full", w.user.as_ref().unwrap());
}

#[tokio::main]
async fn main() {
	// libtest for IntelliJ integration
	World::cucumber()
		.with_writer(writer::Libtest::or_basic())
		.run("tests/features").await;
}