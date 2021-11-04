use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer};
use reqwest::Client;
use std::convert::TryInto;
use std::env;

fn get_image_buffer(clipboard: &mut Clipboard) -> Option<Vec<u8>> {
    let image = match clipboard.get_image() {
        Ok(img) => img,
        Err(_) => {
            return None;
        }
    };

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

    let args: Vec<String> = env::args().collect();

    let client_id = args.get(1).expect("You need to provide a imgur client_id");

    let client = Client::new();
    let response = client
        .post("https://api.imgur.com/3/image")
        .form(&params)
        .header("Authorization", format!("Client-ID {}", client_id))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if response["success"].as_bool().unwrap_or(false) {
        let link = response["data"]["link"].as_str().unwrap();

        println!("uploaded: {}", link);

        Ok(link.into())
    } else {
        Err("Failed to upload image".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new().unwrap();
    let buffer = if let Some(buffer) = get_image_buffer(&mut clipboard) {
        buffer
    } else {
        println!("No image found on clipboard");
        return Ok(());
    };

    let link = upload_image_buffer(buffer).await?;

    clipboard.set_text(link)?;

    Ok(())
}
