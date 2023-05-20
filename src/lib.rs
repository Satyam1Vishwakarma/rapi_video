
pub mod rapi{

    use reqwest::{self, multipart};
    use reqwest::{Body, StatusCode};
    use serde_json::{self, json};
    use tokio::fs::File;
    use tokio_util::codec::{BytesCodec, FramedRead};
    
    #[warn(dead_code)]
    pub struct ApiVideo {
        pub production: bool,
        pub token: String,
    }
    
    #[warn(dead_code)]
    struct Uri {
        pro: String,
        sand: String,
    }
    
    #[warn(dead_code)]
    impl ApiVideo {
        async fn geturl(&self) -> String {
            let z = Uri {
                pro: "https://ws.api.video".to_string(),
                sand: "https://sandbox.api.video".to_string(),
            };
    
            let z = match self.production {
                true => z.pro,
                false => z.sand,
            };
            return z;
        }
    
        async fn video_object(
            &self,
            url: String,
            public: bool,
            panoramic: bool,
            mp4support: bool,
            _title: String,
        ) -> String {
            let z =
                json!({"public":public,"panoramic":panoramic,"mp4Support":mp4support,"title":"title"});
    
            let client = reqwest::Client::new();
            let res: serde_json::Value = client
                .post(url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .json(&z)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
    
            let mut vid = String::new();
            for a in res["videoId"].to_string().chars() {
                if a != '"' {
                    vid.push(a);
                }
            }
            return vid;
        }
    
        ///Upload video to api.video
        /// # Example
        /// video_upload("myfirstvideo","G:\video.mp4")
    
        async fn video_upload(&self, title: String, full_file_path: String) -> String {
            let z = &self.geturl().await;
    
            let url = format!("{}/videos", z);
    
            let vid = match self.production {
                true => self.video_object(url, true, false, true, title).await,
                false => self.video_object(url, true, false, true, title).await,
            };
    
            let f = File::open(full_file_path.clone()).await.unwrap();
            let file_len = f.metadata().await.unwrap().len();
    
            if file_len < 209715200 {
                let file = File::open(full_file_path).await.unwrap();
                let stream = FramedRead::new(file, BytesCodec::new());
                let file_body = Body::wrap_stream(stream);
    
                let some_file = multipart::Part::stream(file_body).file_name("foo");
    
                let form = multipart::Form::new().part("file", some_file);
    
                let url = format!("{}/videos/{}/source", z, vid);
                let client = reqwest::Client::new();
                client
                    .post(url)
                    .bearer_auth(&self.token)
                    .multipart(form)
                    .send()
                    .await
                    .unwrap();
            }
    
            let z = format!("https://embed.api.video/vod/{}", vid);
            return z;
        }
    
        async fn all_video(&self, page_num: u8, page_size: u8) -> serde_json::Value {
            let z = &self.geturl().await;
            let url = format!(
                "{}/videos?currentPage={}&pageSize={}'",
                z, page_num, page_size
            );
            let client = reqwest::Client::new();
            let res: serde_json::Value = client
                .get(url)
                .bearer_auth(&self.token)
                .header("accept", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
    
            return res;
        }
    
        async fn del_video(&self, vid: String) -> StatusCode {
            let url = format!("https://ws.api.video/videos/{}", vid);
            let client = reqwest::Client::new();
            let res = client
                .delete(url)
                .bearer_auth(&self.token)
                .send()
                .await
                .unwrap();
            return res.status();
        }
    }

}