use core::{marker::PhantomData, mem};

use crate::{Permutation, State};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Cyclist<S, P, M> {
	state: S,
	phase: Phase,
	permutation: PhantomData<P>,
	mode: PhantomData<M>
}

macro_rules! absorb_any {
	() => {
		fn absorb_any<const R: usize>(&mut self, buf: &[u8], color: u8) {
			if self.phase != Phase::Up {
				self.state.permute::<P>();
			}

			let mut chunks = buf.chunks(R);
			self.down(chunks.next().unwrap_or_default(), color);

			for chunk in chunks {
				self.state.permute::<P>();
				self.down(chunk, 0)
			}
		}

		fn squeeze_any<const R: usize>(&mut self, mut buf: &mut [u8], color: u8) {
			let (l, r) = buf.split_at_mut(buf.len().min(R));
			buf = r;

			self.up(l, color);

			while !buf.is_empty() {
				let (l, r) = buf.split_at_mut(buf.len().min(R));
				buf = r;

				self.state.xor_in_u8(0, 0x01);
				self.up(l, 0x00);
			}
		}
	};
}

macro_rules! crypt {
	($name:ident, $a:ident, $b:ident) => {
		pub fn $name(&mut self, mut buf: &mut [u8]) {
			let mut color = 0x80;

			loop {
				let (l, r) = buf.split_at_mut(buf.len().min(RATE_SQUEEZE));
				buf = r;

				self.state.xor_in_u8(S::LEN - 1, mem::replace(&mut color, 0x00));
				self.state.permute::<P>();

				self.state.$a(0, l);
				self.state.$b(0, l);

				self.state.xor_in_u8(l.len(), 0x01);

				if buf.is_empty() {
					break;
				}
			}

			self.phase = Phase::Down;
		}
	};
}

impl<S, P, const RATE: usize> Cyclist<S, P, Hash<RATE>>
where
	S: State,
	P: Permutation<S::Inner>
{
	#[track_caller]
	pub fn new() -> Self {
		Self::from_state(S::default())
	}

	#[track_caller]
	pub fn from_state(state: S) -> Self {
		assert!(RATE != 0 && RATE <= S::LEN - 2);
		Self { state, phase: Phase::Up, permutation: PhantomData, mode: PhantomData }
	}

	pub fn absorb(&mut self, buf: &[u8]) {
		self.absorb_any::<RATE>(buf, 0x03);
	}

	pub fn squeeze_into(&mut self, buf: &mut [u8]) {
		assert!(!buf.is_empty());

		self.squeeze_any::<RATE>(buf, 0x00)
	}

	pub fn squeeze<const LEN: usize>(&mut self) -> [u8; LEN] {
		let mut buf = [0; LEN];
		self.squeeze_into(&mut buf);
		buf
	}

	absorb_any!();

	fn down(&mut self, buf: &[u8], color: u8) {
		self.state.xor_in_u8_slice(0, buf);
		self.state.xor_in_u8(buf.len(), 0x01);
		self.state.xor_in_u8(S::LEN - 1, color & 0x01);

		self.phase = Phase::Down;
	}

	fn up(&mut self, buf: &mut [u8], _color: u8) {
		self.state.permute::<P>();
		self.state.xor_out_u8_slice(0, buf);

		self.phase = Phase::Up;
	}
}

impl<S, P, const RATE: usize> Default for Cyclist<S, P, Hash<RATE>>
where
	S: State,
	P: Permutation<S::Inner>
{
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(feature = "zeroize")]
impl<S, P, M> zeroize::Zeroize for Cyclist<S, P, M>
where
	S: zeroize::Zeroize
{
	fn zeroize(&mut self) {
		self.state.zeroize();
		self.phase.zeroize();
	}
}

impl<S, P, const RATE_ABSORB: usize, const RATE_SQUEEZE: usize, const RATCHET: usize>
	Cyclist<S, P, Keyed<RATE_ABSORB, RATE_SQUEEZE, RATCHET>>
where
	S: State,
	P: Permutation<S::Inner>
{
	#[track_caller]
	pub fn with_key(key: &[u8]) -> Self {
		Self::from_state_with_key_id_counter(S::default(), key, &[], &[])
	}

	#[track_caller]
	pub fn with_key_id(key: &[u8], id: &[u8]) -> Self {
		Self::from_state_with_key_id_counter(S::default(), key, id, &[])
	}

	#[track_caller]
	pub fn with_key_id_counter(key: &[u8], id: &[u8], counter: &[u8]) -> Self {
		Self::from_state_with_key_id_counter(S::default(), key, id, counter)
	}

	#[track_caller]
	pub fn from_state_with_key(state: S, key: &[u8]) -> Self {
		Self::from_state_with_key_id_counter(state, key, &[], &[])
	}

	#[track_caller]
	pub fn from_state_with_key_id(state: S, key: &[u8], id: &[u8]) -> Self {
		Self::from_state_with_key_id_counter(state, key, id, &[])
	}

	#[track_caller]
	pub fn from_state_with_key_id_counter(
		mut state: S,
		key: &[u8],
		id: &[u8],
		counter: &[u8]
	) -> Self {
		assert!(S::LEN <= 256);
		assert!(RATE_ABSORB != 0 && RATE_ABSORB <= S::LEN - 2);
		assert!(RATE_SQUEEZE != 0 && RATE_SQUEEZE <= S::LEN - 2);
		assert!(RATCHET != 0 && RATCHET <= S::LEN - 2);
		assert!(key.len() < 256);
		assert!(id.len() < 256);
		assert!(key.len() + id.len() <= RATE_ABSORB - 1);

		state.xor_in_u8_slice(0, key);
		state.xor_in_u8_slice(key.len(), id);
		state.xor_in_u8(key.len() + id.len(), id.len() as u8);
		state.xor_in_u8(key.len() + id.len() + 1, 0x01);
		state.xor_in_u8(S::LEN - 1, 0x02);

		let mut cyclist =
			Self { state, phase: Phase::Down, permutation: PhantomData, mode: PhantomData };

		if !counter.is_empty() {
			cyclist.absorb_any::<1>(counter, 0x00);
		}

		cyclist
	}

	pub fn absorb(&mut self, buf: &[u8]) {
		self.absorb_any::<RATE_ABSORB>(buf, 0x03);
	}

	#[track_caller]
	pub fn squeeze_into(&mut self, buf: &mut [u8]) {
		assert!(!buf.is_empty());
		self.squeeze_any::<RATE_SQUEEZE>(buf, 0x40)
	}

	pub fn squeeze<const LEN: usize>(&mut self) -> [u8; LEN] {
		let mut buf = [0; LEN];
		self.squeeze_into(&mut buf);
		buf
	}

	crypt!(encrypt, xor_in_u8_slice, get_u8_slice);

	crypt!(decrypt, xor_out_u8_slice, xor_in_u8_slice);

	#[track_caller]
	pub fn squeeze_key_into(&mut self, buf: &mut [u8]) {
		assert!(!buf.is_empty());
		self.squeeze_any::<RATE_SQUEEZE>(buf, 0x20);
	}

	#[track_caller]
	pub fn squeeze_key<const LEN: usize>(&mut self) -> [u8; LEN] {
		let mut buf = [0; LEN];
		self.squeeze_key_into(&mut buf);
		buf
	}

	pub fn ratchet(&mut self) {
		let mut buf = [0; RATCHET];
		self.squeeze_any::<RATE_SQUEEZE>(&mut buf, 0x10);
		self.absorb_any::<RATE_ABSORB>(&buf, 0x00);
	}

	absorb_any!();

	fn down(&mut self, buf: &[u8], color: u8) {
		self.state.xor_in_u8_slice(0, buf);
		self.state.xor_in_u8(buf.len(), 0x01);
		self.state.xor_in_u8(S::LEN - 1, color);

		self.phase = Phase::Down;
	}

	fn up(&mut self, buf: &mut [u8], color: u8) {
		self.state.xor_in_u8(S::LEN - 1, color);
		self.state.permute::<P>();
		self.state.xor_out_u8_slice(0, buf);

		self.phase = Phase::Up;
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Phase {
	Up,
	Down
}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for Phase {
	fn zeroize(&mut self) {
		*self = Self::Up;
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Hash<const RATE: usize> {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Keyed<const RATE_ABSORB: usize, const RATE_SQUEEZE: usize, const RATCHET: usize> {}
