use crate::pkcs7_pad;
use aes::cipher::generic_array::typenum::Unsigned;
use aes::{Aes128, BlockCipher, BlockDecrypt, BlockEncrypt, NewBlockCipher};

fn xor_in_place(a: &mut [u8], b: &[u8]) {
    assert_eq!(a.len(), b.len());
    a.iter_mut().zip(b.iter()).for_each(|(l, r)| *l = *l ^ *r);
}

const BLOCK_LEN: usize = <Aes128 as BlockCipher>::BlockSize::USIZE;

// apply padding if it's the wrong length, I guess?
pub fn encrypt(key: &[u8; 16], iv: &[u8; 16], msg: &[u8]) -> Vec<u8> {
    if msg.len() == 0 {
        return Vec::new();
    }

    let cipher = Aes128::new(key.into());

    let mut result = msg.to_vec();
    if result.len() % BLOCK_LEN != 0 {
        pkcs7_pad(&mut result, BLOCK_LEN);
    }
    let mut chunks = result.chunks_exact_mut(BLOCK_LEN);
    let mut last = chunks.next().unwrap();
    xor_in_place(last, iv);
    cipher.encrypt_block(last.into());
    for chunk in chunks {
        xor_in_place(chunk, last);
        cipher.encrypt_block(chunk.into());
        last = chunk;
    }
    result
}

/// does not strip padding
pub fn decrypt(key: &[u8; 16], iv: &[u8; 16], msg: &[u8]) -> Vec<u8> {
    assert_eq!(0, msg.len() % BLOCK_LEN);
    if msg.len() == 0 {
        return Vec::new();
    }

    let cipher = Aes128::new(key.into());

    let prev_blocks = std::iter::once(&iv[..]).chain(msg.chunks_exact(BLOCK_LEN));
    let mut result = msg.to_vec();
    for (chunk, last) in result.chunks_exact_mut(BLOCK_LEN).zip(prev_blocks) {
        cipher.decrypt_block(chunk.into());
        xor_in_place(chunk, last);
    }
    result
}