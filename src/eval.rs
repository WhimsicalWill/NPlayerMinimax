use rand::Rng;
use crate::game::GameState;

pub trait EvaluationFunction {
    fn evaluate(&self, state: &GameState) -> Vec<f64>;
}

pub struct RandomEvaluationFunction {
    num_players: usize,
}

impl RandomEvaluationFunction {
    pub fn new(num_players: usize) -> Self {
        RandomEvaluationFunction {
            num_players,
        }
    }
}

impl EvaluationFunction for RandomEvaluationFunction {
    // The random evaluation function doesn't actually use the state
    fn evaluate(&self, _state: &GameState) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        
        // Generate n-1 random numbers between 0 and 1
        let mut numbers: Vec<f64> = (0..self.num_players - 1)
            .map(|_| rng.gen::<f64>())
            .collect();

        // Add 0 at the start and 1 at the end, then sort
        numbers.push(1.0);
        numbers.insert(0, 0.0);
        numbers.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate the difference between adjacent numbers to get the vector
        let vector: Vec<f64> = numbers.windows(2).map(|w| w[1] - w[0]).collect();

        vector
    }
}