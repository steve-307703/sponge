use sponge::Permutation;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Xoodoo;

impl Xoodoo {
	pub const RCS: [u32; 12] = [
		0x00000058, 0x00000038, 0x000003C0, 0x000000D0, 0x00000120, 0x00000014, 0x00000060,
		0x0000002C, 0x00000380, 0x000000F0, 0x000001A0, 0x00000012
	];
}

impl Permutation<[u32; 12]> for Xoodoo {
	#[inline]
	fn permute(state: &mut [u32; 12]) {
		xoodoo(state, &Self::RCS);
	}
}

#[allow(clippy::identity_op)]
#[inline]
pub fn xoodoo(s: &mut [u32; 12], rcs: &[u32]) {
	let mut v1;
	let mut v2;

	for &rc in rcs {
		// theta
		v1 = s[0 + 3] ^ s[4 + 3] ^ s[8 + 3];
		v2 = s[0 + 0] ^ s[4 + 0] ^ s[8 + 0];
		v1 = v1.rotate_left(5) ^ v1.rotate_left(14);
		s[0 + 0] ^= v1;
		s[4 + 0] ^= v1;
		s[8 + 0] ^= v1;
		v1 = s[0 + 1] ^ s[4 + 1] ^ s[8 + 1];
		v2 = v2.rotate_left(5) ^ v2.rotate_left(14);
		s[0 + 1] ^= v2;
		s[4 + 1] ^= v2;
		s[8 + 1] ^= v2;
		v2 = s[0 + 2] ^ s[4 + 2] ^ s[8 + 2];
		v1 = v1.rotate_left(5) ^ v1.rotate_left(14);
		s[0 + 2] ^= v1;
		s[4 + 2] ^= v1;
		s[8 + 2] ^= v1;
		v2 = v2.rotate_left(5) ^ v2.rotate_left(14);
		s[0 + 3] ^= v2;
		s[4 + 3] ^= v2;
		s[8 + 3] ^= v2;

		// rho-west
		s[8 + 0] = s[8 + 0].rotate_left(11);
		s[8 + 1] = s[8 + 1].rotate_left(11);
		s[8 + 2] = s[8 + 2].rotate_left(11);
		s[8 + 3] = s[8 + 3].rotate_left(11);
		v1 = s[4 + 3];
		s[4 + 3] = s[4 + 2];
		s[4 + 2] = s[4 + 1];
		s[4 + 1] = s[4 + 0];
		s[4 + 0] = v1;

		// iota
		s[0 + 0] ^= rc;

		// chi
		s[0 + 0] ^= !s[4 + 0] & s[8 + 0];
		s[4 + 0] ^= !s[8 + 0] & s[0 + 0];
		s[8 + 0] ^= !s[0 + 0] & s[4 + 0];

		s[0 + 1] ^= !s[4 + 1] & s[8 + 1];
		s[4 + 1] ^= !s[8 + 1] & s[0 + 1];
		s[8 + 1] ^= !s[0 + 1] & s[4 + 1];

		s[0 + 2] ^= !s[4 + 2] & s[8 + 2];
		s[4 + 2] ^= !s[8 + 2] & s[0 + 2];
		s[8 + 2] ^= !s[0 + 2] & s[4 + 2];

		s[0 + 3] ^= !s[4 + 3] & s[8 + 3];
		s[4 + 3] ^= !s[8 + 3] & s[0 + 3];
		s[8 + 3] ^= !s[0 + 3] & s[4 + 3];

		// rho-east
		s[4 + 0] = s[4 + 0].rotate_left(1);
		s[4 + 1] = s[4 + 1].rotate_left(1);
		s[4 + 2] = s[4 + 2].rotate_left(1);
		s[4 + 3] = s[4 + 3].rotate_left(1);
		v1 = s[8 + 3].rotate_left(8);
		s[8 + 3] = s[8 + 1].rotate_left(8);
		s[8 + 1] = v1;
		v1 = s[8 + 2].rotate_left(8);
		s[8 + 2] = s[8 + 0].rotate_left(8);
		s[8 + 0] = v1;
	}
}
