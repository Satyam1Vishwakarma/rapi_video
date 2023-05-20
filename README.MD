
# api-video Rust Client
This is unofficial client for [api.video](https://api.video) written in Rust.

It is the video infrastructure for product builders. Lightning fast video APIs for integrating, scaling, and managing on-demand & low latency live streaming features in your app.

# Installation
```rust
cargo add api-video
```
# Usage
```rust
use api_video::rapi;

#[tokio::main]
async fn main(){
    let api = rapi::ApiVideo{production:false,token:"token".to_string()};
    api.video_upload("myfirstvideo".to_string(), "G:\\foo.mp4".to_string()).await;
   let all_video = api.get_all_video(1, 2).await;

}
```

