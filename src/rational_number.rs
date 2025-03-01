#[derive(Debug)]
pub struct RatNum {
    n: isize, // numerator
    d: isize, // denominator
}

impl RatNum {
    pub fn new(numerator: isize, denominator: isize) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero!");
        }
        RatNum { n: numerator, d: denominator }
    }

    pub fn from_int(int: isize) -> Self {
        RatNum { n: int, d: 1 }
    }

    pub fn add(&self, other: &RatNum) -> RatNum {
        let new_n = self.n * other.d + other.n * self.d;
        let new_d = self.d * other.d;
        let mut result = RatNum::new(new_n, new_d);
        result.simplify();
        result
    }

    pub fn multiply(&self, other: &RatNum) -> RatNum {
        let new_n = self.n * other.n;
        let new_d = self.d * other.d;
        let mut result = RatNum::new(new_n, new_d);
        result.simplify();
        result
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.n, self.d)
    }

    fn simplify(&mut self) {
        let gcd = self.greatest_common_divisor(self.n.abs(), self.d.abs());
        self.n /= gcd;
        self.d /= gcd;
        
        // Move minus sign to numerator
        if self.d < 0 {
            self.n *= -1;
            self.d *= -1;
        }
    }
    fn greatest_common_divisor(&self, a: isize, b: isize) -> isize {
        let mut x = a;
        let mut y = b;

        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }

        x
    }
}