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
fn shift_rows(state: &mut [[u8; 4]; 4]) {
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
fn mix_columns(column: &mut [u8; 4]) {
    println!("mix_columns {:?}", column);
    // TODO implement
}
fn add_round_key(mut ciphertext: &str, mut round_key: &str) {
    // Use bitwise xor on the ciphertext
    println!("add_round_key {:?}", ciphertext);
    // TODO implement
}

fn decrypt() {
    // TODO implement
}

fn encrypt(plaintext: &mut &str, aes_type: &AESType) -> String {
    let rounds: u8 = match aes_type {
        AESType::AES128 => 10,
        AESType::AES192 => 12,
        AESType::AES256 => 14,
    };

    println!("{}", plaintext.to_string());
    let mut state: [[u8; 4]; 4] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
    let key_block: [[u8; 4]; 4] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
    // println!("{:#?}", state[1][1]);

    let round_key = "Test round key";
    add_round_key(plaintext, round_key);

    for i in 1..rounds {
        let round_key = "Test round key";
        println!("{}", i);
        sub_bytes(plaintext);
        shift_rows(&mut state);
        mix_columns(&mut state[0]); // TODO Fix this workaround
        add_round_key(plaintext, round_key);
    }
    let round_key = "Test round key";
    sub_bytes(plaintext);
    shift_rows(&mut state);
    add_round_key(plaintext, round_key);

    println!("{}", plaintext.to_string());
    return plaintext.to_string();
}

fn main() {
    // https://en.wikipedia.org/wiki/Advanced_Encryption_Standard
    let mut plaintext = "Hello World!"; // Some example inputs for testing
    let aes_type = AES256;

    let ciphertext = encrypt(&mut plaintext, &aes_type);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shift_rows() {
        let mut test_matrix: [[u8; 4]; 4] = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        shift_rows(&mut test_matrix);
        assert_eq!(
            test_matrix,
            [
                [1, 2, 3, 4],
                [6, 7, 8, 5],
                [11, 12, 9, 10],
                [16, 13, 14, 15]
            ]
        );
    }
    #[test]
    fn test_mix_columns() {
        let mut test_input: [u8; 4] = [99, 71, 162, 240];
        mix_columns(&mut test_input);
        assert_eq!(test_input, [93, 224, 112, 187]);
    }
}
