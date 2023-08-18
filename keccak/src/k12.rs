use std::mem;

use sponge::{state::Lsbu64, suffix, Permutation, State};

use crate::{
	encode::RightEncoded,
	keccakp::Keccak1600,
	sha3::{TurboShake128Squeezer, TurboShakeShake128}
};

const CHUNK_LEN: usize = 8 << 10;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KangarooTwelve<S> {
	root: TurboShakeShake128<S>,
	leaf: TurboShakeShake128<S>,
	chunk: u64,
	length: usize
}

impl KangarooTwelve<Lsbu64<25>> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<S> KangarooTwelve<S>
where
	S: State,
	Keccak1600<12>: Permutation<S::Inner>
{
	pub fn absorb_u8(&mut self, b: u8) {
		debug_assert!(self.length <= CHUNK_LEN);

		if self.chunk == 0 {
			if self.length == CHUNK_LEN {
				self.switch_to_chaining();
			} else {
				self.root.absorb_u8(b);
				self.length += 1;
				return;
			}
		}

		debug_assert!(self.chunk != 0);
		debug_assert!(self.length <= CHUNK_LEN);

		if self.length == CHUNK_LEN {
			self.absorb_chaining_value();
		}

		debug_assert!(self.length < CHUNK_LEN);
		self.leaf.absorb_u8(b);
		self.length += 1;
		debug_assert!(self.length <= CHUNK_LEN);
	}

	pub fn absorb(&mut self, mut buf: &[u8]) {
		if buf.is_empty() {
			return;
		}

		debug_assert!(self.length <= CHUNK_LEN);

		if self.chunk == 0 {
			let (l, r) = buf.split_at(buf.len().min(CHUNK_LEN - self.length));
			buf = r;

			self.root.absorb(l);
			self.length += l.len();
			debug_assert!(self.length <= CHUNK_LEN);

			if !buf.is_empty() {
				self.switch_to_chaining();
			}
		}

		while !buf.is_empty() {
			debug_assert!(self.chunk != 0);
			debug_assert!(self.length <= CHUNK_LEN);

			if self.length == CHUNK_LEN {
				self.absorb_chaining_value();
			}

			debug_assert!(self.length < CHUNK_LEN);
			let (l, r) = buf.split_at(buf.len().min(CHUNK_LEN - self.length));
			buf = r;

			self.leaf.absorb(l);
			self.length += l.len();
			debug_assert!(self.length <= CHUNK_LEN);
		}
	}

	pub fn into_squeezer(mut self) -> TurboShake128Squeezer<S> {
		self.absorb_u8(0);
		self.into_squeezer_impl()
	}

	pub fn into_squeezer_customized(mut self, customization: &[u8]) -> TurboShake128Squeezer<S> {
		self.absorb(customization);
		self.absorb(&RightEncoded::from(customization.len()));
		self.into_squeezer_impl()
	}

	fn switch_to_chaining(&mut self) {
		debug_assert!(self.chunk == 0);
		debug_assert!(self.length == CHUNK_LEN);

		self.root.absorb(&[suffix!(1).into(), 0, 0, 0, 0, 0, 0, 0]);
		self.chunk = 1;
		self.length = 0;
	}

	fn absorb_chaining_value(&mut self) {
		debug_assert!(self.chunk != 0);
		debug_assert!(self.length <= CHUNK_LEN);

		self.root.absorb(&mem::take(&mut self.leaf).squeeze::<32>(suffix!(1, 1, 0)));
		self.chunk += 1;
		self.length = 0;
	}

	fn into_squeezer_impl(mut self) -> TurboShake128Squeezer<S> {
		let suffix = if self.chunk == 0 {
			suffix!(1, 1)
		} else {
			debug_assert!(self.length != 0);
			self.absorb_chaining_value();
			self.root.absorb(&RightEncoded::from(self.chunk - 1));
			self.root.absorb(&[0xFF; 2]);
			suffix!(0, 1)
		};

		self.root.into_squeezer(suffix)
	}

	pub fn squeeze_into(self, buf: &mut [u8]) {
		self.into_squeezer().squeeze_into(buf);
	}

	pub fn squeeze_into_customized(self, customization: &[u8], buf: &mut [u8]) {
		self.into_squeezer_customized(customization).squeeze_into(buf);
	}

	pub fn squeeze<const LEN: usize>(self) -> [u8; LEN] {
		self.into_squeezer().squeeze()
	}

	pub fn squeeze_customized<const LEN: usize>(self, customization: &[u8]) -> [u8; LEN] {
		self.into_squeezer_customized(customization).squeeze()
	}
}

impl<S> Default for KangarooTwelve<S>
where
	S: State,
	Keccak1600<12>: Permutation<S::Inner>
{
	fn default() -> Self {
		Self {
			root: TurboShakeShake128::default(),
			leaf: TurboShakeShake128::default(),
			chunk: 0,
			length: 0
		}
	}
}

#[cfg(feature = "zeroize")]
impl<S> zeroize::Zeroize for KangarooTwelve<S>
where
	S: zeroize::Zeroize
{
	fn zeroize(&mut self) {
		self.root.zeroize();
	}
}
