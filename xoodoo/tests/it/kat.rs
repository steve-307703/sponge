use sponge::state::Lsbu32;
use xoodoo::{XoodyakHash, XoodyakKeyed};

#[test]
fn xoodoo_hash() {
	for (i, test) in include_str!("LWC_HASH_KAT_256.txt").split_terminator("\n\n").enumerate() {
		let mut lines = test.split('\n');

		let count = lines.next().unwrap();
		assert!(count.starts_with("Count = ") && i + 1 == count[8..].parse().unwrap());

		let msg = lines.next().unwrap();
		assert!(msg.starts_with("Msg = "));
		let msg = crate::hex::decode(&msg[6..]);

		let hash = lines.next().unwrap();
		assert!(hash.starts_with("MD = "));
		let hash = crate::hex::decode(&hash[5..]);

		assert!(lines.next().is_none());

		let mut xoodyak = XoodyakHash::<Lsbu32<12>>::new();
		xoodyak.absorb(&msg);

		assert_eq!(&xoodyak.squeeze::<32>()[..], hash);
	}
}

#[test]
fn xoodoo_keyed() {
	for (i, test) in include_str!("LWC_AEAD_KAT_128_128.txt").split_terminator("\n\n").enumerate() {
		let mut lines = test.split('\n');

		let count = lines.next().unwrap();
		assert!(count.starts_with("Count = ") && i + 1 == count[8..].parse().unwrap());

		let key = lines.next().unwrap();
		assert!(key.starts_with("Key = "));
		let key = crate::hex::decode(&key[6..]);

		let nonce = lines.next().unwrap();
		assert!(nonce.starts_with("Nonce = "));
		let nonce = crate::hex::decode(&nonce[8..]);

		let data = lines.next().unwrap();
		assert!(data.starts_with("PT = "));
		let mut data = crate::hex::decode(&data[5..]);

		let metadata = lines.next().unwrap();
		assert!(metadata.starts_with("AD = "));
		let metadata = crate::hex::decode(&metadata[5..]);

		let tag = lines.next().unwrap();
		assert!(tag.starts_with("CT = "));
		let tag = crate::hex::decode(&tag[5..]);

		assert!(lines.next().is_none());

		let mut xoodyak = XoodyakKeyed::<Lsbu32<12>>::with_key_id(&key, &nonce);
		xoodyak.absorb(&metadata);
		xoodyak.encrypt(&mut data);
		data.extend_from_slice(&xoodyak.squeeze::<16>());

		assert_eq!(data, tag, "Count = {}", i + 1);
	}
}
