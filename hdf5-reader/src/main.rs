use aws_sdk_s3;
use std::path::Path;
use ndarray::s;
use ndarray::ArrayBase;
use ndarray::OwnedRepr;
use ndarray::SliceInfo;
use ndarray::SliceInfoElem;
use ndarray::Dim;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    // Specify S3 bucket and object key
    let bucket = "nasa-veda-scratch";
    let key = "ATL08_20240510063615_08032313_006_01.h5";

    // Download the file from S3
    let object = s3_client.get_object().bucket(bucket).key(key).send().await?;

    // Write the file to a temporary file on disk
    let tmp_file_path = "/tmp/downloaded_file.h5";
    let mut file = tokio::fs::File::create(tmp_file_path).await?;
    let mut stream = object.body.into_async_read();
    tokio::io::copy(&mut stream, &mut file).await?;

    // Open the HDF5 file from the temporary location
    let hdf5_file = hdf5::File::open(Path::new(tmp_file_path))?;

    // Access the dataset from the HDF5 file
    let dataset = hdf5_file.dataset("/gt1l/land_segments/canopy/h_canopy_20m")?;
    let data: ArrayBase<OwnedRepr<f64>, _> = dataset.read_slice::<f64, SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 2]>>, Dim<[usize; 2]>>(s![1.., ..])?;
    println!("Data: {:?}", data);

    Ok(())
}
