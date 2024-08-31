use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LETTER_TO_NUMBER: Mutex<HashMap<char, i32>> = Mutex::new(HashMap::new());
    static ref NUMBER_TO_LETTER: Mutex<HashMap<i32, char>> = Mutex::new(HashMap::new());
}

trait Cipher {
    fn encrypt(&self, key: &str, data: &str) -> String;
    fn decrypt(&self, key: &str, data: &str) -> String;
    fn letter_to_number(&self, letter: char) -> i32 {
        let letter_to_number = LETTER_TO_NUMBER.lock().unwrap();
        letter_to_number.get(&letter).unwrap().clone()
    }

    fn number_to_letter(&self, number: i32) -> char {
        let number_to_letter = NUMBER_TO_LETTER.lock().unwrap();
        number_to_letter.get(&number).unwrap().clone()
    }
}

struct CaeserCipher;
impl Cipher for CaeserCipher {
    fn encrypt(&self, key: &str, data: &str) -> String{
        // Get offset number from letter value
        let offset: i32 = self.letter_to_number(key.chars().next().unwrap());
        // Vector to store the encrypted stuff
        let mut encrypted_array: Vec<char> = Vec::new();
        // Do the encrypting
        for char in data.chars() {                                              // rem_euclid is circular mod
            let number = (self.letter_to_number(char.to_ascii_uppercase()) + offset).rem_euclid(27);
            encrypted_array.push(self.number_to_letter(number));
        }

        encrypted_array.iter().collect::<String>()
    }
    fn decrypt(&self, key: &str, data: &str) -> String {
        // Get offset number from letter value
        let offset: i32 = self.letter_to_number(key.chars().next().unwrap());
        // Vector to store the decrypted stuff
        let mut decrypted: Vec<char> = Vec::new();
        // Do the decrypting
        for char in data.chars() {                                              // rem_euclid is circular mod
            let number = (self.letter_to_number(char.to_ascii_uppercase()) - offset).rem_euclid(27);
            decrypted.push(self.number_to_letter(number));
        }

        decrypted.iter().collect::<String>()
    }
}

struct VigenereCipher;
impl Cipher for VigenereCipher {
    fn encrypt(&self, key: &str, data: &str) -> String {
        // Vectors to store the keys and encrypted stuff
        let mut offsets: Vec<i32> = Vec::new();
        let mut encrypted_array: Vec<char> = Vec::new();
        // Get offset values
        for char in key.chars() {
            let number = self.letter_to_number(char.to_ascii_uppercase());
            offsets.push(number);
        }
        // Do the encrypting
        for (i, char) in data.chars().enumerate() {                         // rem_euclid is circular mod
            let number = (self.letter_to_number(char.to_ascii_uppercase()) + offsets[i % offsets.len()]).rem_euclid(27);
            encrypted_array.push(self.number_to_letter(number));
        }

        encrypted_array.iter().collect::<String>()
    }
    fn decrypt(&self, key: &str, data: &str) -> String{
        // Vectors to store the keys and encrypted stuff
        let mut offsets: Vec<i32> = Vec::new();
        let mut decrypted_array: Vec<char> = Vec::new();
        // Get offset values
        for char in key.chars() {
            let number = self.letter_to_number(char.to_ascii_uppercase());
            offsets.push(number);
        }
        // Do the encrypting
        for (i, char) in data.chars().enumerate() {                         // rem_euclid is circular mod
            let number = (self.letter_to_number(char.to_ascii_uppercase()) - offsets[i % offsets.len()]).rem_euclid(27);
            decrypted_array.push(self.number_to_letter(number));
        }

        decrypted_array.iter().collect::<String>()
    }
}

struct OTPCipher;
impl Cipher for OTPCipher {
    fn encrypt(&self, key: &str, data: &str) -> String {
        if (key.len() < data.len()) {
            panic!("One Time Pad Cipher Requires a key larger than or equal to the data to be encrypted")
        }
        // Vectors to store the keys and encrypted stuff
        let mut offsets: Vec<i32> = Vec::new();
        let mut encrypted_array: Vec<char> = Vec::new();
        // Get offset values
        for char in key.chars() {
            let number = self.letter_to_number(char.to_ascii_uppercase());
            offsets.push(number);
        }
        // Do the encrypting
        for (i, char) in data.chars().enumerate() {                         // rem_euclid is circular mod
            let number = (self.letter_to_number(char.to_ascii_uppercase()) + offsets[i]).rem_euclid(27);
            encrypted_array.push(self.number_to_letter(number));
        }

        encrypted_array.iter().collect::<String>()
    }
    fn decrypt(&self, key: &str, data: &str) -> String{
        if (key.len() < data.len()) {
            panic!("One Time Pad Cipher Requires a key larger than or equal to the data to be decrypted")
        }
        // Vectors to store the keys and encrypted stuff
        let mut offsets: Vec<i32> = Vec::new();
        let mut decrypted_array: Vec<char> = Vec::new();
        // Get offset values
        for char in key.chars() {
            let number = self.letter_to_number(char.to_ascii_uppercase());
            offsets.push(number);
        }
        // Do the encrypting
        for (i, char) in data.chars().enumerate() {                         // rem_euclid is circular mod
            let number = (self.letter_to_number(char.to_ascii_uppercase()) - offsets[i]).rem_euclid(27);
            decrypted_array.push(self.number_to_letter(number));
        }

        decrypted_array.iter().collect::<String>()
    }
}

fn main() {
    // Initialize the hashmaps for translation between letter and number
    let mut letter_to_number = LETTER_TO_NUMBER.lock().unwrap();
    let mut number_to_letter = NUMBER_TO_LETTER.lock().unwrap();
    for (i, letter) in ('A'..='Z').enumerate() {
        letter_to_number.insert(letter, i as i32);
        number_to_letter.insert(i as i32, letter);
    }
    // Don't forget space
    letter_to_number.insert(' ', 26);
    number_to_letter.insert(26, ' ');
    // Release the mtx lock
    drop(letter_to_number);
    drop(number_to_letter);


    // Initialize ciphers
    let caeser_cipher = CaeserCipher;
    let vigenere_cipher = VigenereCipher;
    let otp_cipher = OTPCipher;

    assert_eq!(
        "Bikini Bottom".to_ascii_uppercase(),
        caeser_cipher.decrypt("D", &caeser_cipher.encrypt("D", "Bikini Bottom"))
    );
    assert_eq!(
        "Bikini Bottom".to_ascii_uppercase(),
        vigenere_cipher.decrypt("DINGUS", &vigenere_cipher.encrypt("DINGUS", "Bikini Bottom")));
    assert_eq!(
        "Bikini Bottom".to_ascii_uppercase(),
        otp_cipher.decrypt("YOU FEEL AN EVIL PRESENCE WATCHING YOU", &otp_cipher.encrypt("YOU FEEL AN EVIL PRESENCE WATCHING YOU", "Bikini Bottom")));

}
