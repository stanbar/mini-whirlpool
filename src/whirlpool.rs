use std::convert::TryFrom;
use std::convert::TryInto;

type Matrix = [[BiPoly; 4]; 4];

const T: Matrix = [
    [BiPoly(1), BiPoly(5), BiPoly(3), BiPoly(2)],
    [BiPoly(2), BiPoly(1), BiPoly(5), BiPoly(3)],
    [BiPoly(3), BiPoly(2), BiPoly(1), BiPoly(5)],
    [BiPoly(5), BiPoly(3), BiPoly(2), BiPoly(1)],
];
const r: [[BiPoly; 4]; 6] = [
    [BiPoly(0x05), BiPoly(0x8C), BiPoly(0xB5), BiPoly(0x60)],
    [BiPoly(0x31), BiPoly(0x60), BiPoly(0xB2), BiPoly(0xA3)],
    [BiPoly(0x42), BiPoly(0xDA), BiPoly(0x56), BiPoly(0x1B)],
    [BiPoly(0x95), BiPoly(0x6D), BiPoly(0x09), BiPoly(0x7F)],
    [BiPoly(0x20), BiPoly(0x40), BiPoly(0x5B), BiPoly(0xCB)],
    [BiPoly(0xAB), BiPoly(0x60), BiPoly(0xB8), BiPoly(0xDB)],
];

fn print_matrix(matrix: Matrix) {
    println!(
        "{}",
        matrix
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| format!("{:x}", y.0))
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}
pub fn hash(mut input: Vec<u8>) -> [u8; 16] {
    add_padding(&mut input);
    let hash = input.chunks(16).fold([[BiPoly(0); 4]; 4], |acc, element| {
        whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
    });
    return to_array(&hash);
}

fn whirlpool(h: Matrix, w: [u8; 16]) -> Matrix {
    let a: [[BiPoly; 4]; 4] = [
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
    ];
    let mut k = h;
    let mut m = add_matrices(a, h);

    for round in 0..6 {
        // SB
        for i in 0..4 {
            for j in 0..4 {
                k[i][j] = S(k[i][j]);
                m[i][j] = S(m[i][j]);
            }
        }
        // SC
        let mut k_prim: Matrix = [[BiPoly(0); 4]; 4];
        let mut m_prim: Matrix = [[BiPoly(0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                k_prim[i][j] = k[i][(j + i) % 4];
                m_prim[i][j] = m[i][(j + i) % 4];
            }
        }

        // MR
        k = mul_matrices(T, k_prim);
        m = mul_matrices(T, m_prim);

        // AR
        for i in 0..4 {
            k[0][i] = k[0][i].add(&r[round][i]);
        }
        m = add_matrices(m, k);
    }
    add_matrices(add_matrices(m, a), h)
}

fn mul_matrices(a: Matrix, b: Matrix) -> Matrix {
    let mut out: Matrix = [[BiPoly(0); 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                out[i][j] = a[i][k].mulmod(&b[k][j]).add(&out[i][j]);
            }
        }
    }

    out
}
fn add_matrices(a: Matrix, b: Matrix) -> Matrix {
    let mut c: Matrix = [[BiPoly(0); 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            c[row][col] = a[row][col].add(&b[row][col])
        }
    }
    c
}

/// Binary Ring Polynomial element
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct BiPoly(u8);

impl std::fmt::Display for BiPoly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct BiPoly16(u16);

impl std::fmt::Display for BiPoly16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BiPoly {
    fn to_array(&self) -> [bool; 8] {
        return [
            self.0 >> 0 & 1 == 1,
            self.0 >> 1 & 1 == 1,
            self.0 >> 2 & 1 == 1,
            self.0 >> 3 & 1 == 1,
            self.0 >> 4 & 1 == 1,
            self.0 >> 5 & 1 == 1,
            self.0 >> 6 & 1 == 1,
            self.0 >> 7 & 1 == 1,
        ];
    }
    /// Adding two elements in a field Z_2[x] can be done with xoring two numbers
    /// Example.
    /// 1111 + 0011
    /// = (x^3 + x^2 + x + 1 ) + (x^2 + 1)
    /// = x^3 + 2*x^2 + x + 2
    /// Since Z_2 we perform mod 2 on each coefficient
    /// = (1 mod 2)x^3 + (2 mod 2)x^2 + (1 mod 2)x + (2 mod 2)x
    /// = x^3 + 0x^2 + 1x + 0
    /// = x^3 + x
    fn add(&self, other: &BiPoly) -> BiPoly {
        return BiPoly(self.0 ^ other.0);
    }

    /// Substracting two elements is the same as adding them
    fn sub(&self, other: &BiPoly) -> BiPoly {
        return BiPoly(self.0 ^ other.0);
    }
    ///
    /// Substracting two elements is the same as adding them
    fn sub16(&self, other: &BiPoly16) -> BiPoly16 {
        return BiPoly16((self.0 as u16) ^ other.0);
    }

    /// Multyplying two elements of polynomial ring can be done by adding together the
    fn mulmod(&self, other: &BiPoly) -> BiPoly {
        // Split these u8 to array of limbs
        let arr = self.to_array();
        let brr = other.to_array();
        let mut out = [false; 16];

        for i in 0..8 {
            for j in 0..8 {
                // & is a multiplication mod 2 and ^ is addition mod 2
                out[i + j] = (arr[i] & brr[j]) ^ out[i + j];
            }
        }

        BiPoly16(
            (u16::from(out[0]) << 0)
                | (u16::from(out[1]) << 1)
                | (u16::from(out[2]) << 2)
                | (u16::from(out[3]) << 3)
                | (u16::from(out[4]) << 4)
                | (u16::from(out[5]) << 5)
                | (u16::from(out[6]) << 6)
                | (u16::from(out[7]) << 7)
                | (u16::from(out[8]) << 8)
                | (u16::from(out[9]) << 9)
                | (u16::from(out[10]) << 10)
                | (u16::from(out[11]) << 11)
                | (u16::from(out[12]) << 12)
                | (u16::from(out[13]) << 13)
                | (u16::from(out[14]) << 14)
                | (u16::from(out[15]) << 15),
        )
        .div16(&modulo)
        .1
    }
    /// Multyplying two elements of polynomial ring can be done by adding together the
    fn mul16(&self, other: &BiPoly16) -> BiPoly16 {
        // Split these u8 to array of limbs
        let arr: [bool; 8] = self.to_array(); // and extend this array to match 16 limbs
        let brr: [bool; 16] = other.to_array();
        let mut out = [false; 16 + 8];

        for i in 0..8 {
            for j in 0..16 {
                //TODO but then extend also the indexes
                // & is a multiplication mod 2 and ^ is addition mod 2
                out[i + j] = (arr[i] & brr[j]) ^ out[i + j];
            }
        }

        return BiPoly16(
            (u16::from(out[0]) << 0)
                | (u16::from(out[1]) << 1)
                | (u16::from(out[2]) << 2)
                | (u16::from(out[3]) << 3)
                | (u16::from(out[4]) << 4)
                | (u16::from(out[5]) << 5)
                | (u16::from(out[6]) << 6)
                | (u16::from(out[7]) << 7)
                | (u16::from(out[8]) << 8)
                | (u16::from(out[9]) << 9)
                | (u16::from(out[10]) << 10)
                | (u16::from(out[11]) << 11)
                | (u16::from(out[12]) << 12)
                | (u16::from(out[13]) << 13)
                | (u16::from(out[14]) << 14)
                | (u16::from(out[15]) << 15),
        );
    }

    fn div(&self, other: BiPoly) -> (BiPoly, BiPoly) {
        return poly_long_division(self.clone(), other).unwrap();
    }

    fn div16(&self, other: BiPoly16) -> (BiPoly16, BiPoly) {
        return poly_long_division16(BiPoly16(self.0 as u16), &other).unwrap();
    }

    fn degree(&self) -> Option<u8> {
        let zeros = self.0.leading_zeros();
        if zeros == 8 {
            None
        } else {
            u8::try_from(7 - zeros).ok()
        }
    }
}

impl BiPoly16 {
    fn to_array(&self) -> [bool; 16] {
        return [
            self.0 >> 0 & 1 == 1,
            self.0 >> 1 & 1 == 1,
            self.0 >> 2 & 1 == 1,
            self.0 >> 3 & 1 == 1,
            self.0 >> 4 & 1 == 1,
            self.0 >> 5 & 1 == 1,
            self.0 >> 6 & 1 == 1,
            self.0 >> 7 & 1 == 1,
            self.0 >> 8 & 1 == 1,
            self.0 >> 9 & 1 == 1,
            self.0 >> 10 & 1 == 1,
            self.0 >> 11 & 1 == 1,
            self.0 >> 12 & 1 == 1,
            self.0 >> 13 & 1 == 1,
            self.0 >> 14 & 1 == 1,
            self.0 >> 15 & 1 == 1,
        ];
    }
    fn degree(&self) -> Option<u8> {
        let zeros = self.0.leading_zeros();
        if zeros == 16 {
            None
        } else {
            u8::try_from(15 - zeros).ok()
        }
    }

    fn mul(&self, other: &BiPoly) -> BiPoly16 {
        // Split these u8 to array of limbs
        let arr = self.to_array();
        let brr = other.to_array();
        let mut out = [false; 16 + 8];

        for i in 0..16 {
            for j in 0..8 {
                // & is a multiplication mod 2 and ^ is addition mod 2
                out[i + j] = (arr[i] & brr[j]) ^ out[i + j];
            }
        }

        return BiPoly16(
            (u16::from(out[0]) << 0)
                | (u16::from(out[1]) << 1)
                | (u16::from(out[2]) << 2)
                | (u16::from(out[3]) << 3)
                | (u16::from(out[4]) << 4)
                | (u16::from(out[5]) << 5)
                | (u16::from(out[6]) << 6)
                | (u16::from(out[7]) << 7)
                | (u16::from(out[8]) << 8)
                | (u16::from(out[9]) << 9)
                | (u16::from(out[10]) << 10)
                | (u16::from(out[11]) << 11)
                | (u16::from(out[12]) << 12)
                | (u16::from(out[13]) << 13)
                | (u16::from(out[14]) << 14)
                | (u16::from(out[15]) << 15),
        );
    }

    fn mul16(&self, other: &BiPoly16) -> BiPoly16 {
        // Split these u8 to array of limbs
        let arr: [bool; 16] = self.to_array(); // and extend this array to match 16 limbs
        let brr: [bool; 16] = other.to_array();
        let mut out = [false; 16 + 16];

        for i in 0..16 {
            for j in 0..16 {
                //TODO but then extend also the indexes
                // & is a multiplication mod 2 and ^ is addition mod 2
                out[i + j] = (arr[i] & brr[j]) ^ out[i + j];
            }
        }

        return BiPoly16(
            (u16::from(out[0]) << 0)
                | (u16::from(out[1]) << 1)
                | (u16::from(out[2]) << 2)
                | (u16::from(out[3]) << 3)
                | (u16::from(out[4]) << 4)
                | (u16::from(out[5]) << 5)
                | (u16::from(out[6]) << 6)
                | (u16::from(out[7]) << 7)
                | (u16::from(out[8]) << 8)
                | (u16::from(out[9]) << 9)
                | (u16::from(out[10]) << 10)
                | (u16::from(out[11]) << 11)
                | (u16::from(out[12]) << 12)
                | (u16::from(out[13]) << 13)
                | (u16::from(out[14]) << 14)
                | (u16::from(out[15]) << 15),
        );
    }

    fn add(&self, other: &BiPoly) -> BiPoly16 {
        return BiPoly16(self.0 ^ other.0 as u16);
    }

    fn add16(&self, other: &BiPoly16) -> BiPoly16 {
        return BiPoly16(self.0 ^ other.0);
    }
    /// Substracting two elements is the same as adding them
    fn sub(&self, other: &BiPoly) -> BiPoly16 {
        return BiPoly16(self.0 ^ other.0 as u16);
    }

    fn sub16(&self, other: &BiPoly16) -> BiPoly16 {
        return BiPoly16(self.0 ^ other.0);
    }

    fn div16(&self, other: &BiPoly16) -> (BiPoly16, BiPoly) {
        return poly_long_division16(BiPoly16(self.0 as u16), other).unwrap();
    }
}

#[derive(Debug, Clone)]
enum PolyLongDivision {
    AttemptDivisionByZeroError,
    ReminderBiggerThanModulo,
}

impl std::fmt::Display for PolyLongDivision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Divisor is zero polynomial for which divison is undefined"
        )
    }
}

fn poly_long_division(n: BiPoly, d: BiPoly) -> Result<(BiPoly, BiPoly), PolyLongDivision> {
    if n.0 == 0 {
        return Ok((BiPoly(0), BiPoly(0)));
    }
    let div_deg = d
        .degree()
        .ok_or(PolyLongDivision::AttemptDivisionByZeroError)?;
    let mut divident = n;
    let mut quot = BiPoly(0);

    while divident
        .degree()
        .and_then(|x| Some(x >= div_deg))
        .unwrap_or(false)
    {
        let divident_deg = divident.degree().unwrap();
        let deg_diff = divident_deg - div_deg;
        let monomial = BiPoly(1 << deg_diff);

        quot = quot.add(&monomial);
        let to_sub = monomial.mulmod(&d);

        divident = match to_sub.degree() {
            None => divident,
            Some(_) => divident.sub(&to_sub),
        };
    }
    let rem = u8::try_from(divident.0).map_err(|_| PolyLongDivision::ReminderBiggerThanModulo)?;

    Ok((quot, BiPoly(rem)))
}

fn poly_long_division16(n: BiPoly16, d: &BiPoly16) -> Result<(BiPoly16, BiPoly), PolyLongDivision> {
    if n.0 == 0 {
        return Ok((BiPoly16(0), BiPoly(0)));
    }
    let div_deg = d
        .degree()
        .ok_or(PolyLongDivision::AttemptDivisionByZeroError)?;
    let mut divident = n;
    let mut quot = BiPoly16(0);

    while divident
        .degree()
        .and_then(|x| Some(x >= div_deg))
        .unwrap_or(false)
    {
        let divident_deg = divident.degree().unwrap();
        let deg_diff = divident_deg - div_deg;
        let monomial = BiPoly16(1 << deg_diff);

        quot = quot.add16(&monomial);
        let to_sub = monomial.mul16(&d);

        divident = match to_sub.degree() {
            None => divident,
            Some(_) => divident.sub16(&to_sub),
        };
    }
    let rem = u8::try_from(divident.0).map_err(|_| PolyLongDivision::ReminderBiggerThanModulo)?;

    Ok((quot, BiPoly(rem)))
}

fn poly_mod_inv(a: BiPoly) -> Option<BiPoly> {
    if a.0 == 0 {
        return Some(BiPoly(0));
    }
    let mut old_r: BiPoly16 = BiPoly16(a.0 as u16);
    let mut rem: BiPoly16 = modulo;

    let mut old_s: BiPoly16 = BiPoly16(1);
    let mut s: BiPoly16 = BiPoly16(0);

    while rem.degree().and_then(|_| Some(true)).unwrap_or(false) {
        let quotient = old_r.div16(&rem).0;

        let temp_r = rem;
        rem = old_r.sub16(&quotient.mul16(&temp_r));
        old_r = temp_r;

        let temp_s = s;
        s = old_s.sub16(&quotient.mul16(&temp_s));
        old_s = temp_s;
    }

    if old_r.0 > 1 {
        return None;
    }
    // if old_s < 0 {
    //     old_s += i16::from(b);
    // }
    //

    let modinv = u8::try_from(old_s.0).expect("Mod inv should fit into u8");
    return Some(BiPoly(modinv));
}

const modulo: BiPoly16 = BiPoly16(0b0000_0001_0010_1011);
fn S(a: BiPoly) -> BiPoly {
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

const MATRIX: [[BiPoly; 16]; 16] = [
    [
        BiPoly(0x34),
        BiPoly(0x82),
        BiPoly(0x6F),
        BiPoly(0xBF),
        BiPoly(0x8C),
        BiPoly(0x4D),
        BiPoly(0xE4),
        BiPoly(0x6B),
        BiPoly(0x68),
        BiPoly(0x01),
        BiPoly(0x9D),
        BiPoly(0xF1),
        BiPoly(0x5C),
        BiPoly(0x54),
        BiPoly(0x8E),
        BiPoly(0xFA),
    ],
    [
        BiPoly(0x1A),
        BiPoly(0x97),
        BiPoly(0xBB),
        BiPoly(0x2F),
        BiPoly(0xF5),
        BiPoly(0x48),
        BiPoly(0xC3),
        BiPoly(0x14),
        BiPoly(0x00),
        BiPoly(0xC9),
        BiPoly(0x04),
        BiPoly(0x27),
        BiPoly(0x69),
        BiPoly(0x77),
        BiPoly(0x53),
        BiPoly(0x60),
    ],
    [
        BiPoly(0x23),
        BiPoly(0xE1),
        BiPoly(0xF0),
        BiPoly(0x41),
        BiPoly(0xE6),
        BiPoly(0xC0),
        BiPoly(0xAC),
        BiPoly(0xEC),
        BiPoly(0xC1),
        BiPoly(0xE3),
        BiPoly(0x0A),
        BiPoly(0x86),
        BiPoly(0xDA),
        BiPoly(0xDC),
        BiPoly(0x24),
        BiPoly(0x89),
    ],
    [
        BiPoly(0x2E),
        BiPoly(0xBC),
        BiPoly(0xDF),
        BiPoly(0x55),
        BiPoly(0x2C),
        BiPoly(0x3D),
        BiPoly(0xA8),
        BiPoly(0xCE),
        BiPoly(0x8F),
        BiPoly(0xCD),
        BiPoly(0x80),
        BiPoly(0xB9),
        BiPoly(0x92),
        BiPoly(0xA0),
        BiPoly(0x1E),
        BiPoly(0xF9),
    ],
    [
        BiPoly(0xAA),
        BiPoly(0x96),
        BiPoly(0xCB),
        BiPoly(0x29),
        BiPoly(0x56),
        BiPoly(0x7D),
        BiPoly(0x9B),
        BiPoly(0xA1),
        BiPoly(0x5D),
        BiPoly(0x71),
        BiPoly(0x4E),
        BiPoly(0x63),
        BiPoly(0x78),
        BiPoly(0x4F),
        BiPoly(0x58),
        BiPoly(0xB4),
    ],
    [
        BiPoly(0xDB),
        BiPoly(0xB6),
        BiPoly(0xCA),
        BiPoly(0x4C),
        BiPoly(0x2B),
        BiPoly(0xF2),
        BiPoly(0x6D),
        BiPoly(0x7F),
        BiPoly(0x43),
        BiPoly(0x62),
        BiPoly(0x40),
        BiPoly(0x36),
        BiPoly(0x3C),
        BiPoly(0x28),
        BiPoly(0xFF),
        BiPoly(0x33),
    ],
    [
        BiPoly(0x39),
        BiPoly(0x07),
        BiPoly(0x70),
        BiPoly(0x9E),
        BiPoly(0xD4),
        BiPoly(0xFE),
        BiPoly(0x91),
        BiPoly(0x2A),
        BiPoly(0x38),
        BiPoly(0x7C),
        BiPoly(0xA5),
        BiPoly(0x45),
        BiPoly(0x7A),
        BiPoly(0xA9),
        BiPoly(0x49),
        BiPoly(0x81),
    ],
    [
        BiPoly(0xFC),
        BiPoly(0x5F),
        BiPoly(0xDD),
        BiPoly(0xFD),
        BiPoly(0x6E),
        BiPoly(0x17),
        BiPoly(0xE7),
        BiPoly(0x6C),
        BiPoly(0x67),
        BiPoly(0x31),
        BiPoly(0x7E),
        BiPoly(0x79),
        BiPoly(0x21),
        BiPoly(0x5A),
        BiPoly(0xC7),
        BiPoly(0x90),
    ],
    [
        BiPoly(0x7B),
        BiPoly(0xB1),
        BiPoly(0x65),
        BiPoly(0xCF),
        BiPoly(0xDE),
        BiPoly(0xB5),
        BiPoly(0xAF),
        BiPoly(0xF7),
        BiPoly(0x05),
        BiPoly(0xED),
        BiPoly(0x85),
        BiPoly(0x37),
        BiPoly(0xF6),
        BiPoly(0x0F),
        BiPoly(0xEB),
        BiPoly(0x26),
    ],
    [
        BiPoly(0x95),
        BiPoly(0x5E),
        BiPoly(0x83),
        BiPoly(0xF4),
        BiPoly(0x09),
        BiPoly(0x73),
        BiPoly(0x8A),
        BiPoly(0xC6),
        BiPoly(0x12),
        BiPoly(0xE5),
        BiPoly(0x9C),
        BiPoly(0xB2),
        BiPoly(0x02),
        BiPoly(0x59),
        BiPoly(0x74),
        BiPoly(0xCC),
    ],
    [
        BiPoly(0xD6),
        BiPoly(0xA4),
        BiPoly(0x75),
        BiPoly(0x25),
        BiPoly(0x4B),
        BiPoly(0x52),
        BiPoly(0x08),
        BiPoly(0x8B),
        BiPoly(0xAE),
        BiPoly(0x3E),
        BiPoly(0x57),
        BiPoly(0xE8),
        BiPoly(0x8D),
        BiPoly(0xE2),
        BiPoly(0x84),
        BiPoly(0x72),
    ],
    [
        BiPoly(0x9A),
        BiPoly(0xBE),
        BiPoly(0x1F),
        BiPoly(0xC2),
        BiPoly(0x0E),
        BiPoly(0x5B),
        BiPoly(0x35),
        BiPoly(0xA6),
        BiPoly(0x30),
        BiPoly(0x98),
        BiPoly(0x3A),
        BiPoly(0x0C),
        BiPoly(0xC4),
        BiPoly(0x1B),
        BiPoly(0xA2),
        BiPoly(0x93),
    ],
    [
        BiPoly(0xA7),
        BiPoly(0x20),
        BiPoly(0xB8),
        BiPoly(0xB3),
        BiPoly(0x16),
        BiPoly(0x3F),
        BiPoly(0x61),
        BiPoly(0xF8),
        BiPoly(0x44),
        BiPoly(0x47),
        BiPoly(0x51),
        BiPoly(0x6A),
        BiPoly(0xF3),
        BiPoly(0x0B),
        BiPoly(0x3B),
        BiPoly(0xEA),
    ],
    [
        BiPoly(0x32),
        BiPoly(0xAD),
        BiPoly(0x10),
        BiPoly(0x42),
        BiPoly(0xE9),
        BiPoly(0x15),
        BiPoly(0x99),
        BiPoly(0x1D),
        BiPoly(0x13),
        BiPoly(0xBD),
        BiPoly(0xEF),
        BiPoly(0xEE),
        BiPoly(0x9F),
        BiPoly(0xE0),
        BiPoly(0xFB),
        BiPoly(0xBA),
    ],
    [
        BiPoly(0x50),
        BiPoly(0xD0),
        BiPoly(0x94),
        BiPoly(0xD1),
        BiPoly(0xD5),
        BiPoly(0xD7),
        BiPoly(0xC5),
        BiPoly(0xD9),
        BiPoly(0x19),
        BiPoly(0x46),
        BiPoly(0xB0),
        BiPoly(0x06),
        BiPoly(0xC8),
        BiPoly(0xD3),
        BiPoly(0x18),
        BiPoly(0x64),
    ],
    [
        BiPoly(0x88),
        BiPoly(0xD2),
        BiPoly(0xA3),
        BiPoly(0x4A),
        BiPoly(0x11),
        BiPoly(0x1C),
        BiPoly(0x87),
        BiPoly(0x22),
        BiPoly(0xAB),
        BiPoly(0x0D),
        BiPoly(0x03),
        BiPoly(0xB7),
        BiPoly(0xD8),
        BiPoly(0x2D),
        BiPoly(0x66),
        BiPoly(0x76),
    ],
];

fn to_array(matrix: &Matrix) -> [u8; 16] {
    let flattened: Vec<u8> = matrix.iter().flatten().map(|x| x.0).collect();
    flattened.try_into().expect("Could not map vec to array")
}

const BLOCK_SIZE: usize = 16;
const LENGTH_SIZE: usize = 2;

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
            let hash = whirlpool([[BiPoly(0); 4]; 4], input);
            let expected = [
                0xEA, 0xBC, 0x8C, 0x30, 0x17, 0xDC, 0x2D, 0x09, 0x60, 0x9E, 0x2A, 0x27, 0x2B, 0x26,
                0x0B, 0xE1,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let input = [0u8; 16];
            let hash = whirlpool([[BiPoly(0); 4]; 4], input);
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
            let hash = input.chunks(16).fold([[BiPoly(0); 4]; 4], |acc, element| {
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
            let hash = input.chunks(16).fold([[BiPoly(0); 4]; 4], |acc, element| {
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
            let hash = input.chunks(16).fold([[BiPoly(0); 4]; 4], |acc, element| {
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
            let hash = input.chunks(16).fold([[BiPoly(0); 4]; 4], |acc, element| {
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
            let hash = input.chunks(16).fold([[BiPoly(0); 4]; 4], |acc, element| {
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
            let hash =
                input
                    .chunks(16)
                    .enumerate()
                    .fold([[BiPoly(0); 4]; 4], |acc, (i, element)| {
                        println!("folding {}% {}/{}", 100 * i / (48000 / 16), i, 48000 / 16);
                        whirlpool(acc, element.try_into().expect("Slice with incorrect size"))
                    });
            let expected = [
                0x4A, 0x07, 0x19, 0x09, 0xC7, 0xA6, 0xBD, 0x41, 0x5B, 0xB8, 0xA2, 0x41, 0x87, 0xB3,
                0x61, 0xEB,
            ];
            assert_eq!(to_array(&hash), expected);
        }
        {
            let mut input = &mut "a".repeat(48479).as_bytes().to_vec();
            add_padding(&mut input);
            let hash =
                input
                    .chunks(16)
                    .enumerate()
                    .fold([[BiPoly(0); 4]; 4], |acc, (i, element)| {
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
                    .fold([[BiPoly(0); 4]; 4], |acc, (i, element)| {
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
    #[test]
    fn test_to_array() {
        let m = BiPoly(0b00000011);
        assert_eq!(
            m.to_array(),
            [true, true, false, false, false, false, false, false]
        );

        let a = BiPoly16(0b0000_0001_0000_0010);
        assert_eq!(
            a.to_array(),
            [
                false, true, false, false, false, false, false, false, true, false, false, false,
                false, false, false, false
            ]
        );
    }

    #[test]
    fn test_poly_long_division() {
        {
            println!("---------------------");
            let a = BiPoly(0b0000_0110);
            let m = BiPoly(0b0000_0011);
            let (quot, rem) = poly_long_division(a, m).unwrap();
            assert_eq!(quot.0, BiPoly(0b0000_0010).0);
            assert_eq!(rem.0, BiPoly(0b0000_0000).0);
        }
        {
            println!("---------------------");
            let a = BiPoly(0b0000_0111);
            let m = BiPoly(0b0000_0011);
            let (quot, rem) = poly_long_division(a, m).unwrap();
            assert_eq!(quot.0, BiPoly(0b0000_0010).0);
            assert_eq!(rem.0, BiPoly(0b0000_0001).0);
        }
        {
            println!("---------------------");
            let a = BiPoly(0b1100_0011);
            let m = BiPoly(0b0001_1001);
            let (quot, rem) = poly_long_division(a, m).unwrap();
            assert_eq!(quot.0, BiPoly(0b0000_1000).0);
            assert_eq!(rem.0, BiPoly(0b0000_1011).0);
        }
    }

    #[test]
    fn test_poly_mod_inv() {
        let eca = poly_mod_inv(BiPoly(0b1011));
        assert!(eca.is_some());
        assert_eq!(eca.unwrap().0, 0b0000_0001);

        let eca = poly_mod_inv(BiPoly(0b0100));
        assert!(eca.is_some());
        assert_eq!(eca.unwrap().0, 0b0000_1101);
    }
}
