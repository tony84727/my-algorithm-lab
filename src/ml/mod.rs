mod mat;
use faer::Mat;

use self::mat::MatExtension;

pub struct LinearRegression {
    weights: Mat<f32>,
}

fn compute_gradient(x: &Mat<f32>, y: &Mat<f32>, weights: &Mat<f32>) -> Mat<f32> {
    let predict = x * weights;
    return (x.transpose() * &(predict - y))
        .sum_rows()
        .scale(1.0 / x.rows() as f32);
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
    pub fn get_weights(&self) -> &Mat<f32> {
        &self.weights
    }
    pub fn set_weights(&mut self, weights: Mat<f32>) -> &mut Self {
        self.weights = weights;
        self
    }
    pub fn predict(&self, input: &Mat<f32>) -> Mat<f32> {
        input * &self.weights
    }
    pub fn mse_cost(&self, x: &Mat<f32>, y: &Mat<f32>) -> f32 {
        (self.predict(x) - y).power(2.0).sum()
    }
    pub fn fit(&mut self, x: &Mat<f32>, y: &Mat<f32>, learning_rate: f32, epochs: usize) {
        for _ in 0..epochs {
            let weights = self.get_weights();
            let update = compute_gradient(x, y, weights).scale(learning_rate);
            self.set_weights(weights - update);
        }
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

    #[test]
    fn test_fit() {
        let mut model = LinearRegression::with_weights(mat![[0_f32], [0_f32]]);
        let x = mat![[1_f32, 1_f32], [2_f32, 1_f32]];
        let y = mat![[300_f32], [500_f32]];
        model.fit(&x, &y, 0.01, 5000);
        assert!(model.mse_cost(&x, &y) <= 1.0);
    }
}
