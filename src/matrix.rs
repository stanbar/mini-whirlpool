use super::bipoly::BiPoly;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Matrix(pub [[BiPoly; 4]; 4]);

impl Matrix {
    pub fn zeros() -> Matrix {
        Matrix([[BiPoly(0); 4]; 4])
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| {
                    x.iter()
                        .map(|y| format!("{:x}", y.0))
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut out = Matrix::zeros();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    out.0[i][j] = self.0[i][k].mulmod(&rhs.0[k][j]).add(&out.0[i][j]);
                }
            }
        }

        out
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        let mut c = Matrix::zeros();
        for row in 0..4 {
            for col in 0..4 {
                c.0[row][col] = self.0[row][col].add(&rhs.0[row][col])
            }
        }
        c
    }
}
