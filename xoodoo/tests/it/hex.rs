#![allow(unused)]

pub fn decode(s: &str) -> Vec<u8> {
	fn decode_char(c: u8) -> u8 {
		match c {
			b'0'..=b'9' => c - b'0',
			b'a'..=b'f' => c - b'a' + 10,
			b'A'..=b'F' => c - b'A' + 10,
			_ => panic!("not a hex character: {:?}", c as char)
		}
	}

	assert!(s.len() % 2 == 0);
	let s = s.as_bytes();

	let mut vec = Vec::with_capacity(s.len() / 2);

	for i in 0..s.len() / 2 {
		vec.push(decode_char(s[i * 2]) << 4 | decode_char(s[i * 2 + 1]));
	}

	vec
}

pub fn encode(data: &[u8]) -> String {
	const ALPHABET: &[u8] = b"0123456789abcdef";

	let mut vec = Vec::with_capacity(data.len() * 2);

	for b in data {
		vec.push(ALPHABET[(b >> 4) as usize]);
		vec.push(ALPHABET[(b & 0x0F) as usize]);
	}

	String::from_utf8(vec).unwrap()
}
