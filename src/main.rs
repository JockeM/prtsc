use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer};
use reqwest::Client;
use std::convert::TryInto;

fn get_image_buffer(clipboard: &mut Clipboard) -> Option<Vec<u8>> {
    let image = match clipboard.get_image() {
        Ok(img) => img,
        Err(_) => {
            return None;
        }
    };

    eprintln!("getting {}×{} image", image.width, image.height);

    let image = DynamicImage::ImageRgba8(ImageBuffer::from_raw(
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.bytes.into_owned(),
    )?);

    let mut png: Vec<u8> = vec![];
    image
        .write_to(&mut png, image::ImageOutputFormat::Png)
        .unwrap();

    Some(png)
}

async fn upload_image_buffer(buffer: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let img_string = format!("\"{}\"", base64::encode(&buffer));
    let params = [("image", img_string)];

    let client_id = "123123";
    let client = Client::new();
    let response = client
        .post("https://api.imgur.com/3/image")
        .form(&params)
        .header("Authorization", format!("Client-ID {}", client_id))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    match response["data"]["link"].as_str() {
        Some(str) => Ok(str.to_string()),
        None => panic!("failed to fetch link from json"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new().unwrap();
    let buffer = match get_image_buffer(&mut clipboard) {
        Some(buffer) => buffer,
        None => return Ok(()),
    };

    let link = upload_image_buffer(buffer).await?;

    clipboard.set_text(link)?;

    Ok(())
}
