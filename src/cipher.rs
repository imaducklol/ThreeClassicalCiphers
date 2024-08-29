use std::collections::HashMap;

const letter_to_number: HashMap<char, i32> = HashMap::new();
const number_to_letter: HashMap<i32, char> = HashMap::new();
trait Cipher {
    fn encrypt(&self, key: &str, data: &str);
    fn decrypt(&self, key: &str, data: &str);
}

struct CeaserCipher;
impl Cipher for CeaserCipher {
    fn encrypt(&self, key: &str, data: &str) {
        todo!()
    }
    fn decrypt(&self, key: &str, data: &str) {
        todo!()
    }
}

struct VigenereCipher;
impl Cipher for VigenereCipher {
    fn encrypt(&self, key: &str, data: &str) {
        todo!()
    }
    fn decrypt(&self, key: &str, data: &str) {
        todo!()
    }
}

struct OTPCipher;
impl Cipher for OTPCipher {
    fn encrypt(&self, key: &str, data: &str) {
        todo!()
    }
    fn decrypt(&self, key: &str, data: &str) {
        todo!()
    }
}