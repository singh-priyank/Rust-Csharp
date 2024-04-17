// mnist_loader
// ~~~~~~~~~~~~

// A library to load the MNIST image data.  For details of the data
// structures that are returned, see the doc strings for ``load_data``
// and ``load_data_wrapper``.  In practice, ``load_data_wrapper`` is the
// function usually called by our neural network code.


use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
  let mut file = File::open("C:\\Users\\PriyankSingh\\Neural-Networks-and-Deep-Learning\\data\\mnist.pkl.gz")?;
  let mut data = Vec::new();
  file.read_to_end(&mut data)?;

  let deserialized_data = serde_pickle::from_reader(&mut &data[..])?;

  OK(());
}