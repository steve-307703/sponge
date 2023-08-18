use keccak::{keccakp::*, sha3::*};
use sponge::{state::*, suffix, Sponge};

macro_rules! kat {
	($name:ident, $file:expr, $md:expr, $sponge_new:expr $(, $padding:expr)?) => {
		#[test]
		fn $name() {
			const KAT: &str = include_str!($file);

			let mut iter = KAT.lines().skip(1).filter(|s| !s.is_empty()).peekable();

			while iter.peek().is_some() {
				let len = iter.next().unwrap();
				assert!(len.starts_with("Len = "));
				let len: u32 = len[6..].parse().unwrap();

				let msg = iter.next().unwrap();
				assert!(msg.starts_with("Msg = "));
				let msg = super::hex::decode(&msg[6..]);

				let md = iter.next().unwrap();
				assert!(md.starts_with($md));
				let md = super::hex::decode(&md[$md.len()..]);

				if len % 8 != 0 {
					continue;
				}

				let mut sponge = $sponge_new;

				if len != 0 {
					sponge.absorb(&*msg);
				}

				let mut hash = vec![0; md.len()];
				sponge.squeeze_into($($padding, )? &mut hash);

				assert_eq!(hash, md);
			}
		}
	};
}

kat!(kat_sha3_224, "ShortMsgKAT_SHA3-224.txt", "MD = ", Sha3_224::new());
kat!(kat_sha3_256, "ShortMsgKAT_SHA3-256.txt", "MD = ", Sha3_256::new());
kat!(kat_sha3_384, "ShortMsgKAT_SHA3-384.txt", "MD = ", Sha3_384::new());
kat!(kat_sha3_512, "ShortMsgKAT_SHA3-512.txt", "MD = ", Sha3_512::new());
kat!(kat_shake_128, "ShortMsgKAT_SHAKE128.txt", "Squeezed = ", Shake128::new());
kat!(kat_shake_256, "ShortMsgKAT_SHAKE256.txt", "Squeezed = ", Shake256::new());

kat!(
	kat_keccak800_640,
	"ShortMsgKAT_Keccakr640c160.txt",
	"Squeezed = ",
	Sponge::<Lsbu32<25>, Keccak800<22>, { 160 / 8 }, false>::default(),
	suffix!()
);

kat!(
	kat_keccak800_544,
	"ShortMsgKAT_Keccakr544c256.txt",
	"Squeezed = ",
	Sponge::<Lsbu32<25>, Keccak800<22>, { 256 / 8 }, false>::default(),
	suffix!()
);

kat!(
	kat_keccak800_288,
	"ShortMsgKAT_Keccakr288c512.txt",
	"Squeezed = ",
	Sponge::<Lsbu32<25>, Keccak800<22>, { 512 / 8 }, false>::default(),
	suffix!()
);

kat!(
	kat_keccak400_240,
	"ShortMsgKAT_Keccakr240c160.txt",
	"Squeezed = ",
	Sponge::<Lsbu16<25>, Keccak400<20>, { 160 / 8 }, false>::default(),
	suffix!()
);

kat!(
	kat_keccak400_144,
	"ShortMsgKAT_Keccakr144c256.txt",
	"Squeezed = ",
	Sponge::<Lsbu16<25>, Keccak400<20>, { 256 / 8 }, false>::default(),
	suffix!()
);

kat!(
	kat_keccak400_40,
	"ShortMsgKAT_Keccakr40c160.txt",
	"Squeezed = ",
	Sponge::<Lsbu8<25>, Keccak200<18>, { 160 / 8 }, false>::default(),
	suffix!()
);
