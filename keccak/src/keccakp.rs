use sponge::Permutation;

macro_rules! permutation {
	($name:ident, $lane:ty, $make_rcs:ident, $keccakf:ident) => {
		#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<const ROUNDS: usize>;

		impl<const ROUNDS: usize> $name<ROUNDS> {
			pub const RCS: [$lane; ROUNDS] = $make_rcs::<ROUNDS>();
		}

		impl<const ROUNDS: usize> Permutation<[$lane; 25]> for $name<ROUNDS> {
			#[inline]
			fn permute(state: &mut [$lane; 25]) {
				$keccakf(state, &Self::RCS);
			}
		}
	};
}

permutation!(Keccak200, u8, make_rcs_u8, keccakp_200);
permutation!(Keccak400, u16, make_rcs_u16, keccakp_400);
permutation!(Keccak800, u32, make_rcs_u32, keccakp_800);
permutation!(Keccak1600, u64, make_rcs_u64, keccakp_1600);

macro_rules! keccakp {
	($name:ident, $lane:ty) => {
		#[inline(always)]
		pub fn $name(a: &mut [$lane; 5 * 5], rcs: &[$lane]) {
			#[inline(always)]
			fn theta(a: &mut [$lane; 5 * 5]) {
				let mut b = [0; 5];

				for x in 0..5 {
					for y in 0..5 {
						b[x] ^= a[x + y * 5];
					}
				}

				for x in 0..5 {
					for y in 0..5 {
						a[x + y * 5] ^= b[(x + 4) % 5] ^ b[(x + 1) % 5].rotate_left(1);
					}
				}
			}

			#[inline(always)]
			fn rho_and_pi(a: &mut [$lane; 5 * 5]) {
				const RHO: [u32; 24] = [
					1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39,
					61, 20, 44
				];

				const PI: [usize; 24] = [
					10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22,
					9, 6, 1
				];

				let mut t1 = a[1];
				for i in 0..24 {
					let t2 = a[PI[i]];
					a[PI[i]] = t1.rotate_left(RHO[i]);
					t1 = t2;
				}
			}

			#[inline(always)]
			fn chi(a: &mut [$lane; 5 * 5]) {
				for y in 0..5 {
					let mut b = [0; 5];

					for x in 0..5 {
						b[x] = a[x + y * 5];
					}

					for x in 0..5 {
						a[x + y * 5] = b[x] ^ (!b[(x + 1) % 5] & b[(x + 2) % 5]);
					}
				}
			}

			for rc in rcs {
				theta(a);
				rho_and_pi(a);
				chi(a);

				// iota
				a[0] ^= rc;
			}
		}
	};
}

keccakp!(keccakp_200, u8);
keccakp!(keccakp_400, u16);
keccakp!(keccakp_800, u32);
keccakp!(keccakp_1600, u64);

const fn rc_bit(round: i32) -> bool {
	let mut r: u8 = 1;

	let mut i = 0;
	while i < round.rem_euclid(255) {
		r = (r << 1) ^ ((r >> 7) * 0x71);
		i += 1;
	}

	r & 1 == 1
}

macro_rules! make_rcs {
	($name:ident, $lane:ty, $l:literal) => {
		const fn $name<const ROUNDS: usize>() -> [$lane; ROUNDS] {
			let mut rcs = [0; ROUNDS];
			let offset = (12 + 2 * $l) - ROUNDS.rem_euclid(255) as i32;

			let mut i = 0;
			while i < ROUNDS {
				let mut rc: $lane = 0;
				let round = offset + i.rem_euclid(255) as i32;

				let mut j = 0;
				while j <= $l {
					rc |= (rc_bit(7 * round + j) as $lane) << ((1 << j) - 1);
					j += 1;
				}

				rcs[i] = rc;
				i += 1;
			}

			rcs
		}
	};
}

make_rcs!(make_rcs_u8, u8, 3);
make_rcs!(make_rcs_u16, u16, 4);
make_rcs!(make_rcs_u32, u32, 5);
make_rcs!(make_rcs_u64, u64, 6);

#[cfg(test)]
mod tests {
	#[test]
	fn make_rcs_u64() {
		const RCS: [u64; 30] = [
			0x0000000080008081,
			0x0000000000008003,
			0x0000000000008081,
			0x8000000080008000,
			0x0000000000008002,
			0x000000000000008A,
			0x0000000000000001,
			0x0000000000008082,
			0x800000000000808A,
			0x8000000080008000,
			0x000000000000808B,
			0x0000000080000001,
			0x8000000080008081,
			0x8000000000008009,
			0x000000000000008A,
			0x0000000000000088,
			0x0000000080008009,
			0x000000008000000A,
			0x000000008000808B,
			0x800000000000008B,
			0x8000000000008089,
			0x8000000000008003,
			0x8000000000008002,
			0x8000000000000080,
			0x000000000000800A,
			0x800000008000000A,
			0x8000000080008081,
			0x8000000000008080,
			0x0000000080000001,
			0x8000000080008008
		];

		assert_eq!(super::Keccak1600::<30>::RCS, RCS);
		assert_eq!(super::Keccak1600::<24>::RCS, &RCS[6..]);
		assert_eq!(super::Keccak1600::<12>::RCS, &RCS[18..]);
	}

	#[test]
	fn make_rcs_u32() {
		const RCS: [u32; 22] = [
			0x00000001, 0x00008082, 0x0000808A, 0x80008000, 0x0000808B, 0x80000001, 0x80008081,
			0x00008009, 0x0000008A, 0x00000088, 0x80008009, 0x8000000A, 0x8000808B, 0x0000008B,
			0x00008089, 0x00008003, 0x00008002, 0x00000080, 0x0000800A, 0x8000000A, 0x80008081,
			0x00008080
		];

		assert_eq!(super::Keccak800::<22>::RCS, RCS);
	}

	#[test]
	fn make_rcs_u16() {
		const RCS: [u16; 20] = [
			0x0001, 0x8082, 0x808A, 0x8000, 0x808B, 0x0001, 0x8081, 0x8009, 0x008A, 0x0088, 0x8009,
			0x000A, 0x808B, 0x008B, 0x8089, 0x8003, 0x8002, 0x0080, 0x800A, 0x000A
		];

		assert_eq!(super::Keccak400::<20>::RCS, RCS);
	}

	#[test]
	fn make_rcs_u8() {
		const RCS: [u8; 18] = [
			0x01, 0x82, 0x8A, 0x00, 0x8B, 0x01, 0x81, 0x09, 0x8A, 0x88, 0x09, 0x0A, 0x8B, 0x8B,
			0x89, 0x03, 0x02, 0x80
		];

		assert_eq!(super::Keccak200::<18>::RCS, RCS);
	}
}
