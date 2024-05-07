// mnist_loader
// ~~~~~~~~~~~~

// A library to load the MNIST image data.  For details of the data
// structures that are returned, see the doc strings for ``load_data``
// and ``load_data_wrapper``.  In practice, ``load_data_wrapper`` is the
// function usually called by our neural network code.

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

fn vectorized_result(x: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for val in 0..10 {
        if x == val {
            res.push(1);
        } else {
            res.push(0);
        }
    }
    res
}

fn load_data() -> Result<
    (
        (Vec<Vec<f64>>, Vec<i32>),
        (Vec<Vec<f64>>, Vec<i32>),
        (Vec<Vec<f64>>, Vec<i32>),
    ),
    serde_pickle::Error,
> {
    // Open the gzipped MNIST data file
    let file = File::open(
        "file.pkl.gz",
    )?;
    let mut decoder = GzDecoder::new(file); // Wrap the file stream with GzDecoder

    // Read the entire decompressed content into a vector
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    let (training_data, validation_data, test_data): (
        (Vec<Vec<f64>>, Vec<i32>),
        (Vec<Vec<f64>>, Vec<i32>),
        (Vec<Vec<f64>>, Vec<i32>),
    ) = serde_pickle::from_reader(&mut &data[..], Default::default())?;

    Ok((training_data, validation_data, test_data))
}

pub fn load_data_wrapper() -> Result<
    (
        (Vec<Vec<f64>>, Vec<Vec<i32>>),
        (Vec<Vec<f64>>, Vec<Vec<i32>>),
        (Vec<Vec<f64>>, Vec<i32>),
    ),
    serde_pickle::Error,
> {
    let (training, validation, test) =  load_data()?;
    let mut training_data: Vec<Vec<i32>> = Vec::new();
    for val in training.1.into_iter() {
        training_data.push(vectorized_result(val));
    }
    Ok(((training.0, training_data.clone()), (validation.0, training_data.clone()), test))
}