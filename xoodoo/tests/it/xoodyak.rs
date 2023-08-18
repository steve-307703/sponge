// https://github.com/jedisct1/rust-xoodyak/blob/master/src/test.rs

use sponge::state::Lsbu32;
use xoodoo::{XoodyakHash, XoodyakKeyed};

#[test]
fn test_keyed_empty() {
	assert_eq!(
		XoodyakKeyed::<Lsbu32<12>>::with_key(b"key").squeeze::<32>(),
		[
			106, 247, 180, 176, 207, 217, 130, 200, 237, 113, 163, 185, 224, 53, 120, 137, 251,
			126, 216, 3, 87, 45, 239, 214, 41, 201, 246, 56, 83, 55, 18, 108
		]
	);
}

#[test]
fn test_unkeyed_empty() {
	assert_eq!(
		XoodyakHash::<Lsbu32<12>>::new().squeeze(),
		[
			141, 216, 213, 137, 191, 252, 99, 169, 25, 45, 35, 27, 20, 160, 165, 255, 204, 246, 41,
			214, 87, 39, 76, 114, 39, 130, 131, 52, 124, 189, 128, 53
		]
	);

	let mut xoodyak = XoodyakHash::<Lsbu32<12>>::new();
	xoodyak.absorb(&[]);

	assert_eq!(
		xoodyak.squeeze(),
		[
			234, 21, 47, 43, 71, 188, 226, 78, 251, 102, 196, 121, 212, 173, 241, 123, 211, 36,
			216, 6, 232, 95, 247, 94, 227, 105, 238, 80, 220, 143, 139, 209
		]
	);
}

#[test]
fn test_unkeyed_hash() {
	const M: &[u8] = b"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";

	let mut xoodyak = XoodyakHash::<Lsbu32<12>>::new();

	xoodyak.absorb(M);

	assert_eq!(
		xoodyak.squeeze(),
		[
			144, 82, 141, 27, 59, 215, 34, 104, 197, 106, 251, 142, 112, 235, 111, 168, 19, 6, 112,
			222, 160, 168, 230, 38, 27, 229, 248, 179, 94, 227, 247, 25
		]
	);

	xoodyak.absorb(M);

	assert_eq!(
		xoodyak.squeeze(),
		[
			102, 50, 250, 132, 79, 91, 248, 161, 121, 248, 225, 33, 105, 159, 111, 230, 135, 252,
			43, 228, 152, 41, 58, 242, 211, 252, 29, 234, 181, 0, 196, 220
		]
	);
}

// https://github.com/XKCP/XKCP/blob/master/tests/UnitTests/testXoodyak.c

fn gen_data(len: usize, seed1: u8, seed2: u8) -> Vec<u8> {
	debug_assert!(seed2 < 8);

	(0..len)
		.map(|i| {
			let rolled = ((i as u8) << seed2) | ((i as u8) >> (8 - seed2));

			seed1
				.wrapping_add(161_u8.wrapping_mul(len as u8))
				.wrapping_sub(rolled)
				.wrapping_add(i as u8)
		})
		.collect()
}

#[test]
fn xoodyak_hash_generated() {
	fn hash_one(
		global: &mut XoodyakHash<Lsbu32<12>>,
		msgs: usize,
		msg_len: usize,
		hash_len: usize
	) {
		let mut xoodyak = XoodyakHash::<Lsbu32<12>>::new();

		for _ in 0..msgs {
			xoodyak.absorb(&gen_data(msg_len, (msg_len + hash_len + 0x12) as u8, 3));
		}

		let mut hash = vec![0; hash_len];
		xoodyak.squeeze_into(&mut hash);

		global.absorb(&hash);
	}

	let mut global = XoodyakHash::new();

	for msgs in 1..3 {
		for msg_len in 0..=3 * 48 {
			hash_one(&mut global, msgs, msg_len, 32);
		}
	}

	for hash_len in 1..=3 * 48 {
		hash_one(&mut global, 1, 3 * 48 + 1, hash_len);
	}

	assert_eq!(
		global.squeeze(),
		[
			0x72, 0xbb, 0x07, 0xae, 0x9c, 0xae, 0x32, 0xb3, 0x0e, 0xa4, 0x73, 0x65, 0x67, 0x01,
			0xf3, 0xd8, 0x25, 0xbd, 0x56, 0x82, 0x1b, 0xb6, 0xa4, 0x5d, 0x2c, 0xba, 0xbc, 0x50,
			0x78, 0xab, 0x4c, 0x7a
		]
	);
}

#[allow(clippy::too_many_arguments)]
#[test]
fn xoodyak_keyed_generated() {
	fn keyed_one(
		global: &mut XoodyakHash<Lsbu32<12>>,
		key: &[u8],
		id: &[u8],
		nonce: &[u8],
		metadata: &[u8],
		data: &mut [u8],
		msgs: usize,
		key_variant: usize,
		ratchet: usize,
		squeeze_key_len: usize
	) {
		let data2 = data.to_vec();

		let mut encrypt = match key_variant {
			0 => {
				let mut tmp = XoodyakKeyed::<Lsbu32<12>>::with_key_id(key, id);
				tmp.absorb(nonce);
				tmp
			}
			1 => XoodyakKeyed::with_key_id_counter(key, id, nonce),
			_ => unreachable!()
		};

		let mut decrypt = encrypt;

		for _ in 0..msgs {
			let mut key = vec![0; squeeze_key_len];

			if !key.is_empty() {
				encrypt.squeeze_key_into(&mut key);
				global.absorb(&key);
			}

			encrypt.absorb(metadata);
			encrypt.encrypt(data);
			global.absorb(data);

			if ratchet == 1 {
				encrypt.ratchet();
			}

			let tag = encrypt.squeeze::<16>();
			global.absorb(&tag);

			if ratchet == 2 {
				encrypt.ratchet();
			}

			let mut key_prime = vec![0; squeeze_key_len];

			if !key_prime.is_empty() {
				decrypt.squeeze_key_into(&mut key_prime);
			}

			decrypt.absorb(metadata);
			decrypt.decrypt(data);

			if ratchet == 1 {
				decrypt.ratchet();
			}

			let tag_prime = decrypt.squeeze::<16>();

			if ratchet == 2 {
				decrypt.ratchet();
			}

			assert_eq!(key, key_prime);
			assert_eq!(data, data2);
			assert_eq!(tag, tag_prime);
		}
	}

	let mut global = XoodyakHash::new();

	for key_variant in 0..=1 {
		for ratchet in 0..=2 {
			for squeeze_key_len in (0..48 - 4).step_by(16) {
				for key_len in 16..48 - 4 {
					for nonce_len in (0..=16).step_by(if key_len == 16 { 1 } else { 16 }) {
						let key = gen_data(key_len, (key_len + nonce_len + 0x12) as u8, 3);
						let nonce = gen_data(nonce_len, (key_len + nonce_len + 0x45) as u8, 6);

						let c = 0x1234
							+ key_variant + 3 * ratchet + 5 * squeeze_key_len
							+ 9 * nonce_len;

						let id_len = if (key_len <= 16) || (key_variant == 2) {
							0
						} else {
							c % (key_len - 15)
						};

						let id = &key[key_len - id_len..];
						let mut data = *b"DEF";

						keyed_one(
							&mut global,
							&key[..key_len - id_len],
							&id[..id_len],
							&nonce,
							b"ABC",
							&mut data,
							1,
							key_variant,
							ratchet,
							squeeze_key_len
						);
					}
				}
			}
		}
	}

	{
		for ratchet in 0..=2 {
			for msgs in 1..=3 {
				for metadata_len in [0, 1, 48 - 4 - 1, 48 - 4, 48 - 4 + 1] {
					for data_len in (0..=3 * 48 + 1).step_by(if metadata_len == 0 {
						1
					} else {
						ratchet * 4 + 1
					}) {
						let key = gen_data(16, (data_len + metadata_len + 0x23) as u8, 4);
						let nonce = gen_data(16, (data_len + metadata_len + 0x56) as u8, 7);
						let mut metadata =
							gen_data(metadata_len, (data_len + metadata_len + 0xAB) as u8, 3);
						let data = gen_data(data_len, (data_len + metadata_len + 0xCD) as u8, 4);

						keyed_one(
							&mut global,
							&key,
							&[],
							&nonce,
							&data,
							&mut metadata,
							msgs,
							0,
							ratchet,
							0
						);
					}
				}
			}
		}
	}

	{
		for ratchet in 0..=2 {
			for msgs in 1..=3 {
				for data_len in [0, 1, 24 - 1, 24, 24 + 1] {
					for metadata_len in
						(0..=3 * 48 + 1).step_by(if data_len == 0 { 1 } else { ratchet * 4 + 1 })
					{
						let key = gen_data(16, (data_len + metadata_len + 0x34) as u8, 5);
						let nonce = gen_data(16, (data_len + metadata_len + 0x45) as u8, 6);
						let mut metadata =
							gen_data(metadata_len, (data_len + metadata_len + 0x01) as u8, 5);
						let data = gen_data(data_len, (data_len + metadata_len + 0x23) as u8, 6);

						keyed_one(
							&mut global,
							&key,
							&[],
							&nonce,
							&data,
							&mut metadata,
							msgs,
							0,
							ratchet,
							0
						);
					}
				}
			}
		}
	}

	assert_eq!(
		global.squeeze(),
		[
			0xaa, 0x2c, 0x40, 0x75, 0x31, 0x3f, 0xce, 0x6a, 0x55, 0xed, 0xa0, 0x40, 0xf9, 0xd0,
			0x02, 0x54, 0x0e, 0x4b, 0xd1, 0x2e, 0xa0, 0x8d, 0x52, 0x3d, 0x48, 0x86, 0x34, 0xe2,
			0x97, 0x89, 0xd6, 0xd8
		]
	);
}
