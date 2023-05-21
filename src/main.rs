use api_video::rapi;

#[tokio::main]
async fn main(){
    let a=rapi::ApiVideo{production:false,token:"GymsBb4Vf2o7C8SCbCUiRBd8125Xv9wP5M8CK9y4hXh".to_string()};
    a.video_upload("title".to_string(), "G:\\Rust\\python\\fo.mp4".to_string()).await;
}