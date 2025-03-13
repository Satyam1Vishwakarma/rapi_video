pub mod rapi {

    use reqwest::{self, multipart, Body, StatusCode};
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
            title: String,
        ) -> String {
            let z = json!({"public":public,"panoramic":panoramic,"mp4Support":mp4support,"title":title});

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

            let mut videoid = String::new();
            for a in res["videoId"].to_string().chars() {
                if a != '"' {
                    videoid.push(a);
                }
            }
            return videoid;
        }

        ///Upload video to api.video
        /// # Example
        /// video_upload("myfirstvideo","G:\video.mp4")

        pub async fn video_upload(&self, title: String, full_file_path: String) -> String {
            let z = &self.geturl().await;

            let url = format!("{}/videos", z);

            let videoid = match self.production {
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

                let url = format!("{}/videos/{}/source", z, videoid);
                let client = reqwest::Client::new();
                client
                    .post(url)
                    .bearer_auth(&self.token)
                    .multipart(form)
                    .send()
                    .await
                    .unwrap();
            }

            let z = format!("https://embed.api.video/vod/{}", videoid);
            return z;
        }

        pub async fn get_all_video(&self) -> serde_json::Value {
            let z = &self.geturl().await;
            let url = format!("{}/videos", z);
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

        pub async fn del_video(&self, videoid: String) -> StatusCode {
            let url = format!("https://ws.api.video/videos/{}", videoid);
            let client = reqwest::Client::new();
            let res = client
                .delete(url)
                .bearer_auth(&self.token)
                .send()
                .await
                .unwrap();
            return res.status();
        }

        pub async fn thumbnail_upload(
            &self,
            videoid: String,
            full_file_path: String,
        ) -> serde_json::Value {
            let z = &self.geturl().await;

            let url = format!("{}/videos/{}/thumbnail", z, videoid);

            //let f = File::open(full_file_path.clone()).await.unwrap();
            //let file_len = f.metadata().await.unwrap().len();

            let file = File::open(full_file_path).await.unwrap();
            let stream = FramedRead::new(file, BytesCodec::new());
            let file_body = Body::wrap_stream(stream);

            let some_file = multipart::Part::stream(file_body).file_name("foo");

            let form = multipart::Form::new().part("file", some_file);

            let client = reqwest::Client::new();
            let res = client
                .post(url)
                .bearer_auth(&self.token)
                .multipart(form)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            return res;
        }

        pub async fn watermark_upload(&self, full_file_path: String) -> serde_json::Value {
            let z = &self.geturl().await;

            let url = format!("{}/watermarks", z);

            //let f = File::open(full_file_path.clone()).await.unwrap();
            //let file_len = f.metadata().await.unwrap().len();

            let file = File::open(full_file_path).await.unwrap();
            let stream = FramedRead::new(file, BytesCodec::new());
            let file_body = Body::wrap_stream(stream);

            let some_file = multipart::Part::stream(file_body).file_name("foo");

            let form = multipart::Form::new().part("file", some_file);

            let client = reqwest::Client::new();
            let res = client
                .post(url)
                .bearer_auth(&self.token)
                .multipart(form)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            return res;
        }

        pub async fn get_watermark(&self) -> serde_json::Value {
            let z = &self.geturl().await;

            let url = format!("{}/watermarks", z);

            let client = reqwest::Client::new();
            let res = client
                .get(url)
                .bearer_auth(&self.token)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            return res;
        }

        pub async fn watermark_delete(&self, watermarkid: String) -> StatusCode {
            let z = &self.geturl().await;

            let url = format!("{}/watermarks/{}", z, watermarkid);

            //let f = File::open(full_file_path.clone()).await.unwrap();
            //let file_len = f.metadata().await.unwrap().len();

            let client = reqwest::Client::new();
            let res = client
                .delete(url)
                .bearer_auth(&self.token)
                .send()
                .await
                .unwrap();
            return res.status();
        }

        pub async fn get_caption(&self, videoid: String, lang: String) -> serde_json::Value {
            let z = &self.geturl().await;

            let url = format!("{}/videos/{}/captions/{}", z, videoid, lang);

            let client = reqwest::Client::new();
            let res = client
                .get(url)
                .bearer_auth(&self.token)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            return res;
        }

        pub async fn caption_upload(
            &self,
            videoid: String,
            lang: String,
            full_file_path: String,
        ) -> serde_json::Value {
            let z = &self.geturl().await;

            let url = format!("{}/videos/{}/captions/{}", z, videoid, lang);

            //let f = File::open(full_file_path.clone()).await.unwrap();
            //let file_len = f.metadata().await.unwrap().len();

            let file = File::open(full_file_path).await.unwrap();
            let stream = FramedRead::new(file, BytesCodec::new());
            let file_body = Body::wrap_stream(stream);

            let some_file = multipart::Part::stream(file_body).file_name("foo");

            let form = multipart::Form::new().part("file", some_file);

            let client = reqwest::Client::new();
            let res = client
                .post(url)
                .bearer_auth(&self.token)
                .multipart(form)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            return res;
        }

        pub async fn caption_delete(&self, videoid: String, lang: String) -> StatusCode {
            let z = &self.geturl().await;

            let url = format!("{}/videos/{}/captions/{}", z, videoid, lang);

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

pub mod rapi_sync {

    use reqwest::blocking::multipart;
    use reqwest::StatusCode;
    use serde_json::{self, json};

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
        fn geturl(&self) -> String {
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

        fn video_object(
            &self,
            url: String,
            public: bool,
            panoramic: bool,
            mp4support: bool,
            title: String,
        ) -> String {
            let z = json!({"public":public,"panoramic":panoramic,"mp4Support":mp4support,"title":title});

            let client = reqwest::blocking::Client::new();
            let res: serde_json::Value = client
                .post(url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .json(&z)
                .send()
                .unwrap()
                .json()
                .unwrap();

            let mut videoid = String::new();
            for a in res["videoId"].to_string().chars() {
                if a != '"' {
                    videoid.push(a);
                }
            }
            return videoid;
        }

        ///Upload video to api.video
        /// # Example
        /// video_upload("myfirstvideo","G:\video.mp4")

        pub fn video_upload(&self, title: String, full_file_path: String) -> String {
            let z = &self.geturl();

            let url = format!("{}/videos", z);

            let videoid = match self.production {
                true => self.video_object(url, true, false, true, title),
                false => self.video_object(url, true, false, true, title),
            };

            let metadata = std::fs::metadata(full_file_path.clone()).unwrap();
            let file_len = metadata.len();

            if file_len < 209715200 {
                let form = multipart::Form::new().file("file", full_file_path).unwrap();

                let url = format!("{}/videos/{}/source", z, videoid);
                let client = reqwest::blocking::Client::new();
                client
                    .post(url)
                    .bearer_auth(&self.token)
                    .multipart(form)
                    .send()
                    .unwrap();
            }

            let z = format!("https://embed.api.video/vod/{}", videoid);
            return z;
        }

        pub fn get_all_video(&self) -> serde_json::Value {
            let z = &self.geturl();
            let url = format!("{}/videos", z);
            let client = reqwest::blocking::Client::new();
            let res: serde_json::Value = client
                .get(url)
                .bearer_auth(&self.token)
                .header("accept", "application/json")
                .send()
                .unwrap()
                .json()
                .unwrap();

            return res;
        }

        pub fn del_video(&self, videoid: String) -> StatusCode {
            let url = format!("https://ws.api.video/videos/{}", videoid);
            let client = reqwest::blocking::Client::new();
            let res = client.delete(url).bearer_auth(&self.token).send().unwrap();
            return res.status();
        }

        pub fn thumbnail_upload(
            &self,
            videoid: String,
            full_file_path: String,
        ) -> serde_json::Value {
            let z = &self.geturl();

            let url = format!("{}/videos/{}/thumbnail", z, videoid);

            let form = multipart::Form::new().file("file", full_file_path).unwrap();

            let client = reqwest::blocking::Client::new();
            let res = client
                .post(url)
                .bearer_auth(&self.token)
                .multipart(form)
                .send()
                .unwrap()
                .json()
                .unwrap();
            return res;
        }

        pub fn watermark_upload(&self, full_file_path: String) -> serde_json::Value {
            let z = &self.geturl();

            let url = format!("{}/watermarks", z);

            let form = multipart::Form::new().file("file", full_file_path).unwrap();

            let client = reqwest::blocking::Client::new();
            let res = client
                .post(url)
                .bearer_auth(&self.token)
                .multipart(form)
                .send()
                .unwrap()
                .json()
                .unwrap();
            return res;
        }

        pub fn get_watermark(&self) -> serde_json::Value {
            let z = &self.geturl();

            let url = format!("{}/watermarks", z);

            let client = reqwest::blocking::Client::new();
            let res = client
                .get(url)
                .bearer_auth(&self.token)
                .send()
                .unwrap()
                .json()
                .unwrap();
            return res;
        }

        pub fn watermark_delete(&self, watermarkid: String) -> StatusCode {
            let z = &self.geturl();

            let url = format!("{}/watermarks/{}", z, watermarkid);

            let client = reqwest::blocking::Client::new();
            let res = client.delete(url).bearer_auth(&self.token).send().unwrap();
            return res.status();
        }

        pub fn get_caption(&self, videoid: String, lang: String) -> serde_json::Value {
            let z = &self.geturl();

            let url = format!("{}/videos/{}/captions/{}", z, videoid, lang);

            let client = reqwest::blocking::Client::new();
            let res = client
                .get(url)
                .bearer_auth(&self.token)
                .send()
                .unwrap()
                .json()
                .unwrap();
            return res;
        }

        pub fn caption_upload(
            &self,
            videoid: String,
            lang: String,
            full_file_path: String,
        ) -> serde_json::Value {
            let z = &self.geturl();

            let url = format!("{}/videos/{}/captions/{}", z, videoid, lang);

            let form = multipart::Form::new().file("file", full_file_path).unwrap();

            let client = reqwest::blocking::Client::new();
            let res = client
                .post(url)
                .bearer_auth(&self.token)
                .multipart(form)
                .send()
                .unwrap()
                .json()
                .unwrap();
            return res;
        }

        pub fn caption_delete(&self, videoid: String, lang: String) -> StatusCode {
            let z = &self.geturl();

            let url = format!("{}/videos/{}/captions/{}", z, videoid, lang);

            let client = reqwest::blocking::Client::new();
            let res = client.delete(url).bearer_auth(&self.token).send().unwrap();
            return res.status();
        }
    }
}
