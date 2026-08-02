mod s3;

use std::error::Error;
use suppaftp::tokio::AsyncRustlsFtpStream;

use s3::save_to_s3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a connection to an FTP server and authenticate to it.
    let mut ftp_stream = AsyncRustlsFtpStream::connect("localhost:2100").await?;

    ftp_stream.login("files", "files").await?;

    let entries = ftp_stream.nlst(None).await?;

    for entry in entries {
        let data_stream = ftp_stream.retr_as_stream(entry.clone()).await?;

        match save_to_s3(&entry.clone(), data_stream).await {
            Ok(_) => {
                // feat flag option to remove file from ftp source after save on s3 successfully
                println!("entry {} is saved to s3", entry);
            }
            _ => {
                eprintln!("entry {} could not be saved to s3", entry)
            }
        }
    }

    // Terminate the connection to the server.
    ftp_stream.quit().await?;

    Ok(())
}
