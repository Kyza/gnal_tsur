use gnal_tsur::lang_rust;

#[test]
fn one_layer() {
	lang_rust! {
		;a + a = b let
		;1 = a let
	}

	assert_eq!(a, 1);
	assert_eq!(b, 2);

	assert_eq!(lang_rust! { 2 }, 2);
}

#[test]
fn two_layers() {
	assert_eq!(lang_rust! { [2, 1]!vec }, vec![1, 2]);
}

#[test]
fn fib() {
	fn fibonacci_original(n: u32) -> u32 {
		match n {
			0 => 1,
			1 => 1,
			_ => fibonacci_original(n - 1) + fibonacci_original(n - 2),
		}
	}

	lang_rust! {
		{
			{
				,(2 - n)fibonacci + (1 - n)fibonacci >= _
				,1 >= 1
				,1 >= 0
			} n match
		} u32 >- (u32 :n)fibonacci fn
	}

	for i in 0..=10 {
		assert_eq!(fibonacci(i), fibonacci_original(i));
	}
}
