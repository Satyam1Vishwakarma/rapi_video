use api_video::rapi;


#[tokio::main]
async fn main(){
    let api = rapi::ApiVideo{production:false,token:"token".to_string()};
    api.video_upload("myfirstvideo".to_string(), "G:\\foo.mp4".to_string()).await;
   let all_video = api.get_all_video(1, 2).await;

}