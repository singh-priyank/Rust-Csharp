// mnist_loader
// ~~~~~~~~~~~~

// A library to load the MNIST image data.  For details of the data
// structures that are returned, see the doc strings for ``load_data``
// and ``load_data_wrapper``.  In practice, ``load_data_wrapper`` is the
// function usually called by our neural network code.


use std::fs::File;
use std::io::Read;
use flate2::read::GzDecoder;

pub fn load_data() -> Result<(), serde_pickle::Error> {
  // Open the gzipped MNIST data file
  let file = File::open("C:\\Users\\PriyankSingh\\Neural-Networks-and-Deep-Learning\\data\\mnist.pkl.gz")?;
  let mut decoder = GzDecoder::new(file); // Wrap the file stream with GzDecoder

  // Read the entire decompressed content into a vector
  let mut data = Vec::new();
  decoder.read_to_end(&mut data)?;

  // Deserialize the data from the raw bytes using serde_pickle
  let (training_data, validation_data, test_data): (
    Vec<(Vec<Vec<f32>>, Vec<Vec<f32>>)>, 
    Vec<(Vec<Vec<f32>>, Vec<Vec<f32>>)>, 
    Vec<(Vec<Vec<f32>>, Vec<Vec<f32>>)>
  ) = serde_pickle::from_reader(&mut &data[..], Default::default())?;
  // print!("First element of first tuple: {:?}", training_data[0].0[0]);
  Ok(())
}
