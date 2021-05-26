use std::convert::TryInto;

use super::matrix::Matrix;
use super::bipoly::*;
use super::constants::*;

pub fn hash(mut input: Vec<u8>) -> [u8; 16] {
    add_padding(&mut input);
    let hash = input
        .chunks(16)
        .fold(Matrix::zeros(), |acc, element| {
            whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
        });
    to_array(&hash)
}

fn whirlpool(h: Matrix, w: [u8; 16]) -> Matrix {
    let a = Matrix([
        [
            BiPoly(w[0 + 4 * 0]),
            BiPoly(w[1 + 4 * 0]),
            BiPoly(w[2 + 4 * 0]),
            BiPoly(w[3 + 4 * 0]),
        ],
        [
            BiPoly(w[0 + 4 * 1]),
            BiPoly(w[1 + 4 * 1]),
            BiPoly(w[2 + 4 * 1]),
            BiPoly(w[3 + 4 * 1]),
        ],
        [
            BiPoly(w[0 + 4 * 2]),
            BiPoly(w[1 + 4 * 2]),
            BiPoly(w[2 + 4 * 2]),
            BiPoly(w[3 + 4 * 2]),
        ],
        [
            BiPoly(w[0 + 4 * 3]),
            BiPoly(w[1 + 4 * 3]),
            BiPoly(w[2 + 4 * 3]),
            BiPoly(w[3 + 4 * 3]),
        ],
    ]);
    let mut k = h;
    let mut m = a + h;

    for round in 0..6 {
        // SB
        for i in 0..4 {
            for j in 0..4 {
                k.0[i][j] = s(k.0[i][j]);
                m.0[i][j] = s(m.0[i][j]);
            }
        }
        // SC
        let mut k_prim = Matrix::zeros();
        let mut m_prim = Matrix::zeros();
        for i in 0..4 {
            for j in 0..4 {
                k_prim.0[i][j] = k.0[i][(j + i) % 4];
                m_prim.0[i][j] = m.0[i][(j + i) % 4];
            }
        }

        // MR
        k = T * k_prim;
        m = T * m_prim;

        // AR
        for i in 0..4 {
            k.0[0][i] = k.0[0][i].add(&R[round][i]);
        }
        m = m + k;
    }
    m + a + h
}


fn s(a: BiPoly) -> BiPoly {
    // let a_inv = poly_mod_inv(a);
    // if a_inv.is_none() {
    //     panic!("failed to find modulo inverse {:?} mod {:?}", a, modulo)
    // }
    // let a_inv = a_inv.unwrap();
    // TODO this should match
    // if a.0 != 0 {
    //     assert_eq!(a.mul(&a_inv).0, 1);
    // }
    // let mapping = BiPoly(0xB6)
    //     .mulmod(&a_inv)
    //     .add(&BiPoly(0x34))
    //     .div16(modulo)
    //     .1;
    let row = (a.0 >> 4) as usize;
    let col = (a.0 & 0b0000_1111) as usize;
    // println!("S({:x}) = {:x} = <matrix>[{:x}][{:x}] = {:x}", a.0,  mapping.0, row, col, MATRIX[row][col].0);
    return MATRIX[row][col];
}


fn to_array(matrix: &Matrix) -> [u8; 16] {
    let flattened: Vec<u8> = matrix.0.iter().flatten().map(|x| x.0).collect();
    flattened.try_into().expect("Could not map vec to array")
}

fn add_padding(input: &mut Vec<u8>) {
    let payload_size = input.len();
    let blocks = (payload_size + LENGTH_SIZE - 1) / BLOCK_SIZE + 1;
    let bytes = blocks * BLOCK_SIZE;
    input.resize(bytes, 0u8);
    let elem_lower = input.get_mut(bytes - 1).unwrap();
    if payload_size <= (u8::MAX as usize) {
        *elem_lower = payload_size as u8;
    } else {
        let elem_upper = input.get_mut((bytes - 2)..=(bytes - 1)).unwrap();
        elem_upper[0] = (((payload_size as u16) & 0xFF_00) >> 8) as u8;
        elem_upper[1] = ((payload_size as u16) & 0x00_FF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_whirpool() {
        {
            let input = [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ];
            let hash = whirlpool(Matrix::zeros(), input);
            let expected = [
                0xEA, 0xBC, 0x8C, 0x30, 0x17, 0xDC, 0x2D, 0x09, 0x60, 0x9E, 0x2A, 0x27, 0x2B, 0x26,
                0x0B, 0xE1,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let input = [0u8; 16];
            let hash = whirlpool(Matrix::zeros(), input);
            let expected = [
                0xF4, 0xE7, 0xC2, 0x07, 0x92, 0xAF, 0x80, 0x9B, 0x01, 0x84, 0xC6, 0x84, 0x7B, 0xAF,
                0xE8, 0x6A,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "AbCxYz".as_bytes().to_vec();
            println!("input {:?}", input);
            add_padding(&mut input);
            println!("input after padding {:?}", input);
            let hash = input
                .chunks(16)
                .fold(Matrix::zeros(), |acc, element| {
                    whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                });
            let expected = [
                0x67, 0x2F, 0xAE, 0x13, 0xF4, 0x8D, 0xED, 0xA1, 0x99, 0x91, 0x31, 0x9E, 0x06, 0xFF,
                0xC7, 0x88,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "1234567890".as_bytes().to_vec();
            add_padding(&mut input);
            let hash = input
                .chunks(16)
                .fold(Matrix::zeros(), |acc, element| {
                    whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                });
            let expected = [
                0x8E, 0x65, 0x6F, 0xBC, 0xB4, 0xA3, 0xDF, 0xC4, 0xA1, 0x5F, 0x96, 0x90, 0xD2, 0xCC,
                0x12, 0x63,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "Ala ma kota, kot ma ale.".as_bytes().to_vec();
            add_padding(&mut input);
            let hash = input
                .chunks(16)
                .fold(Matrix::zeros(), |acc, element| {
                    whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                });
            let expected = [
                0x83, 0xA9, 0xFB, 0x7E, 0x22, 0x64, 0xAE, 0x75, 0x65, 0x36, 0xB5, 0x1A, 0xA5, 0xDD,
                0x4E, 0x51,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "Ty, ktory wchodzisz, zegnaj sie z nadzieja."
                .as_bytes()
                .to_vec();
            add_padding(&mut input);
            let hash = input
                .chunks(16)
                .fold(Matrix::zeros(), |acc, element| {
                    whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                });
            let expected = [
                0x2B, 0xE5, 0xCC, 0x98, 0xDC, 0xC9, 0x24, 0xC8, 0x66, 0xED, 0xCF, 0xF9, 0xD1, 0x1A,
                0x75, 0xFB,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "Litwo, Ojczyzno moja! ty jestes jak zdrowie;"
                .as_bytes()
                .to_vec();
            add_padding(&mut input);
            let hash = input
                .chunks(16)
                .fold(Matrix::zeros(), |acc, element| {
                    whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                });
            let expected = [
                0xCC, 0xE8, 0x5A, 0x43, 0x1C, 0x3C, 0x2D, 0x8F, 0xC1, 0x02, 0xE4, 0x99, 0x3D, 0xFB,
                0xD3, 0x33,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "a".repeat(48000).as_bytes().to_vec();
            add_padding(&mut input);
            let hash = input.chunks(16).enumerate().fold(
                Matrix::zeros(),
                |acc, (i, element)| {
                    println!("folding {}% {}/{}", 100 * i / (48000 / 16), i, 48000 / 16);
                    whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                },
            );
            let expected = [
                0x4A, 0x07, 0x19, 0x09, 0xC7, 0xA6, 0xBD, 0x41, 0x5B, 0xB8, 0xA2, 0x41, 0x87, 0xB3,
                0x61, 0xEB,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "a".repeat(48479).as_bytes().to_vec();
            add_padding(&mut input);
            let hash = input
                .chunks(16)
                .enumerate()
                .fold(Matrix::zeros(), |acc, (i, element)| {
                    println!("folding {}% {}/{}", 100 * i / (48000 / 16), i, 48000 / 16);
                    whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                });
            let expected = [
                0x7C, 0x93, 0x0B, 0x4F, 0xEE, 0x8D, 0x0A, 0x5F, 0x12, 0xE3, 0x81, 0x74, 0x74, 0x6B,
                0x28, 0xBE,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "a".repeat(48958).as_bytes().to_vec();
            add_padding(&mut input);
            let hash =
                input
                    .chunks(16)
                    .enumerate()
                    .fold(Matrix::zeros(), |acc, (i, element)| {
                        println!("folding {}% {}/{}", 100 * i / (48000 / 16), i, 48000 / 16);
                        whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                    });
            let expected = [
                0x4D, 0xB0, 0x06, 0x4A, 0x8A, 0xE7, 0xE7, 0x8D, 0xDC, 0x0C, 0xC4, 0xD9, 0xD6, 0x91,
                0xEE, 0xAE,
            ];
            assert_eq!(to_array(&hash), expected);
        }
    }

    #[test]
    fn test_padding() {
        {
            let mut input = vec![0, 1, 2, 3, 4, 5];
            add_padding(&mut input);
            assert_eq!(input.len(), 16);
            assert_eq!(input, vec![0, 1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6]);
        }
        {
            let mut input = vec![0, 1, 2, 3, 4, 5, 6];
            add_padding(&mut input);
            assert_eq!(input.len(), 16);
            assert_eq!(input, vec![0, 1, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7]);
        }
        {
            let mut input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
            add_padding(&mut input);
            assert_eq!(input.len(), 16);
            assert_eq!(
                input,
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0, 14]
            );
        }
        {
            let mut input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            add_padding(&mut input);
            assert_eq!(input.len(), 32);
            assert_eq!(
                input,
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0x10
                ]
            );
        }
    }

    #[test]
    fn test_degree() {
        let m = BiPoly(0b00000001);
        assert_eq!(m.degree().unwrap(), 0);
        let m = BiPoly(0b00000011);
        assert_eq!(m.degree().unwrap(), 1);
        let m = BiPoly(0b11000011);
        assert_eq!(m.degree().unwrap(), 7);

        let a = BiPoly16(0b0000_0001_0000_0010);
        assert_eq!(a.degree().unwrap(), 8);

        let a = BiPoly16(0b1000_0001_0000_0010);
        assert_eq!(a.degree().unwrap(), 15);
    }
}
