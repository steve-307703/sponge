use core::marker::PhantomData;

use crate::{Absorb, Permutation, State, Suffix};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Sponge<S, P, const CAPACITY: usize, const FULL_STATE: bool>(Inner<S, P, CAPACITY>);

impl<S, P, const CAPACITY: usize, const FULL_STATE: bool> Sponge<S, P, CAPACITY, FULL_STATE>
where
	S: State,
	P: Permutation<S::Inner>
{
	pub const ABSORB_RATE: usize = if FULL_STATE { S::LEN } else { S::LEN - CAPACITY };
	pub const SQUEEZE_RATE: usize = S::LEN - CAPACITY;

	#[track_caller]
	pub fn from_state(state: S) -> Self {
		Self(Inner::from_state(state))
	}

	pub fn absorb_u8(&mut self, b: u8) {
		self.check_permute();
		self.0.state.xor_in_u8(self.0.index, b);
		self.0.index += 1;
	}

	pub fn absorb(&mut self, mut buf: &[u8]) {
		while !buf.is_empty() {
			self.check_permute();

			let len = buf.len().min(Self::ABSORB_RATE - self.0.index);
			let (a, b) = buf.split_at(len);
			buf = b;

			self.0.state.xor_in_u8_slice(self.0.index, a);
			self.0.index += a.len();
		}
	}

	pub fn absorb_zeroes(&mut self, mut zeroes: usize) {
		while zeroes != 0 {
			self.check_permute();
			let len = zeroes.min(Self::ABSORB_RATE - self.0.index);
			self.0.index += len;
			zeroes -= len;
		}
	}

	pub fn pad_simple(&mut self, suffix: Suffix) {
		debug_assert!(u8::from(suffix) != 0);
		debug_assert!(u8::from(suffix) & 0b1000_0000 == 0);

		self.absorb_u8(suffix.into());
		self.0.index = Self::ABSORB_RATE;
	}

	pub fn pad_multi_rate(&mut self, suffix: Suffix) {
		self.pad_simple(suffix);
		self.0.state.xor_in_u8(Self::ABSORB_RATE - 1, 0b1000_0000);
		self.0.index = Self::ABSORB_RATE;
	}

	pub fn permute(&mut self) {
		self.0.permute();
	}

	pub fn into_squeezer(mut self, suffix: Suffix) -> Squeezer<S, P, CAPACITY> {
		self.pad_multi_rate(suffix);
		self.0.index = Self::SQUEEZE_RATE;
		Squeezer(self.0)
	}

	pub fn squeeze_into(self, suffix: Suffix, buf: &mut [u8]) {
		self.into_squeezer(suffix).squeeze_into(buf);
	}

	pub fn squeeze<const LEN: usize>(self, suffix: Suffix) -> [u8; LEN] {
		self.into_squeezer(suffix).squeeze()
	}

	fn check_permute(&mut self) {
		debug_assert!(self.0.index <= Self::ABSORB_RATE);

		if self.0.index == Self::ABSORB_RATE {
			self.permute();
		}
	}
}

impl<S, P, const CAPACITY: usize, const FULL_STATE: bool> Default
	for Sponge<S, P, CAPACITY, FULL_STATE>
where
	S: State
{
	#[track_caller]
	fn default() -> Self {
		Self(Inner::default())
	}
}

impl<S, P, const CAPACITY: usize, const FULL_STATE: bool> Absorb
	for Sponge<S, P, CAPACITY, FULL_STATE>
where
	S: State,
	P: Permutation<S::Inner>
{
	fn absorb(&mut self, buf: &[u8]) {
		self.absorb(buf);
	}

	fn absorb_u8(&mut self, b: u8) {
		self.absorb_u8(b);
	}
}

#[cfg(feature = "digest")]
impl<S, P, const CAPACITY: usize, const FULL_STATE: bool> digest::Update
	for Sponge<S, P, CAPACITY, FULL_STATE>
where
	S: State,
	P: Permutation<S::Inner>
{
	fn update(&mut self, data: &[u8]) {
		self.absorb(data);
	}
}

#[cfg(feature = "digest")]
impl<S, P, const CAPACITY: usize, const FULL_STATE: bool> digest::Reset
	for Sponge<S, P, CAPACITY, FULL_STATE>
where
	S: State
{
	fn reset(&mut self) {
		*self = Self::default()
	}
}

#[cfg(feature = "zeroize")]
impl<S, P, const CAPACITY: usize, const FULL_STATE: bool> zeroize::Zeroize
	for Sponge<S, P, CAPACITY, FULL_STATE>
where
	S: zeroize::Zeroize
{
	fn zeroize(&mut self) {
		self.0.zeroize();
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Squeezer<S, P, const CAPACITY: usize>(Inner<S, P, CAPACITY>);

impl<S, P, const CAPACITY: usize> Squeezer<S, P, CAPACITY>
where
	S: State,
	P: Permutation<S::Inner>
{
	pub const RATE: usize = S::LEN - CAPACITY;

	pub fn squeeze_into(&mut self, mut buf: &mut [u8]) {
		while !buf.is_empty() {
			self.check_permute();

			let len = buf.len().min(Self::RATE - self.0.index);
			let (a, b) = buf.split_at_mut(len);
			buf = b;

			self.0.state.get_u8_slice(self.0.index, a);
			self.0.index += a.len();
		}
	}

	pub fn squeeze<const LEN: usize>(&mut self) -> [u8; LEN] {
		let mut buf = [0; LEN];
		self.squeeze_into(&mut buf);
		buf
	}

	fn check_permute(&mut self) {
		debug_assert!(self.0.index <= Self::RATE);

		if self.0.index == Self::RATE {
			self.0.permute();
		}
	}
}

impl<S, P, const CAPACITY: usize> crate::Squeezer for Squeezer<S, P, CAPACITY>
where
	S: State,
	P: Permutation<S::Inner>
{
	fn squeeze_into(&mut self, buf: &mut [u8]) {
		self.squeeze_into(buf);
	}
}

#[cfg(feature = "digest")]
impl<S, P, const CAPACITY: usize> digest::XofReader for Squeezer<S, P, CAPACITY>
where
	S: State,
	P: Permutation<S::Inner>
{
	fn read(&mut self, buf: &mut [u8]) {
		self.squeeze_into(buf);
	}
}

#[cfg(feature = "rand")]
impl<S, P, const CAPACITY: usize> rand_core::RngCore for Squeezer<S, P, CAPACITY>
where
	S: State,
	P: Permutation<S::Inner>
{
	fn next_u32(&mut self) -> u32 {
		u32::from_le_bytes(self.squeeze())
	}

	fn next_u64(&mut self) -> u64 {
		u64::from_le_bytes(self.squeeze())
	}

	fn fill_bytes(&mut self, buf: &mut [u8]) {
		self.squeeze_into(buf);
	}

	fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), rand_core::Error> {
		self.squeeze_into(buf);
		Ok(())
	}
}

#[cfg(feature = "rand")]
impl<S, P, const CAPACITY: usize> rand_core::CryptoRng for Squeezer<S, P, CAPACITY>
where
	S: State,
	P: Permutation<S::Inner>
{
}

#[cfg(feature = "zeroize")]
impl<S, P, const CAPACITY: usize> zeroize::Zeroize for Squeezer<S, P, CAPACITY>
where
	S: zeroize::Zeroize
{
	fn zeroize(&mut self) {
		self.0.zeroize();
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Inner<S, P, const CAPACITY: usize> {
	state: S,
	index: usize,
	permutation: PhantomData<P>
}

impl<S, P, const CAPACITY: usize> Inner<S, P, CAPACITY>
where
	S: State,
	P: Permutation<S::Inner>
{
	#[track_caller]
	fn from_state(state: S) -> Self {
		assert!(CAPACITY < S::LEN, "capacity must be less than the state length");

		Self { state, index: 0, permutation: PhantomData }
	}

	fn permute(&mut self) {
		self.state.permute::<P>();
		self.index = 0;
	}
}

impl<S, P, const CAPACITY: usize> Default for Inner<S, P, CAPACITY>
where
	S: Default
{
	fn default() -> Self {
		Self { state: S::default(), index: 0, permutation: PhantomData }
	}
}

#[cfg(feature = "zeroize")]
impl<S, P, const CAPACITY: usize> zeroize::Zeroize for Inner<S, P, CAPACITY>
where
	S: zeroize::Zeroize
{
	fn zeroize(&mut self) {
		self.state.zeroize();
		self.index.zeroize();
	}
}
