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

fn sub_bytes(state: &mut [[u8; 4]; 4]) {
    // Look up table to replace bytes
    let s_box: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];
    let mut i: usize;

    for x in 0..4 {
        for y in 0..4 {
            i = state[y][x] as usize;
            state[y][x] = s_box[i];
        }
    }

    println!("sub_bytes");
    print_matrix(*state);
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
    let a: [u8; 4] = [r[0], r[1], r[2], r[3]]; // Copy input
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
fn add_round_key(state: &mut [[u8; 4]; 4], round_key: &[[u8; 4]; 4]) {
    // Use bitwise xor on the ciphertext
    for x in 0..4 {
        for y in 0..4 {
            state[y][x] ^= round_key[y][x];
        }
    }

    println!("add_round_key ");
    print_matrix(*state)
}

fn get_round_key(cipher_key: &[u8], round: u8) -> [[u8; 4]; 4] {
    // Each round key is block length = 128bit
    let key_len: usize = 16;
    let start: usize = round as usize * key_len;
    let mut result: [[u8; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

    if start < cipher_key.len() {
        // Copy key as is
        for i in start..cipher_key.len() {
            result[i / 4][i % 4] = cipher_key[i];
        }
    }
    // Expand Key for remaining bytes
    // A different expansion scheme is used for cipher_key.len <= 6
    for i in (cipher_key.len() - start)..key_len {
        result[i / 4][i % 4] = 0; // TODO implement
    }

    result
}

fn decrypt() {
    // TODO implement
}

fn encrypt_state(mut state: &mut [[u8; 4]; 4], rounds: u8, cipher_key: &&[u8]) {
    print_matrix(*state);

    let round_key: [[u8; 4]; 4] = get_round_key(cipher_key, 0);
    add_round_key(&mut state, &round_key);

    for i in 1..rounds {
        println!("Round : {}", i);
        let round_key: [[u8; 4]; 4] = get_round_key(&cipher_key, i);
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &round_key);
    }

    let round_key: [[u8; 4]; 4] = get_round_key(&cipher_key, rounds);
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &round_key);
}

fn encrypt(plaintext: &str, cipher_key: &str, aes_type: AESType, ciphertext: &[u8]){
    println!("{}", plaintext);
    let rounds: u8 = match aes_type {
        AESType::AES128 => 10,
        AESType::AES192 => 12,
        AESType::AES256 => 14,
    };
    let cipher_key = cipher_key.as_bytes(); // Convert to bytes
    let plaintext = plaintext.as_bytes();

    // Encrypt 16 bytes at a time
    for i in 0..plaintext.len() / 16 {
        // Column major order (128bit block length)

        let mut state: [[u8; 4]; 4] = [
            [
                plaintext[i + 0],
                plaintext[i + 4],
                plaintext[i + 8],
                plaintext[i + 12],
            ],
            [
                plaintext[i + 1],
                plaintext[i + 5],
                plaintext[i + 9],
                plaintext[i + 13],
            ],
            [
                plaintext[i + 2],
                plaintext[i + 6],
                plaintext[i + 10],
                plaintext[i + 14],
            ],
            [
                plaintext[i + 3],
                plaintext[i + 7],
                plaintext[i + 11],
                plaintext[i + 15],
            ],
        ];
        encrypt_state(&mut state, rounds, &cipher_key);
    }

    // println!("{}", plaintext.to_string());
}

fn main() {
    // https://en.wikipedia.org/wiki/Advanced_Encryption_Standard
    let plaintext = "Hello World!"; // Some example inputs for testing
    let cipher_key = "Secret KEY!"; // Some example inputs for testing
    let mut aes_type = AES256;
    let mut ciphertext: &[u8] = plaintext.as_bytes();

    let ciphertext = encrypt(&plaintext, &cipher_key, aes_type, ciphertext);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shift_rows() {
        let mut test_matrix: [[u8; 4]; 4] = [
            [1, 5, 9, 13],
            [2, 6, 10, 14],
            [3, 7, 11, 15],
            [4, 8, 12, 16],
        ];
        shift_rows(&mut test_matrix);
        assert_eq!(
            test_matrix,
            [
                [1, 6, 11, 16],
                [2, 7, 12, 13],
                [3, 8, 9, 14],
                [4, 5, 10, 15],
            ]
        );
    }
    #[test]
    fn test_mix_single_column() {
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
    #[test]
    fn test_mix_columns() {
        let mut test_input: [[u8; 4]; 4] = [
            [45, 38, 49, 76],
            [1, 1, 1, 1],
            [45, 38, 49, 76],
            [212, 212, 212, 213],
        ];
        mix_columns(&mut test_input);
        assert_eq!(
            test_input,
            [
                [77, 126, 189, 248],
                [1, 1, 1, 1],
                [77, 126, 189, 248],
                [213, 213, 215, 214]
            ]
        );
    }

    #[test]
    fn test_sub_bytes() {
        let mut test_input: [[u8; 4]; 4] = [
            [1, 5, 9,  33],
            [2, 6, 10, 34],
            [3, 7, 11, 35],
            [4, 8, 12, 36],
        ];
        sub_bytes(&mut test_input);
        assert_eq!(
            test_input,
            [
                [0x7C, 0x6B, 0x01, 0xFD],
                [0x77, 0x6F, 0x67, 0x93],
                [0x7B, 0xC5, 0x2B, 0x26],
                [0xF2, 0x30, 0xFE, 0x36]
            ]
        );
    }

    #[test]
    fn test_add_round_key() {
        let mut test_input: [[u8; 4]; 4] = [
            [1, 5, 9, 13],
            [2, 6, 10, 14],
            [3, 7, 11, 15],
            [4, 8, 12, 16],
        ];
        let test_key: [[u8; 4]; 4] = test_input.clone();
        add_round_key(&mut test_input, &test_key);
        assert_eq!(
            test_input,
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        );
    }
}
