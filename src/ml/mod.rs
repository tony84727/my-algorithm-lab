use faer::Mat;

pub struct LinearRegression {
    weights: Mat<f32>,
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
    pub fn predict(&self, input: Mat<f32>) -> Mat<f32> {
        input * &self.weights
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
            model.predict(mat![[1_f32, 1_f32], [2_f32, 1_f32]])
        );
    }
}
