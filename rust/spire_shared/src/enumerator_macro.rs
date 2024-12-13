#[macro_export]
macro_rules! enumerator {
	($Block:block) => {
		std::iter::from_coroutine(
			#[coroutine]
			move || $Block,
		)
	};
}
