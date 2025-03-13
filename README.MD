# api-video Rust Client

This is unofficial client for [api.video](https://api.video) written in Rust.

It is the video infrastructure for product builders. Lightning fast video APIs for integrating, scaling, and managing on-demand & low latency live streaming features in your app.

# Installation

```rust
cargo add api_video
```

# Usage Asynchrounous

```rust
use api_video::rapi;

#[tokio::main]
async fn main(){
    let api = rapi::ApiVideo{production:false,token:"token".to_string()};
    api.video_upload("myfirstvideo".to_string(), "G:\\foo.mp4".to_string()).await;
   let all_video = api.get_all_video(1, 2).await;

}
```

# Usage Synchrounous

```rust
use api_video::rapi_sync;


fn main(){
    let api = rapi_sync::ApiVideo{production:false,token:"token".to_string()};
    api.video_upload("myfirstvideo".to_string(), "G:\\foo.mp4".to_string());
   let all_video = api.get_all_video(1, 2);

}
```

# Done

VideoUpload<200MiB ✅|
GetAllVideo ✅|
DeleteAVideo ✅|
ThumbnailUpload ✅|
WatermarkUpload ✅|
GetWatermark ✅|
DeleteWatermark ✅|
GetCaption ✅|
UploadCaption ✅|
DeleteCaption ✅|
