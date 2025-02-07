pub mod simple_perceptron {
    pub struct Perceptron {
        weights: Vec<f32>,
        learning_rate: f32,
        bias: f32,
    }

    impl Perceptron {
        /// Create a new Perceptron
        /// # Arguments
        /// * `learning_rate` - The learning rate of the Perceptron
        /// * `num_weights` - The number of weights in the Perceptron
        pub fn new(learning_rate: f32, num_weights: usize) -> Perceptron {
            Perceptron {
                weights: vec![0.0; num_weights],
                learning_rate,
                bias: 1.5,
            }
        }
        /// Fit the Perceptron to the data
        /// # Arguments
        /// * `data` - The data to fit the Perceptron to
        /// * `epochs` - The number of epochs to train the Perceptron
        pub fn fit(&mut self, data: &[(Vec<f32>, i8)], epochs: u32) {
            for _ in 0..epochs {
                for (input, label) in data.iter() {
                    let mut sum = 0.0;
                    for (i, x) in input.iter().enumerate() {
                        sum += self.weights[i] * (*x);
                    }
                    sum += self.bias;
                    let prediction = self.activate(sum);
                    let error = label - prediction;
                    if error != 0 {
                        for (i, x) in input.iter().enumerate() {
                            self.weights[i] += self.learning_rate * error as f32 * (*x);
                        }
                        self.bias += self.learning_rate * error as f32;
                    }
                }
            }
        }
        /// Predict the output of the Perceptron
        /// # Arguments
        /// * `input` - The input to predict
        /// 
        /// # Returns
        /// A vector of predictions
        pub fn predict(&mut self, input: &[&[f32]]) -> Vec<i8> {
            let mut predictions = Vec::with_capacity(input.len());
            for element in input.iter() {
                let mut sum = 0.0;
                for (i,x) in element.iter().enumerate() {
                    sum += self.weights[i] * (*x);
                }
                sum += self.bias;

                predictions.push(self.activate(sum));
            }
            predictions
        }
        /// Activate the Perceptron
        /// # Arguments
        /// * `input` - The input to activate
        pub fn activate(&mut self, input: f32) -> i8 {
            if input >= 0.0 {
                1
            } else {
                -1
            }
        }
        /// Calculate the score of the Perceptron
        /// # Arguments
        /// * `pred` - The predictions of the Perceptron
        /// * `real` - The real labels
        /// 
        /// # Returns
        /// The score of the Perceptron
        pub fn score(pred: &[i8],real: &[i8])->f32{
            let mut score = 0.0;
            for (p,r) in pred.iter().zip(real){
                if p == r{
                    score += 1.0;
                }
            }
            score / pred.len() as f32
        }
    }
}
