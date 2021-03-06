extern crate rustc_serialize;


pub fn fixed_xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    assert_eq!(left.len(), right.len());

    left.iter().zip(right.iter())
        .map(|(l, r) : (&u8, &u8)| *l ^ *r)
        .collect()
}

#[test]
fn challenge2() {
    use self::rustc_serialize::hex::{FromHex, ToHex};
    let xored = fixed_xor("1c0111001f010100061a024b53535009181c".from_hex().unwrap().as_ref(),
                          "686974207468652062756c6c277320657965".from_hex().unwrap().as_ref());
    assert_eq!("746865206b696420646f6e277420706c6179".to_string(), xored.to_hex());
}
