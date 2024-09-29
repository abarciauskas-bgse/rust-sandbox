import boto3
import h5py
import os
import tempfile

# Function to download a file from S3
def download_file_from_s3(bucket_name, key, download_path):
    s3 = boto3.client('s3')
    s3.download_file(bucket_name, key, download_path)
    print(f"Downloaded {key} from {bucket_name} to {download_path}")

# Function to read a dataset from the HDF5 file
def read_hdf5_dataset(file_path, dataset_name):
    with h5py.File(file_path, 'r') as hdf5_file:
        dataset = hdf5_file[dataset_name][:]
        print(f"Dataset {dataset_name}:")
        print(dataset)

# Main execution
if __name__ == "__main__":
    # Specify S3 bucket and object key
    bucket_name = "nasa-veda-scratch"
    key = "ATL08_20240510063615_08032313_006_01.h5"

    # Create a temporary file
    with tempfile.NamedTemporaryFile(delete=False) as temp_file:
        temp_file_path = temp_file.name

    try:
        # Step 1: Download HDF5 file from S3 to temporary file
        download_file_from_s3(bucket_name, key, temp_file_path)

        # Step 2: Read dataset from the HDF5 file
        dataset_name = "/gt1l/land_segments/canopy/h_canopy_20m"  # Replace with the actual dataset name in your HDF5 file
        read_hdf5_dataset(temp_file_path, dataset_name)

    finally:
        # Clean up the temporary file
        os.remove(temp_file_path)
        print(f"Deleted temporary file {temp_file_path}")
