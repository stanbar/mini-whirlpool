/// Binary Ring Polynomial element
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiPoly(pub u8);

impl std::fmt::Display for BiPoly {
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
    /// 1111 + 0101
    /// = (x^3 + x^2 + x + 1 ) + (x^2 + 1)
    /// = x^3 + 2*x^2 + x + 2
    /// Since Z_2 we perform mod 2 on each coefficient
    /// = (1 mod 2)x^3 + (2 mod 2)x^2 + (1 mod 2)x + (2 mod 2)x
    /// = x^3 + 0x^2 + 1x + 0
    /// = x^3 + x
    /// 1010 = 1111 ^ 0101
    pub fn add(&self, other: &BiPoly) -> BiPoly {
        return BiPoly(self.0 ^ other.0);
    }

    pub fn mulmod(&self, other: &BiPoly) -> BiPoly {
        // Split these u8 to array of bits
        let arr = self.to_array();
        let brr = other.to_array();
        let mut out = [false; 16];

        // 0xFF * 0xFF
        // = 0b1111_1111 * 0b1111_1111
        // = (x^7  + x^6  + x^5  + x^4  + x^3  + x^2 + x   + 1) * (x^7 + x^6 + x^5 + x^4 + x^3 + x^2 + x + 1) =
        // = (x^14 + x^13 + x^12 + x^11 + x^10 + x^9 + x^8 + x^7)
        //   +      (x^13 + x^12 + x^11 + x^10 + x^9 + x^8 + x^7 + x^6)
        //   +             (x^12 + x^11 + x^10 + x^9 + x^8 + x^7 + x^6 + x^5)
        //   +                    (x^11 + x^10 + x^9 + x^8 + x^7 + x^6 + x^5 + x^4)
        //   +                           (x^10 + x^9 + x^8 + x^7 + x^6 + x^5 + x^4 + x^3)
        //   +                                  (x^9 + x^8 + x^7 + x^6 + x^5 + x^4 + x^3 + x^2)
        //   +                                        (x^8 + x^7 + x^6 + x^5 + x^4 + x^3 + x^2 + x^1)
        //   +                                              (x^7 + x^6 + x^5 + x^4 + x^3 + x^2 + x^1 + x^0)
        // = x^14 + 2 x^13 + 3 x^12 + 4 x^11 + 5 x^10 + 6 x^9 + 7 x^8 + 8 x^7 + 7 x^6 + 6 x^5 + 5 x^4 + 4 x^3 + 3 x^2 + 2 x + 1
        // Z_2 reduction
        // = x^14 + x^12 + x^10 + x^8 + x^6 + x^4 + x^2 + 1

        out[14] = arr[7] & brr[7];
        out[13] = (arr[7] & brr[6]) ^ (arr[6] & brr[7]);
        out[12] = (arr[7] & brr[5]) ^ (arr[5] & brr[7]) ^ (arr[6] & brr[6]);
        out[11] = (arr[7] & brr[4]) ^ (arr[4] & brr[7]) ^ (arr[6] & brr[5]) ^ (arr[5] & brr[6]);
        out[10] = (arr[7] & brr[3])
            ^ (arr[3] & brr[7])
            ^ (arr[6] & brr[4])
            ^ (arr[4] & brr[6])
            ^ (arr[5] & brr[5]);
        out[9] = (arr[7] & brr[2])
            ^ (arr[2] & brr[7])
            ^ (arr[6] & brr[3])
            ^ (arr[3] & brr[6])
            ^ (arr[5] & brr[4])
            ^ (arr[4] & brr[5]);
        out[8] = (arr[7] & brr[1])
            ^ (arr[1] & brr[7])
            ^ (arr[6] & brr[2])
            ^ (arr[2] & brr[6])
            ^ (arr[5] & brr[3])
            ^ (arr[3] & brr[5])
            ^ (arr[4] & brr[4]);
        out[7] = (arr[0] & brr[7])
            ^ (arr[7] & brr[0])
            ^ (arr[6] & brr[1])
            ^ (arr[1] & brr[6])
            ^ (arr[5] & brr[2])
            ^ (arr[2] & brr[5])
            ^ (arr[3] & brr[4])
            ^ (arr[4] & brr[3]);
        out[6] = (arr[0] & brr[6])
            ^ (arr[6] & brr[0])
            ^ (arr[5] & brr[1])
            ^ (arr[1] & brr[5])
            ^ (arr[4] & brr[2])
            ^ (arr[2] & brr[4])
            ^ (arr[3] & brr[3]);
        out[5] = (arr[0] & brr[5])
            ^ (arr[5] & brr[0])
            ^ (arr[4] & brr[1])
            ^ (arr[1] & brr[4])
            ^ (arr[3] & brr[2])
            ^ (arr[2] & brr[3]);
        out[4] = (arr[0] & brr[4])
            ^ (arr[4] & brr[0])
            ^ (arr[3] & brr[1])
            ^ (arr[1] & brr[3])
            ^ (arr[2] & brr[2]);
        out[3] = (arr[0] & brr[3]) ^ (arr[3] & brr[0]) ^ (arr[2] & brr[1]) ^ (arr[1] & brr[2]);
        out[2] = (arr[0] & brr[2]) ^ (arr[2] & brr[0]) ^ (arr[1] & brr[1]);
        out[1] = (arr[0] & brr[1]) ^ (arr[1] & brr[0]);
        out[0] = arr[0] & brr[0];

        // Alternatively but slower
        // for i in 0..8 {
        //     for j in 0..8 {
        //         out[i + j] = (arr[i] & brr[j]) ^ out[i + j];
        //     }
        // }

        // Since we are working in ring, we need to reduce the output by modulo x^8 + x^5 + x^3 + x + 1
        // We could just divide the output polynomil by moduolo polynomial using log polynomial
        // division, but it is computationaly expensive; instead, we use simiple substitution.
        // Notice:
        // we can precompute the congruences for each monomial bigger or equal the degree of modulo
        // The biggest polynomial we can get by multyplying 0xFF x 0xFF is 14 (x^7 + x^7 = x^14),
        // therefore we need to precompute the reductions for x^14, x^13, x^12, ..., x^8 and
        // substitute each of them in the result. Let's start from x^8. We know that
        // x^8 + x^5 + x^3 + x + 1 == 0 (mod x^8 + x^5 + x^3 + x + 1)
        // Let's move everything besides x^8 on the right hand side.
        // x^8 = x^5 + x^3 + x + 1 (mod x^8 + x^5 + x^3 + x + 1)
        // Now each time we find a x^8 in our equation, we can substitute it with x^5 + x^3 + x + 1,
        // thanks to congruency property of modulo. Let's move on.
        // x^9 = x^8 * x = (x^5 + x^3 + x + 1) * x = x^6 + x^4 + x^2 + x (mod x^8 + x^5 + x^3 + x + 1)
        // x^10 = x^9 * x = (x^6 + x^4 + x^2 + x) * x = x^7 * x^4 * x^3 + x^2 (mod x^8 + x^5 + x^3 + x + 1)
        // x^11 = x^10 * x = (x^7 * x^4 * x^3 + x^2) * x = x^8 + x^5 + x^4 + x^3 =
        // Notice that we get x^8, so we need to perform the substitution one more time
        // = (x^5 + x^3 + x + 1) + x^5 + x^4 + x^3 = x^4 + 1 (mod x^8 + x^5 + x^3 + x + 1)
        // x^12 = x^11 * x = (x^4 + 1) * x = x^5 + x (mod x^8 + x^5 + x^3 + x + 1)
        // x^13 = x^12 * x = (x^5 + x) * x = x^6 + x^2 (mod x^8 + x^5 + x^3 + x + 1)
        // x^14 = (x^6 + x^2) * x = x^7 + x^3 (mod x^8 + x^5 + x^3 + x + 1)
        //
        // Now having our substitution table
        // x^8 = x^5 + x^3 + x + 1
        // x^9 = x^6 + x^4 + x^2 + x
        // x^10 = x^7 + x^4 + x^3 + x^2
        // x^11 = x^4 + 1
        // x^12 = x^5 + x
        // x^13 = x^6 + x^2
        // x^14 = x^7 + x^3
        //
        // We can compute our 0xFF * 0xFF (mod 0x12B)
        // = x^14 + x^12 + x^10 + x^8 + x^6 + x^4 + x^2 + 1 (mod x^8 + x^5 + x^3 + x + 1)
        // = x^7 + x^3 + x^5 + x + x^7 + x^4 + x^3 + x^2 + x^5 + x^3 + x + 1 + x^6 + x^4 + x^2 + 1
        // = 2 x^7 + x^6 + 2 x^5 + 2 x^4 + 3 x^3 + 2 x^2 + 2 x + 2
        // Reduce Z_2 and we get
        // x^6 + x^3

        // x^8 = x^5 + x^3 + x + 1
        if out[8] {
            out[5] ^= true;
            out[3] ^= true;
            out[1] ^= true;
            out[0] ^= true;
        }

        // x^9 = x^6 + x^4 + x^2 + x
        if out[9] {
            out[6] ^= true;
            out[4] ^= true;
            out[2] ^= true;
            out[1] ^= true;
        }
        // x^10 = x^7 + x^4 + x^3 + x^2
        if out[10] {
            out[7] ^= true;
            out[4] ^= true;
            out[3] ^= true;
            out[2] ^= true;
        }
        // x^11 = x^4 + 1
        if out[11] {
            out[4] ^= true;
            out[0] ^= true;
        }
        // x^12 = x^5 + x
        if out[12] {
            out[5] ^= true;
            out[0] ^= true;
        }
        // x^13 = x^6 + x^2
        if out[13] {
            out[6] ^= true;
            out[2] ^= true;
        }
        // x^14 = x^7 + x^3
        if out[14] {
            out[7] ^= true;
            out[3] ^= true;
        }

        BiPoly(
            (u8::from(out[0]) << 0)
                | (u8::from(out[1]) << 1)
                | (u8::from(out[2]) << 2)
                | (u8::from(out[3]) << 3)
                | (u8::from(out[4]) << 4)
                | (u8::from(out[5]) << 5)
                | (u8::from(out[6]) << 6)
                | (u8::from(out[7]) << 7),
        )
    }
}
