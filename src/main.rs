#[macro_use]
extern crate lazy_static;

mod s3;

use tokio::io::AsyncReadExt;
use std::error::Error;
use async_ftp::FtpStream;
use tokio::runtime::Builder;

use s3::save_to_s3;

async fn async_main() -> Result<(), Box<dyn Error>> {
    // Create a connection to an FTP server and authenticate to it.
    let mut ftp_stream = FtpStream::connect("localhost:2100").await?;
    let _ = ftp_stream.login("files", "files").await?;

    let entries = ftp_stream.nlst(None).await?;

    for entry in entries.iter() {
        match ftp_stream.get(&entry).await {
            Ok(streamed_entry) => {
                match save_to_s3(entry, streamed_entry).await {
                    Ok(_) => {
                        // feat flag option to remove file from ftp source after save on s3 successfully
                        println!("entry {} is saved to s3", entry);
                    },
                    _ => { eprintln!("entry {} could not be saved to s3", entry) }
                }
            },
            _ => { eprintln!("could not get {}", entry); }
        }
    }

    // Terminate the connection to the server.
    let _ = ftp_stream.quit();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
