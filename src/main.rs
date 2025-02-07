mod perceptron;
mod plt;
use perceptron::simple_perceptron::Perceptron;
use rand::Rng;

fn main() {
    const EPOCHS: u32 = 500;
    const DATA_SIZE: u32 = 2000;
    const SEPARATION: f32 = 0.002;
    const LEARNING_RATE: f32 = 0.3;
    let data = fake_data(DATA_SIZE,SEPARATION);
    let data_start: Vec<(f32, f32, i8)> =
        data.iter().map(|(x, label)| (x[0], x[1], *label)).collect();
    let real = data.iter().map(|(_, label)| *label).collect::<Vec<i8>>();
    plt::plot::scatter_plot(&data_start, "scatter_plot").unwrap();
    let mut perceptron = perceptron::simple_perceptron::Perceptron::new( LEARNING_RATE, 2);
    perceptron.fit(&data, EPOCHS);
    let x: Vec<&[f32]> = data.iter().map(|(x, _)| x.as_slice()).collect();
    let y = perceptron.predict(&x);
    let data: Vec<(f32, f32, i8)> = data
        .iter()
        .zip(y.iter())
        .map(|((x, _), y_pred)| (x[0], x[1], *y_pred))
        .collect();
    plt::plot::scatter_plot(&data, "Perceptron").unwrap();
    let accuraccy = Perceptron::score(&y, &real);
    println!("The accuraccy of the Perceptron is: {}", accuraccy);
}

/// Generate fake data for testing
/// #Arguments
/// * `data_size` - The number of data points to generate
/// * `separation` - The separation between the two classes
///
/// #Returns
/// A vector of tuples containing the x and y coordinates and the label
fn fake_data(data_size: u32, separation: f32) -> Vec<(Vec<f32>, i8)> {
    let mut rng = rand::rng();
    let mut data: Vec<(Vec<f32>, i8)> = Vec::with_capacity(data_size as usize);
    let mut x = Vec::with_capacity(data_size as usize);
    let mut y = Vec::with_capacity(data_size as usize);
    for _ in 0..data_size {
        x.push(rng.random_range(0.0..=1.0f32));
    }
    let sep1 = 0.5 - separation / 2.0;
    let sep2 = 0.5 + separation / 2.0;
    for _ in 0..data_size / 2 {
        let value = rng.random_range(0.0..=1.0f32) * sep1;
        y.push(value);
    }
    for _ in data_size / 2..data_size {
        let value = rng.random_range(0.0..=1.0f32) * sep1 + sep2;
        y.push(value);
    }
    let mut labels: Vec<i8> = Vec::with_capacity(data_size as usize);
    for i in 0..data_size {
        if i < data_size / 2 {
            labels.push(-1);
        } else {
            labels.push(1);
        }
    }
    x.into_iter()
        .zip(y)
        .zip(labels)
        .map(|((x, y), label)| (vec![x, y], label))
        .collect()
}
