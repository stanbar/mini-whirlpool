use super::constants::MODULO;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum PolyLongDivisionError {
    AttemptDivisionByZeroError,
    ReminderBiggerThanModulo,
}

impl std::fmt::Display for PolyLongDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Divisor is zero polynomial for which divison is undefined"
        )
    }
}
/// Binary Ring Polynomial element
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiPoly(pub u8);

impl std::fmt::Display for BiPoly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiPoly16(pub u16);

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
    pub fn add(&self, other: &BiPoly) -> BiPoly {
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
    pub fn mulmod(&self, other: &BiPoly) -> BiPoly {
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
        .div16(&MODULO)
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

    pub fn degree(&self) -> Option<u8> {
        let zeros = self.0.leading_zeros();
        if zeros == 8 {
            None
        } else {
            u8::try_from(7 - zeros).ok()
        }
    }
}

impl BiPoly16 {
    pub fn to_array(&self) -> [bool; 16] {
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
    pub fn degree(&self) -> Option<u8> {
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

fn poly_long_division(n: BiPoly, d: BiPoly) -> Result<(BiPoly, BiPoly), PolyLongDivisionError> {
    if n.0 == 0 {
        return Ok((BiPoly(0), BiPoly(0)));
    }
    let div_deg = d
        .degree()
        .ok_or(PolyLongDivisionError::AttemptDivisionByZeroError)?;
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
    let rem = u8::try_from(divident.0).map_err(|_| PolyLongDivisionError::ReminderBiggerThanModulo)?;

    Ok((quot, BiPoly(rem)))
}

fn poly_long_division16(n: BiPoly16, d: &BiPoly16) -> Result<(BiPoly16, BiPoly), PolyLongDivisionError> {
    if n.0 == 0 {
        return Ok((BiPoly16(0), BiPoly(0)));
    }
    let div_deg = d
        .degree()
        .ok_or(PolyLongDivisionError::AttemptDivisionByZeroError)?;
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
    let rem = u8::try_from(divident.0).map_err(|_| PolyLongDivisionError::ReminderBiggerThanModulo)?;

    Ok((quot, BiPoly(rem)))
}

fn poly_mod_inv(a: BiPoly) -> Option<BiPoly> {
    if a.0 == 0 {
        return Some(BiPoly(0));
    }
    let mut old_r: BiPoly16 = BiPoly16(a.0 as u16);
    let mut rem: BiPoly16 = MODULO;

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



#[cfg(test)]
mod tests {
    use super::*;

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
