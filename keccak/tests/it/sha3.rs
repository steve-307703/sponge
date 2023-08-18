macro_rules! sha3_test {
	($(#[$attr:meta])? $name:ident: $input:literal $($repeat:expr)? => [$a:literal, $b:literal, $c:literal, $d:literal, $e:literal, $f:literal]) => {
		mod $name {
			use keccak::sha3::*;

			#[test]
			$(#[$attr])?
			fn sha3_224() {
				test!(Sha3_224, $input, $a);
			}

			#[test]
			$(#[$attr])?
			fn sha3_256() {
				test!(Sha3_256, $input, $b);
			}

			#[test]
			$(#[$attr])?
			fn sha3_384() {
				test!(Sha3_384, $input, $c);
			}

			#[test]
			$(#[$attr])?
			fn sha3_512() {
				test!(Sha3_512, $input, $d);
			}

			#[test]
			$(#[$attr])?
			fn shake_128() {
				test!(Shake128, $input, $e);
			}

			#[test]
			$(#[$attr])?
			fn shake_256() {
				test!(Shake256, $input, $f);
			}
		}
	};
}

macro_rules! test {
	($sha:ident, $input:literal $($repeat:expr)?, $expected:literal) => {
		let mut absorber = $sha::new();

		let repeat = 1;
		$(let repeat = $repeat;)?

		for _ in 0..repeat {
			absorber.absorb($input);
		}

		let mut hash = vec![0; $expected.len() / 2];
		absorber.squeeze_into(&mut hash);

		assert_eq!(crate::hex::encode(&hash), $expected);
	}
}

sha3_test!(_1: b"" => [
	"6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7",
	"a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a",
	"0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004",
	"a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26",
	"7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26",
	"46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be"
]);

sha3_test!(_2: b"abc" => [
	"e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf",
	"3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
	"ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25",
	"b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0",
	"5881092dd818bf5cf8a3ddb793fbcba74097d5c526a6d35f97b83351940f2cc8",
	"483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739d5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4"
]);

sha3_test!(_3: b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq" => [
	"8a24108b154ada21c9fd5574494479ba5c7e7ab76ef264ead0fcce33",
	"41c0dba2a9d6240849100376a8235e2c82e1b9998a999e21db32dd97496d3376",
	"991c665755eb3a4b6bbdfb75c78a492e8c56a22c5c4d7e429bfdbc32b9d4ad5aa04a1f076e62fea19eef51acd0657c22",
	"04a371e84ecfb5b8b77cb48610fca8182dd457ce6f326a0fd3d7ec2f1e91636dee691fbe0c985302ba1b0d8dc78c086346b533b49c030d99a27daf1139d6e75e",
	"1a96182b50fb8c7e74e0a707788f55e98209b8d91fade8f32f8dd5cff7bf21f5",
	"4d8c2dd2435a0128eefbb8c36f6f87133a7911e18d979ee1ae6be5d4fd2e332940d8688a4e6a59aa8060f1f9bc996c05aca3c696a8b66279dc672c740bb224ec"
]);

sha3_test!(_4: b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu" => [
	"543e6868e1666c1a643630df77367ae5a62a85070a51c14cbf665cbc",
	"916f6061fe879741ca6469b43971dfdb28b1a32dc36cb3254e812be27aad1d18",
	"79407d3b5916b59c3e30b09822974791c313fb9ecc849e406f23592d04f625dc8c709b98b43b3852b337216179aa7fc7",
	"afebb2ef542e6579c50cad06d2e578f9f8dd6881d7dc824d26360feebf18a4fa73e3261122948efcfd492e74e82e2189ed0fb440d187f382270cb455f21dd185",
	"7b6df6ff181173b6d7898d7ff63fb07b7c237daf471a5ae5602adbccef9ccf4b",
	"98be04516c04cc73593fef3ed0352ea9f6443942d6950e29a372a681c3deaf4535423709b02843948684e029010badcc0acd8303fc85fdad3eabf4f78cae1656"
]);

// sha3_test!(_5: Cycle { buf: &[b'a'; 200], count: 1_000_000 } => [
// 	"d69335b93325192e516a912e6d19a15cb51c6ed5c15243e7a7fd653c",
// 	"5c8875ae474a3634ba4fd55ec85bffd661f32aca75c6d699d0cdcb6c115891c1",
// 	"eee9e24d78c1855337983451df97c8ad9eedf256c6334f8e948d252d5e0e76847aa0774ddb90a842190d2c558b4b8340",
// 	"3c3a876da14034ab60627c077bb98f7e120a2a5370212dffb3385a18d4f38859ed311d0a9d5141ce9cc5c66ee689b266a8aa18ace8282a0e0db596c90b0a7b87",
// 	"9d222c79c4ff9d092cf6ca86143aa411e369973808ef97093255826c5572ef58",
// 	"3578a7a4ca9137569cdf76ed617d31bb994fca9c1bbf8b184013de8234dfd13a3fd124d4df76c0a539ee7dd2f6e1ec346124c815d9410e145eb561bcd97b18ab"
// ]);

// sha3_test!(#[ignore] _6: Cycle { buf:
// b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno", count: 16_777_216 * 64 } =>
// [ 	"c6d66e77ae289566afb2ce39277752d6da2a3c46010f1e0a0970ff60",
// 	"ecbbc42cbf296603acb2c6bc0410ef4378bafb24b710357f12df607758b33e2b",
// 	"a04296f4fcaae14871bb5ad33e28dcf69238b04204d9941b8782e816d014bcb7540e4af54f30d578f1a1ca2930847a12",
// 	"235ffd53504ef836a1342b488f483b396eabbfe642cf78ee0d31feec788b23d0d18d5c339550dd5958a500d4b95363da1b5fa18affc1bab2292dc63b7d85097c",
// 	"f4e546891fa8bacea5a159301feebaa4b67c9dd8d8787d82caf3b6bd8c5bc363",
// 	"3c23f2c994061ff3041d7e52089972c2d074d281912cc4c7d8475ab6c816d78730c475532f684820c9306dc22c435b02429d3e7cb2667c141ac03da4e7b11884"
// ]);

sha3_test!(_7: b"The quick brown fox jumps over the lazy dog" => [
	"d15dadceaa4d5d7bb3b48f446421d542e08ad8887305e28d58335795",
	"69070dda01975c8c120c3aada1b282394e7f032fa9cf32f4cb2259a0897dfc04",
	"7063465e08a93bce31cd89d2e3ca8f602498696e253592ed26f07bf7e703cf328581e1471a7ba7ab119b1a9ebdf8be41",
	"01dedd5de4ef14642445ba5f5b97c15e47b9ad931326e4b0727cd94cefc44fff23f07bf543139939b49128caf436dc1bdee54fcb24023a08d9403f9b4bf0d450",
	"f4202e3c5852f9182a0430fd8144f0a74b95e7417ecae17db0f8cfeed0e3e66e",
	"2f671343d9b2e1604dc9dcf0753e5fe15c7c64a0d283cbbf722d411a0e36f6ca1d01d1369a23539cd80f7c054b6e5daf9c962cad5b8ed5bd11998b40d5734442"
]);

sha3_test!(_8: b"The quick brown fox jumps over the lazy dof" => [
	"d755800c33a9a61f20d94eabaefe35f79cfe45719f6cac4afec884bc",
	"89b8e9fa3de9b8fc846032270d236d642abd7520547668127f934c11529874dd",
	"764fcf078e40e6b21d9873df7717a077daba89cd0e5e7960ec4dacae46be899c8e2a507554843b20b2cbea9ee30b6e97",
	"9c1121393dfb88ae4d81c3ae22e125bdc863e94c710977bdf999b18299cc81a38a6cab805630057c4c4e7720d9b75ca3d51fe2c8c0952529e062fff76ba69328",
	"853f4538be0db9621a6cea659a06c1107b1f83f02b13d18297bd39d7411cf10c",
	"46b1ebb2e142c38b9ac9081bef72877fe4723959640fa57119b366ce6899d4013af024f4222921320bee7d3bfaba07a758cd0fde5d27bbd2f8d709f4307d2c34"
]);
