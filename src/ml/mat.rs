pub trait MatExtension {
    fn scale(&self, scalar: f32) -> Self;
    fn rows(&self) -> usize;
    fn sum(&self) -> f32;
    fn sum_rows(&self) -> Self;
    fn power(&self, exp: f32) -> Self;
}

impl MatExtension for faer::Mat<f32> {
    fn scale(&self, scalar: f32) -> Self {
        Self::from_fn(self.nrows(), self.ncols(), |row, column| {
            self.get(row, column) * scalar
        })
    }

    fn rows(&self) -> usize {
        self.nrows()
    }
    fn sum(&self) -> f32 {
        let mut sum = 0_f32;
        for row in 0..self.nrows() {
            for column in 0..self.ncols() {
                sum += self.get(row, column);
            }
        }
        sum
    }
    fn sum_rows(&self) -> Self {
        Self::from_fn(self.nrows(), 1, |row, _column| {
            let mut sum = 0_f32;
            for column in 0..self.ncols() {
                sum += self.get(row, column);
            }
            sum
        })
    }
    fn power(&self, exp: f32) -> Self {
        Self::from_fn(self.nrows(), self.ncols(), |row, column| {
            self.get(row, column).powf(exp)
        })
    }
}
