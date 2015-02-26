extern crate "rustc-serialize" as rustc_serialize;

use std::iter::AdditiveIterator;
use std::num::Int;

use set1::challenge3::single_character_xor;
use set1::challenge5::rotating_key_xor;

pub fn hamming(left: &[u8], right: &[u8]) -> usize {
    assert_eq!(left.len(), right.len());

    left.iter().zip(right.iter())
        .map(|(l, r)| l ^ r)
        .map(|xored| xored.count_ones())
        .sum()
}

pub fn score_keysize(keysize: usize, encrypted_data: &Vec<u8>) -> usize {
    let mut keysize_iterator = encrypted_data.chunks(keysize);
    let average_over_distances = 5;
    let sum_distance : usize = range(0, average_over_distances).map(|_| {
        hamming(&keysize_iterator.next().unwrap(), &keysize_iterator.next().unwrap()) / keysize
    }).sum();
    let avg_distance = (sum_distance*100) / average_over_distances;


    println!("Hamming avg_distance: {} sum_distance: {} keysize: {}, score: {}", avg_distance, sum_distance, keysize, avg_distance);
    avg_distance
}

pub fn transpose_blocks(keysize: usize, encrypted_data: &Vec<u8>) -> Vec<Vec<u8>> {
    let keysize_iterator = encrypted_data.chunks(keysize);
    let mut blocks : Vec<Vec<u8>> = Vec::new();
    let num_blocks = encrypted_data.len() / keysize;
    println!("Num blocks: {}", num_blocks);
    blocks.resize(keysize, Vec::with_capacity(keysize));
    for mut block in keysize_iterator {
        let mut counter = range(0, keysize);
        for ch in block {
            let count = counter.next().unwrap();
            // println!("Count: {}, CH: {}", count, str::from_utf8(ch).unwrap());
            blocks[count].push(ch.clone());
        }
    }
    blocks
}


#[test]
fn challenge6() {
    use self::rustc_serialize::base64::FromBase64;
    use std::old_io::File;
    use std::env::current_dir;

    let contents = File::open(&current_dir().unwrap().join("data").join("6.txt")).read_to_end().unwrap().from_base64().unwrap();
    let mut scored_keysizes: Vec<(usize, usize)> = vec![];
    for keysize in 2..65 {
        scored_keysizes.push((score_keysize(keysize, &contents), keysize));
    }
    scored_keysizes.sort_by(|s1, s2| s1.0.cmp(&s2.0));
    for i in range(0,2) {
        let possible_keysize = scored_keysizes[i];
        println!("[{}] Trying with keysize: {} it scored: {}", i, possible_keysize.1, possible_keysize.0);
        let blocks = transpose_blocks(possible_keysize.1, &contents);
        let repeating_key : Vec<u8> = blocks.iter().map(|block| {
            match single_character_xor(block.as_slice()).pop() {
                Some((score, ch, result)) => {
                    // println!("Success: {}, Block: {}\n Res: {} \n Char: {} \n length: {}", score, str::from_utf8(block).unwrap(), result, ch, block.len());
                    ch
                },
                None => {
                    println!("Fail: length: {}", block.len());
                    0
                }
            }
        }).collect();
        println!("Trying KEY: {}", String::from_utf8_lossy(repeating_key.as_slice()));
        println!("{}", String::from_utf8_lossy(rotating_key_xor(&contents, repeating_key.as_slice()).as_slice()).as_slice());
    }
}

#[test]
fn test_hamming() {
    assert_eq!(hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()), 37);
}

#[test]
fn test_keysize() {
    use self::rustc_serialize::hex::FromHex;
    let test_string = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap();
    let mut scored_keysizes: Vec<(usize, usize)> = vec![];
    for keysize in 2..7 {
        scored_keysizes.push((score_keysize(keysize, &test_string), keysize));
    }
    scored_keysizes.sort_by(|s1, s2| s1.0.cmp(&s2.0));

    assert_eq!(scored_keysizes[0].1, 3);
}
