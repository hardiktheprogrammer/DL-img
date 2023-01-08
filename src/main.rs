use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;
error_chain! {

    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);


    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://docs.rs/rust-logo-20200726-1.47.0-nightly-6c8927b0c.png";
    let response = reqwest::get(target).await?; // pass response to reponse

    
    let mut dest = {
        // mutatable destination files
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin"); // name of file to download

        println!("file to download:'{}'", fname);

        let fname = tmp_dir.path().join(fname); // join path to file name
        println!("will we located under: '{:?}'", fname); //
        File::create(fname)? // create file
    };

    let content = response.text().await?; //
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
    // println!("content.as_b")
}
