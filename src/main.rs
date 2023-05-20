use api_video::rapi;


#[tokio::main]
async fn main(){
    let api = rapi::ApiVideo{production:false,token:"token".to_string()};
}