use crate::AESType::AES256;

pub enum AESType {
    AES128,
    AES192,
    AES256,
}
fn sub_bytes(mut ciphertext: &str) {
    // Look up table to replace bytes
    println!("sub_bytes {:?}", ciphertext)
    // TODO implement
}
fn shift_rows(state: &mut [[i8; 4]; 4]) {
    println!("shift_rows {:?}", state);
    let temp = state[1][0];
    state[1][0] = state[1][1];
    state[1][1] = state[1][2];
    state[1][2] = state[1][3];
    state[1][3] = temp;

    let temp = state[2][0];
    state[2][0] = state[2][2];
    state[2][2] = temp;
    let temp = state[2][1];
    state[2][1] = state[2][3];
    state[2][3] = temp;

    let temp = state[3][3];
    state[3][3] = state[3][2];
    state[3][2] = state[3][1];
    state[3][1] = state[3][0];
    state[3][0] = temp;
    println!("shift_rows {:?}", state);
}
fn mix_columns(mut ciphertext: &str) {
    println!("mix_columns {:?}", ciphertext);
    // TODO implement
}
fn add_round_key(mut ciphertext: &str, mut round_key: &str) {
    // Use bitwise xor on the ciphertext
    println!("add_round_key {:?}", ciphertext);
    // TODO implement
}

fn decrypt(){
    // TODO implement
}

fn encrypt(plaintext: &mut &str, aes_type: &AESType) -> String{
    let rounds: i8 = match aes_type {
        AESType::AES128 => 10,
        AESType::AES192 => 12,
        AESType::AES256 => 14
    };

    println!("{}", plaintext.to_string());
    let mut state: [[i8; 4]; 4] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
    let key_block: [[i8; 4]; 4] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
    // println!("{:#?}", state[1][1]);

    let round_key = "Test round key";
    add_round_key(plaintext, round_key);

    for i in 1..rounds {
        let round_key = "Test round key";
        println!("{}", i);
        sub_bytes(plaintext);
        shift_rows(&mut state);
        mix_columns(plaintext);
        add_round_key(plaintext, round_key);
    }
    let round_key = "Test round key";
    sub_bytes(plaintext);
    shift_rows(&mut state);
    add_round_key(plaintext, round_key);

    println!("{}", plaintext.to_string());
    return plaintext.to_string()
}

fn main() {
    // https://en.wikipedia.org/wiki/Advanced_Encryption_Standard
    let mut plaintext = "Hello World!"; // Some example inputs for testing
    let aes_type = AES256;

    let ciphertext = encrypt(&mut plaintext, &aes_type);
}