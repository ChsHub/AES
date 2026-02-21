use crate::AESType::AES256;

pub enum AESType {
    AES128,
    AES192,
    AES256,
}

fn print_matrix(state: [[u8; 4]; 4]) {
    // Print actual column major matrix
    for x in 0..4 {
        for y in 0..4 {
            print!("{} ", state[y][x]);
        }
        println!();
    }
    println!();
}

fn sub_bytes(mut ciphertext: &str) {
    // Look up table to replace bytes
    println!("sub_bytes {:?}", ciphertext)
    // TODO implement
}

fn shift_rows(state: &mut [[u8; 4]; 4]) {
    //Shift 2nd row
    let temp = state[0][1];
    state[0][1] = state[1][1];
    state[1][1] = state[2][1];
    state[2][1] = state[3][1];
    state[3][1] = temp;

    //Shift 3rd row
    let temp = state[0][2];
    state[0][2] = state[2][2];
    state[2][2] = temp;
    let temp = state[1][2];
    state[1][2] = state[3][2];
    state[3][2] = temp;

    //Shift 4th row
    let temp = state[3][3];
    state[3][3] = state[2][3];
    state[2][3] = state[1][3];
    state[1][3] = state[0][3];
    state[0][3] = temp;

    println!("shift_rows");
    print_matrix(*state);
}

fn mix_columns(state: &mut [[u8; 4]; 4]) {
    for y in 0..4 {
        mix_single_column(&mut state[y]);
    }
    println!("mix_columns");
    print_matrix(*state);
}

fn mix_single_column(r: &mut [u8; 4]) {
    let mut a: [u8; 4] = [r[0], r[1], r[2], r[3]]; // Copy input
    let mut h: [u8; 4] = [r[0] >> 7, r[1] >> 7, r[2] >> 7, r[3] >> 7]; // Right shift by 7. Only keep left most bit of input
    h = [
        (r[0] << 1) ^ (h[0] * 27),
        (r[1] << 1) ^ (h[1] * 27),
        (r[2] << 1) ^ (h[2] * 27),
        (r[3] << 1) ^ (h[3] * 27),
    ];

    // Matrix mult in GF2^8
    r[0] = h[0] ^ a[3] ^ a[2] ^ h[1] ^ a[1];
    r[1] = h[1] ^ a[0] ^ a[3] ^ h[2] ^ a[2];
    r[2] = h[2] ^ a[1] ^ a[0] ^ h[3] ^ a[3];
    r[3] = h[3] ^ a[2] ^ a[1] ^ h[0] ^ a[0];
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
    // Column major order
    let mut state: [[u8; 4]; 4] = [[0, 4, 8, 12], [1, 5, 9, 13], [2, 6, 10, 14], [3, 7, 11, 15]];
    print_matrix(state);

    let round_key = "Test round key";
    add_round_key(plaintext, round_key);

    for i in 1..rounds {
        println!("Round : {}", i);
        let round_key = "Test round key";
        sub_bytes(plaintext);
        shift_rows(&mut state);
        mix_columns(&mut state);
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
        mix_single_column(&mut test_input);
        assert_eq!(test_input, [93, 224, 112, 187]);

        let mut test_input: [u8; 4] = [242, 10, 34, 92];
        mix_single_column(&mut test_input);
        assert_eq!(test_input, [159, 220, 88, 157]);

        let mut test_input: [u8; 4] = [1, 1, 1, 1];
        mix_single_column(&mut test_input);
        assert_eq!(test_input, [1, 1, 1, 1]);

        let mut test_input: [u8; 4] = [212, 212, 212, 213];
        mix_single_column(&mut test_input);
        assert_eq!(test_input, [213, 213, 215, 214]);

        let mut test_input: [u8; 4] = [45, 38, 49, 76];
        mix_single_column(&mut test_input);
        assert_eq!(test_input, [77, 126, 189, 248]);
    }
}
