// https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KangarooTwelve.txt

use keccak::k12::KangarooTwelve;

fn pattern(count: usize) -> Vec<u8> {
	(0x00..=0xFA).cycle().take(count).collect()
}

fn hash(customization: &[u8], input: &[u8], skip: usize, output: &mut [u8]) {
	let mut absorber = KangarooTwelve::new();
	absorber.absorb(input);

	let mut squeezer = if customization.is_empty() {
		absorber.into_squeezer()
	} else {
		absorber.into_squeezer_customized(customization)
	};

	if skip != 0 {
		let mut skip = vec![0; skip];
		squeezer.squeeze_into(&mut skip);
	}

	squeezer.squeeze_into(output);
}

#[track_caller]
fn test(customization: usize, input: &[u8], skip: usize, expected: &str) {
	let mut output = vec![0; expected.len() / 2];
	hash(&pattern(customization), input, skip, &mut output);
	assert_eq!(super::hex::encode(&output), expected);
}

#[test]
fn k12_ref() {
	let pattern = pattern(16 << 10);
	let mut k12 = KangarooTwelve::new();
	let mut output = [0; 32];

	for m in 0..=16 << 10 {
		for c in [0, 32, 4 << 10, 16 << 10] {
			hash(&pattern[..c], &pattern[..m], 0, &mut output);
			k12.absorb(&output);
		}
	}

	assert_eq!(
		super::hex::encode(&k12.squeeze::<32>()),
		"eafcde8bfa24e0d4ab8e3598ed1dec739d7dffc5a1f0157cb6fd9ad939f84ade"
	);
}

#[test]
fn k12_01() {
	test(0, &[], 0, "1ac2d450fc3b4205d19da7bfca1b37513c0803577ac7167f06fe2ce1f0ef39e5");
}

#[test]
fn k12_02() {
	test(
		0,
		&[],
		0,
		"1ac2d450fc3b4205d19da7bfca1b37513c0803577ac7167f06fe2ce1f0ef39e54269c056b8c82e48276038b6d292966cc07a3d4645272e31ff38508139eb0a71"
	);
}

#[test]
fn k12_03() {
	test(0, &[], 10_000, "e8dc563642f7228c84684c898405d3a834799158c079b12880277a1d28e2ff6d");
}

#[test]
fn k12_04() {
	test(0, &pattern(1), 0, "2bda92450e8b147f8a7cb629e784a058efca7cf7d8218e02d345dfaa65244a1f");
}

#[test]
fn k12_05() {
	test(0, &pattern(17), 0, "6bf75fa2239198db4772e36478f8e19b0f371205f6a9a93a273f51df37122888");
}

#[test]
fn k12_06() {
	test(
		0,
		&pattern(17_usize.pow(2)),
		0,
		"0c315ebcdedbf61426de7dcf8fb725d1e74675d7f5327a5067f367b108ecb67c"
	);
}

#[test]
fn k12_07() {
	test(
		0,
		&pattern(17_usize.pow(3)),
		0,
		"cb552e2ec77d9910701d578b457ddf772c12e322e4ee7fe417f92c758f0d59d0"
	);
}

#[test]
fn k12_08() {
	test(
		0,
		&pattern(17_usize.pow(4)),
		0,
		"8701045e22205345ff4dda05555cbb5c3af1a771c2b89baef37db43d9998b9fe"
	);
}

#[test]
fn k12_09() {
	test(
		0,
		&pattern(17_usize.pow(5)),
		0,
		"844d610933b1b9963cbdeb5ae3b6b05cc7cbd67ceedf883eb678a0a8e0371682"
	);
}

#[test]
fn k12_10() {
	test(
		0,
		&pattern(17_usize.pow(6)),
		0,
		"3c390782a8a4e89fa6367f72feaaf13255c8d95878481d3cd8ce85f58e880af8"
	);
}

#[test]
fn k12_11() {
	test(1, &pattern(0), 0, "fab658db63e94a246188bf7af69a133045f46ee984c56e3c3328caaf1aa1a583");
}

#[test]
fn k12_12() {
	test(41, &[0xFF], 0, "d848c5068ced736f4462159b9867fd4c20b808acc3d5bc48e0b06ba0a3762ec4");
}

#[test]
fn k12_13() {
	test(
		41_usize.pow(2),
		&[0xFF; 3],
		0,
		"c389e5009ae57120854c2e8c64670ac01358cf4c1baf89447a724234dc7ced74"
	);
}

#[test]
fn k12_14() {
	test(
		41_usize.pow(3),
		&[0xFF; 7],
		0,
		"75d2f86a2e644566726b4fbcfc5657b9dbcf070c7b0dca06450ab291d7443bcf"
	);
}
