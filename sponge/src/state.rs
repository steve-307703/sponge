use std::{fmt::Debug, mem::size_of, slice};

use crate::Permutation;

pub trait State: Default {
	const LEN: usize;
	type Inner;

	fn from_inner(inner: Self::Inner) -> Self;
	fn get_u8_slice(&self, index: usize, buf: &mut [u8]);
	fn xor_out_u8_slice(&self, index: usize, buf: &mut [u8]);
	fn set_u8_slice(&mut self, index: usize, buf: &[u8]);
	fn set_u8_repeated(&mut self, index: usize, b: u8, len: usize);
	fn xor_in_u8_slice(&mut self, index: usize, buf: &[u8]);
	fn xor_in_u8(&mut self, index: usize, b: u8);
	fn permute<P>(&mut self)
	where
		P: Permutation<Self::Inner>;
}

macro_rules! state {
	($name:ident, $ty:ty) => {
		#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<const LEN: usize>([$ty; LEN]);

		impl<const LEN: usize> $name<LEN> {
			pub fn from_ne(mut buf: [$ty; LEN]) -> Self {
				for i in &mut buf {
					*i = i.to_le();
				}

				Self(buf)
			}

			pub fn into_ne(mut self) -> [$ty; LEN] {
				for i in &mut self.0 {
					*i = <$ty>::from_le(*i);
				}

				self.0
			}

			pub fn with_ne<F>(&mut self, f: F)
			where
				F: FnOnce(&mut [$ty; LEN])
			{
				for i in &mut self.0 {
					*i = <$ty>::from_le(*i);
				}

				f(&mut self.0);

				for i in &mut self.0 {
					*i = i.to_le();
				}
			}

			#[allow(trivial_casts)]
			#[inline]
			const fn as_u8s(&self) -> &[u8] {
				unsafe { &*(slice::from_raw_parts(self.0.as_ptr() as *const u8, Self::LEN)) }
			}

			#[allow(trivial_casts)]
			#[inline]
			fn as_mut_u8s(&mut self) -> &mut [u8] {
				unsafe {
					&mut *(slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut u8, Self::LEN))
				}
			}
		}

		impl<const LEN: usize> Default for $name<LEN> {
			#[inline]
			fn default() -> Self {
				Self([0; LEN])
			}
		}

		#[cfg(feature = "zeroize")]
		impl<const LEN: usize> zeroize::DefaultIsZeroes for $name<LEN> {}

		impl<const LEN: usize> State for $name<LEN> {
			const LEN: usize = LEN * size_of::<$ty>();
			type Inner = [$ty; LEN];

			#[inline]
			fn from_inner(inner: Self::Inner) -> Self {
				Self(inner)
			}

			#[inline]
			#[track_caller]
			fn get_u8_slice(&self, index: usize, buf: &mut [u8]) {
				buf.copy_from_slice(&self.as_u8s()[index..][..buf.len()]);
			}

			#[inline]
			#[track_caller]
			fn xor_out_u8_slice(&self, index: usize, buf: &mut [u8]) {
				let len = buf.len();

				for (dst, &src) in buf.iter_mut().zip(&self.as_u8s()[index..][..len]) {
					*dst ^= src;
				}
			}

			#[inline]
			#[track_caller]
			fn set_u8_slice(&mut self, index: usize, buf: &[u8]) {
				self.as_mut_u8s()[index..][..buf.len()].copy_from_slice(buf);
			}

			#[inline]
			#[track_caller]
			fn set_u8_repeated(&mut self, index: usize, b: u8, len: usize) {
				self.as_mut_u8s()[index..][..len].fill(b);
			}

			#[inline]
			#[track_caller]
			fn xor_in_u8_slice(&mut self, index: usize, buf: &[u8]) {
				for (dst, &src) in self.as_mut_u8s()[index..][..buf.len()].iter_mut().zip(buf) {
					*dst ^= src;
				}
			}

			#[inline]
			#[track_caller]
			fn xor_in_u8(&mut self, index: usize, b: u8) {
				self.as_mut_u8s()[index] ^= b;
			}

			fn permute<P>(&mut self)
			where
				P: Permutation<Self::Inner>
			{
				self.with_ne(P::permute);
			}
		}
	};
}

state!(Lsbu8, u8);
state!(Lsbu16, u16);
state!(Lsbu32, u32);
state!(Lsbu64, u64);
state!(Lsbu128, u128);

#[cfg(feature = "zeroize")]
#[derive(Clone, Default)]
pub struct SecretState<S>(zeroize::Zeroizing<S>)
where
	S: State + zeroize::DefaultIsZeroes;

#[cfg(feature = "zeroize")]
impl<S> Debug for SecretState<S>
where
	S: State + zeroize::DefaultIsZeroes
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "SecretState<{}>", std::any::type_name::<S>())
	}
}

#[cfg(feature = "zeroize")]
impl<S> From<S> for SecretState<S>
where
	S: State + zeroize::DefaultIsZeroes
{
	fn from(s: S) -> Self {
		Self(s.into())
	}
}

#[cfg(feature = "zeroize")]
impl<S> State for SecretState<S>
where
	S: State + zeroize::DefaultIsZeroes
{
	const LEN: usize = S::LEN;
	type Inner = S::Inner;

	#[inline]
	fn from_inner(inner: Self::Inner) -> Self {
		Self(S::from_inner(inner).into())
	}

	#[inline]
	#[track_caller]
	fn get_u8_slice(&self, index: usize, buf: &mut [u8]) {
		self.0.get_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn xor_out_u8_slice(&self, index: usize, buf: &mut [u8]) {
		self.0.xor_out_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn set_u8_slice(&mut self, index: usize, buf: &[u8]) {
		self.0.set_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn set_u8_repeated(&mut self, index: usize, b: u8, len: usize) {
		self.0.set_u8_repeated(index, b, len);
	}

	#[inline]
	#[track_caller]
	fn xor_in_u8_slice(&mut self, index: usize, buf: &[u8]) {
		self.0.xor_in_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn xor_in_u8(&mut self, index: usize, b: u8) {
		self.0.xor_in_u8(index, b);
	}

	#[inline]
	fn permute<P>(&mut self)
	where
		P: Permutation<Self::Inner>
	{
		self.0.permute::<P>();
	}
}

impl<S> State for Box<S>
where
	S: State
{
	const LEN: usize = S::LEN;
	type Inner = S::Inner;

	#[inline]
	fn from_inner(inner: Self::Inner) -> Self {
		Self::new(S::from_inner(inner))
	}

	#[inline]
	#[track_caller]
	fn get_u8_slice(&self, index: usize, buf: &mut [u8]) {
		(**self).get_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn xor_out_u8_slice(&self, index: usize, buf: &mut [u8]) {
		(**self).xor_out_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn set_u8_slice(&mut self, index: usize, buf: &[u8]) {
		(**self).set_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn set_u8_repeated(&mut self, index: usize, b: u8, len: usize) {
		(**self).set_u8_repeated(index, b, len);
	}

	#[inline]
	#[track_caller]
	fn xor_in_u8_slice(&mut self, index: usize, buf: &[u8]) {
		(**self).xor_in_u8_slice(index, buf);
	}

	#[inline]
	#[track_caller]
	fn xor_in_u8(&mut self, index: usize, b: u8) {
		(**self).xor_in_u8(index, b);
	}

	#[inline]
	fn permute<P>(&mut self)
	where
		P: Permutation<Self::Inner>
	{
		(**self).permute::<P>();
	}
}
