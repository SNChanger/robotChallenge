use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;

type AesCbc = Cbc<Aes256, Pkcs7>;

// ランダム文字列の元
const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

// IV用にランダム文字列生成
fn gen_ascii_chars(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect()
    ).unwrap()
}

// 暗号化
fn encrypt(key: &str, data: &str) -> String {
    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_from_slices(key.as_bytes(), iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

fn main() {
    let plaintext = "予定表 2020/08/06でよろしく";
    println!("原文がこれだよ {plaintext}");
    let zokugunn = "01234567012345670123456701234567";
    let kangunn =  "01234567012345670123456701234566";
    let zokuenc = encrypt(&zokugunn, &plaintext);
    let kanenc = encrypt(&kangunn, &plaintext);

    println!("賊軍宛て変換結果 {}", zokuenc);
    println!("官軍宛て結果 {}", kanenc);
}