use sponge::{
	sponge::Squeezer, state::Lsbu64, suffix, Absorb, IntoSqueezer, Permutation, Sponge, Squeeze,
	State, Suffix
};

use crate::keccakp::Keccak1600;

macro_rules! sha3 {
	($sponge:ident, $capacity:expr, $hash_len:expr, $typenum:ident) => {
		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
		pub struct $sponge<S>(Sponge<S, Keccak1600<24>, $capacity, false>);

		impl $sponge<Lsbu64<25>> {
			pub fn new() -> Self {
				Self::default()
			}
		}

		impl<S> $sponge<S>
		where
			S: State + From<Lsbu64<25>>,
			Keccak1600<24>: Permutation<S::Inner>
		{
			pub fn new_wrapped() -> Self {
				Self(Sponge::from_state(S::from(Lsbu64::<25>::default())))
			}
		}

		impl<S> $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			pub fn absorb(&mut self, buf: &[u8]) {
				self.0.absorb(buf);
			}

			#[track_caller]
			pub fn squeeze_into(self, buf: &mut [u8]) {
				assert!(buf.len() == $hash_len);
				self.0.squeeze_into(suffix!(0, 1), buf);
			}

			pub fn squeeze(self) -> [u8; $hash_len] {
				let mut buf = [0; $hash_len];
				self.squeeze_into(&mut buf);
				buf
			}
		}

		impl<S> Default for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			fn default() -> Self {
				Self(Sponge::default())
			}
		}

		impl<S> Absorb for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			fn absorb(&mut self, buf: &[u8]) {
				self.absorb(buf);
			}

			fn absorb_u8(&mut self, b: u8) {
				self.0.absorb_u8(b);
			}
		}

		impl<S> Squeeze for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			fn squeeze_into(self, buf: &mut [u8]) {
				self.squeeze_into(buf);
			}
		}

		#[cfg(feature = "digest")]
		impl<S> digest::Update for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			fn update(&mut self, buf: &[u8]) {
				self.absorb(buf);
			}
		}

		#[cfg(feature = "digest")]
		impl<S> digest::OutputSizeUser for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			type OutputSize = digest::consts::$typenum;
		}

		#[cfg(feature = "digest")]
		impl<S> digest::FixedOutput for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			fn finalize_into(self, out: &mut digest::Output<Self>) {
				self.squeeze_into(out);
			}
		}

		#[cfg(feature = "digest")]
		impl<S> digest::Reset for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			fn reset(&mut self) {
				*self = Self::default();
			}
		}

		#[cfg(feature = "zeroize")]
		impl<S> zeroize::Zeroize for $sponge<S>
		where
			S: zeroize::Zeroize
		{
			fn zeroize(&mut self) {
				self.0.zeroize();
			}
		}
	};
}

macro_rules! shake_impl {
	($sponge:ident, $squeezer:ident, $rounds:expr, $capacity:expr) => {
		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
		pub struct $sponge<S>(Sponge<S, Keccak1600<$rounds>, $capacity, false>);

		pub type $squeezer<S> = Squeezer<S, Keccak1600<$rounds>, $capacity>;

		impl $sponge<Lsbu64<25>> {
			pub fn new() -> Self {
				Self::default()
			}
		}

		impl<S> $sponge<S>
		where
			S: State + From<Lsbu64<25>>,
			Keccak1600<$rounds>: Permutation<S::Inner>
		{
			pub fn new_wrapped() -> Self {
				Self(Sponge::from_state(S::from(Lsbu64::<25>::default())))
			}
		}

		impl<S> $sponge<S>
		where
			S: State,
			Keccak1600<$rounds>: Permutation<S::Inner>
		{
			pub fn absorb(&mut self, buf: &[u8]) {
				self.0.absorb(buf);
			}
		}

		impl<S> Default for $sponge<S>
		where
			S: State,
			Keccak1600<$rounds>: Permutation<S::Inner>
		{
			fn default() -> Self {
				Self(Sponge::default())
			}
		}

		impl<S> Absorb for $sponge<S>
		where
			S: State,
			Keccak1600<$rounds>: Permutation<S::Inner>
		{
			fn absorb(&mut self, buf: &[u8]) {
				self.absorb(buf);
			}

			fn absorb_u8(&mut self, b: u8) {
				self.0.absorb_u8(b);
			}
		}

		#[cfg(feature = "digest")]
		impl<S> digest::Update for $sponge<S>
		where
			S: State,
			Keccak1600<$rounds>: Permutation<S::Inner>
		{
			fn update(&mut self, buf: &[u8]) {
				self.absorb(buf);
			}
		}

		#[cfg(feature = "digest")]
		impl<S> digest::Reset for $sponge<S>
		where
			S: State,
			Keccak1600<$rounds>: Permutation<S::Inner>
		{
			fn reset(&mut self) {
				*self = Self::default();
			}
		}

		#[cfg(feature = "zeroize")]
		impl<S> zeroize::Zeroize for $sponge<S>
		where
			S: zeroize::Zeroize
		{
			fn zeroize(&mut self) {
				self.0.zeroize();
			}
		}
	};
}

macro_rules! shake {
	($sponge:ident, $squeezer:ident, $capacity:expr) => {
		shake_impl!($sponge, $squeezer, 24, $capacity);

		impl<S> $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			pub fn into_squeezer(self) -> $squeezer<S> {
				self.0.into_squeezer(suffix!(1, 1, 1, 1))
			}

			pub fn squeeze_into(self, buf: &mut [u8]) {
				self.into_squeezer().squeeze_into(buf);
			}

			pub fn squeeze<const LEN: usize>(self) -> [u8; LEN] {
				self.into_squeezer().squeeze()
			}
		}

		impl<S> IntoSqueezer for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			type Squeezer = $squeezer<S>;

			fn into_squeezer(self) -> Self::Squeezer {
				self.into_squeezer()
			}
		}

		#[cfg(feature = "digest")]
		impl<S> digest::ExtendableOutput for $sponge<S>
		where
			S: State,
			Keccak1600<24>: Permutation<S::Inner>
		{
			type Reader = $squeezer<S>;

			fn finalize_xof(self) -> Self::Reader {
				self.into_squeezer()
			}
		}
	};
}

macro_rules! turbo_shake {
	($sponge:ident, $squeezer:ident, $capacity:expr) => {
		shake_impl!($sponge, $squeezer, 12, $capacity);

		impl<S> $sponge<S>
		where
			S: State,
			Keccak1600<12>: Permutation<S::Inner>
		{
			pub fn into_squeezer(self, suffix: Suffix) -> $squeezer<S> {
				self.0.into_squeezer(suffix)
			}

			pub fn squeeze_into(self, suffix: Suffix, buf: &mut [u8]) {
				self.into_squeezer(suffix).squeeze_into(buf);
			}

			pub fn squeeze<const LEN: usize>(self, suffix: Suffix) -> [u8; LEN] {
				self.into_squeezer(suffix).squeeze()
			}
		}
	};
}

sha3!(Sha3_224, { 224 * 2 / 8 }, 224 / 8, U28);
sha3!(Sha3_256, { 256 * 2 / 8 }, 256 / 8, U32);
sha3!(Sha3_384, { 384 * 2 / 8 }, 384 / 8, U48);
sha3!(Sha3_512, { 512 * 2 / 8 }, 512 / 8, U64);

shake!(Shake128, Shake128Squeezer, { 128 * 2 / 8 });
shake!(Shake256, Shake256Squeezer, { 256 * 2 / 8 });

turbo_shake!(TurboShakeShake128, TurboShake128Squeezer, { 128 * 2 / 8 });
turbo_shake!(TurboShakeShake256, TurboShake256Squeezer, { 256 * 2 / 8 });
