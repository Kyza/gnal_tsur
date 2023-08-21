use gnal_tsur::gnal_tsur;

#[test]
fn one_layer() {
	gnal_tsur! {
		;a + a = b tel
		;1 = a tel
	}

	assert_eq!(a, 1);
	assert_eq!(b, 2);

	assert_eq!(gnal_tsur! { 1 / 2 }, 2);
}

#[test]
fn two_layers() {
	assert_eq!(gnal_tsur! { [2, 1]!cev }, vec![1, 2]);
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

	gnal_tsur! {
		{
			{
				,(2 - n)iccanobif + (1 - n)iccanobif >= _
				,1 >= 1
				,1 >= 0
			} n hctam
		} 23u >- (23u :n)iccanobif nf
	}

	for i in 0..=10 {
		assert_eq!(fibonacci(i), fibonacci_original(i));
	}
}
