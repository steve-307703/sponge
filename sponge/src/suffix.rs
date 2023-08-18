#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Suffix(u8);

impl Suffix {
	#[inline]
	pub const fn from_bits_1(a: bool) -> Self {
		Self((a as u8) | 1 << 1)
	}

	#[inline]
	pub const fn from_bits_2(a: bool, b: bool) -> Self {
		Self((a as u8) | (b as u8) << 1 | 1 << 2)
	}

	#[inline]
	pub const fn from_bits_3(a: bool, b: bool, c: bool) -> Self {
		Self((a as u8) | (b as u8) << 1 | (c as u8) << 2 | 1 << 3)
	}

	#[inline]
	pub const fn from_bits_4(a: bool, b: bool, c: bool, d: bool) -> Self {
		Self((a as u8) | (b as u8) << 1 | (c as u8) << 2 | (d as u8) << 3 | 1 << 4)
	}

	#[allow(clippy::many_single_char_names)]
	#[inline]
	pub const fn from_bits_5(a: bool, b: bool, c: bool, d: bool, e: bool) -> Self {
		Self((a as u8) | (b as u8) << 1 | (c as u8) << 2 | (d as u8) << 3 | (e as u8) << 4 | 1 << 5)
	}

	#[allow(clippy::many_single_char_names)]
	#[inline]
	pub const fn from_bits_6(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> Self {
		Self(
			(a as u8)
				| (b as u8) << 1 | (c as u8) << 2
				| (d as u8) << 3 | (e as u8) << 4
				| (f as u8) << 5 | 1 << 6
		)
	}
}

impl Default for Suffix {
	#[inline]
	fn default() -> Self {
		Self(1)
	}
}

impl From<Suffix> for u8 {
	#[inline]
	fn from(s: Suffix) -> Self {
		s.0
	}
}

#[macro_export]
macro_rules! suffix {
	(check 0) => { false };
	(check 1) => { true };
	(check $a:tt) => { compile_error!(concat!("expected 0 or 1, found `", stringify!($a), "`")); };

	() => { $crate::Suffix::default() };

	($a:tt) => {
		$crate::Suffix::from_bits_1(
			suffix!(check $a)
		)
	};

	($a:tt, $b:tt) => {
		$crate::Suffix::from_bits_2(
			suffix!(check $a),
			suffix!(check $b)
		)
	};

	($a:tt, $b:tt, $c:tt) => {
		$crate::Suffix::from_bits_3(
			suffix!(check $a),
			suffix!(check $b),
			suffix!(check $c)
		)
	};

	($a:tt, $b:tt, $c:tt, $d:tt) => {
		$crate::Suffix::from_bits_4(
			suffix!(check $a),
			suffix!(check $b),
			suffix!(check $c),
			suffix!(check $d)
		)
	};

	($a:tt, $b:tt, $c:tt, $d:tt, $e:tt) => {
		$crate::Suffix::from_bits_5(
			suffix!(check $a),
			suffix!(check $b),
			suffix!(check $c),
			suffix!(check $d),
			suffix!(check $e)
		)
	};

	($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt) => {
		$crate::Suffix::from_bits_6(
			suffix!(check $a),
			suffix!(check $b),
			suffix!(check $c),
			suffix!(check $d),
			suffix!(check $e),
			suffix!(check $f)
		)
	};
}
