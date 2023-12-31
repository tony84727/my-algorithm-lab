use faer::Mat;

pub struct LinearRegression {
    weights: Mat<f32>,
}

fn mat_power(mat: &mut Mat<f32>, power: f32) {
    for row in 0..mat.nrows() {
        for column in 0..mat.ncols() {
            mat.write(row, column, mat.read(row, column).powf(power));
        }
    }
}
fn mat_sum(mat: &Mat<f32>) -> f32 {
    let mut sum = 0_f32;
    for row in 0..mat.nrows() {
        for column in 0..mat.ncols() {
            sum += mat.read(row, column);
        }
    }
    sum
}

impl LinearRegression {
    pub fn new(feature_count: usize) -> Self {
        Self {
            weights: Mat::zeros(feature_count, 1),
        }
    }
    pub fn with_weights(weights: Mat<f32>) -> Self {
        Self { weights }
    }
    pub fn predict(&self, input: &Mat<f32>) -> Mat<f32> {
        input * &self.weights
    }
    pub fn mse_cost(&self, x: &Mat<f32>, y: &Mat<f32>) -> f32 {
        let mut error = self.predict(x) - y;
        mat_power(&mut error, 2.0);
        mat_sum(&error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use faer::mat;
    #[test]
    fn test_predict() {
        let model = LinearRegression::with_weights(mat![[200_f32], [100_f32]]);
        assert_eq!(
            mat![[300_f32], [500_f32]],
            model.predict(&mat![[1_f32, 1_f32], [2_f32, 1_f32]])
        );
    }

    #[test]
    fn test_cost_perfect() {
        let model = LinearRegression::with_weights(mat![[200_f32], [100_f32]]);
        let input = mat![[123_f32, 1_f32], [456_f32, 1_f32]];
        assert_eq!(0.0, model.mse_cost(&input, &model.predict(&input)));
    }

    #[test]
    fn test_cost() {
        let model = LinearRegression::with_weights(mat![[0_f32], [0_f32]]);
        let input = mat![[1_f32, 1_f32], [2_f32, 1_f32]];
        assert_eq!(
            300_f32.powi(2) + 500_f32.powi(2),
            model.mse_cost(&input, &mat![[300_f32], [500_f32]])
        );
    }
}
