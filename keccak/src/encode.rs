use std::{
	fmt::{self, Debug, Formatter},
	mem::size_of,
	ops::Deref
};

#[derive(Clone, Copy)]
pub struct RightEncoded<const LEN: usize> {
	buf: [u8; LEN],
	offset: u8
}

impl<const LEN: usize> Debug for RightEncoded<LEN> {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
		Debug::fmt(&**self, fmt)
	}
}

impl<const LEN: usize> Deref for RightEncoded<LEN> {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&self.buf[usize::from(self.offset)..]
	}
}

impl<const LEN: usize> Eq for RightEncoded<LEN> {}

impl<const LEN: usize> PartialEq for RightEncoded<LEN> {
	fn eq(&self, rhs: &Self) -> bool {
		**self == **rhs
	}
}

macro_rules! right_encode {
	($($ty:ident),*) => { $(
		impl From<$ty> for RightEncoded<{ size_of::<$ty>() + 1 }> {
			fn from(i: $ty) -> Self {
				let mut buf = [0; size_of::<$ty>() + 1];
				buf[..size_of::<$ty>()].copy_from_slice(&i.to_be_bytes());

				let zeros = i.leading_zeros() as usize / 8;
				buf[size_of::<$ty>()] = (size_of::<$ty>() - zeros) as u8;

				debug_assert!((i == 0 && buf[size_of::<$ty>()] == 0) || (i != 0 && buf[size_of::<$ty>()] != 0));
				debug_assert!(buf[zeros..] == [0] || (buf[zeros..][0] != 0 && buf[size_of::<$ty>()] != 0));
				debug_assert!(buf[..zeros].iter().all(|&i| i == 0));

				Self { buf, offset: zeros as u8 }
			}
		}
	)* };
}

right_encode!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
	use std::convert::TryFrom;

	use super::RightEncoded;

	#[test]
	fn right_encode() {
		const TESTS: &[(u128, &[u8])] = &[
			(0, &[0]),
			(1, &[1, 1]),
			(12, &[0x0c, 0x01]),
			(255, &[255, 1]),
			(256, &[1, 0, 2]),
			(65536, &[0x01, 0x00, 0x00, 0x03]),
			(65537, &[0x01, 0x00, 0x01, 0x03]),
			(65538, &[0x01, 0x00, 0x02, 0x03]),
			(u32::MAX as u128, &[0xFF, 0xFF, 0xFF, 0xFF, 4]),
			(u64::MAX as u128, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 8]),
			(
				u128::MAX,
				&[
					0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
					0xFF, 0xFF, 0xFF, 16
				]
			)
		];

		for &(i, right_encoded) in TESTS {
			if let Ok(i) = u8::try_from(i) {
				assert_eq!(&*RightEncoded::from(i), right_encoded);
			}

			if let Ok(i) = u16::try_from(i) {
				assert_eq!(&*RightEncoded::from(i), right_encoded);
			}

			if let Ok(i) = u32::try_from(i) {
				assert_eq!(&*RightEncoded::from(i), right_encoded);
			}

			if let Ok(i) = u64::try_from(i) {
				assert_eq!(&*RightEncoded::from(i), right_encoded);
			}

			if let Ok(i) = usize::try_from(i) {
				assert_eq!(&*RightEncoded::from(i), right_encoded);
			}

			assert_eq!(&*RightEncoded::from(i), right_encoded);
		}
	}
}
