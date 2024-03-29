use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use futures::StreamExt;
use image::ImageFormat;

async fn upload_image(mut payload: web::Payload) -> impl Responder {
    let mut bytes = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        bytes.extend_from_slice(&chunk);
    }

    println!("Received {} bytes.", bytes.len());

    let format = match image::guess_format(&bytes) {
        Ok(format) => format,
        Err(err) => {
            eprintln!("Error: {}", err);
            return HttpResponse::BadRequest().body("Invalid image format.");
        }
    };

    match format {
        ImageFormat::Jpeg | ImageFormat::Png => {
            println!("Image format: {:?}", format);
        }
        _ => return HttpResponse::BadRequest().body("Unsupported image format."),
    }

    let file_extension = match format {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Png => "png",
        _ => unreachable!(),
    };

    let image_name = format!("{}.{}", chrono::Local::now().timestamp(), file_extension);
    std::fs::write(format!("images/{}", image_name), &bytes).unwrap();

    HttpResponse::Ok().body("Image uploaded successfully.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server.");

    HttpServer::new(|| App::new().route("/upload", web::post().to(upload_image)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
