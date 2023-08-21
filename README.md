# gnal_tsur!

.esrever ni edoc etirW

## Features

- [x] Reversed tokens.
- [x] Reversed identifiers and literals.

## Usage

### `lang_rust!`

This macro reverses the order of every token in the code provided to it.

Even though the character order of identifiers and literals are untouched, the characters of punctuations are considered separate and are therefore reversed as well.

```rs
fn fibonacci(n: u32) -> u32 {
	match n {
		0 => 1,
		1 => 1,
		_ => fibonacci(n - 1) + fibonacci(n - 2),
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
```

### `gnal_tsur!`

This macro is like `lang_rust!`, but it also reverses the order of characters in identifiers and literals.

```rs
fn fibonacci(n: u32) -> u32 {
	match n {
		0 => 1,
		1 => 1,
		_ => fibonacci(n - 1) + fibonacci(n - 2),
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
```
