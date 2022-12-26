use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::Region;
use std::env;
use log::{warn,error,info};

struct AppState {
    bucket: Bucket,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let bucket_name = "crosswords";
    let region_name = "auto".to_string();
    let endpoint = env::var("R2_ENDPOINT").unwrap();
    let region = Region::Custom {
        region: region_name,
        endpoint,
    };
    let credentials = Credentials::default().unwrap();
    let bucket = Bucket::new(bucket_name, region, credentials).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                bucket: bucket.clone(),
            }))
            .service(get_puz)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/puz/{puz_id}")]
async fn get_puz(puz_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let bucket = &data.bucket;
    let result = bucket.get_object(puz_id.into_inner()).await;

    match result {
        Ok(data) => {
            let bytes = Vec::from(data.bytes());
            HttpResponse::Ok().body(bytes)
        },
        Err(err) => {
            warn!("{:#?}", err);
            HttpResponse::NotFound().finish()
        }
    }
}
