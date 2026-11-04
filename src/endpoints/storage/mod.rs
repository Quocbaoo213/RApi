use std::sync::Arc;
use crate::core::base::BaseContext;

#[derive(Clone)]
pub struct StorageEndpoint {
    pub ctx: Arc<BaseContext>,
}

impl StorageEndpoint {
    pub fn new(ctx: Arc<BaseContext>) -> Self {
        Self { ctx }
    }
}

pub mod build_get_video_cdn_packet;
pub mod build_upload_request_packet;
pub mod build_upload_request_payload;
pub mod download_chunked;
pub mod drain_and_parse;
pub mod get_uphost;
pub mod get_video_cdn_info;
pub mod request_upload_url;
pub mod upload;
pub mod upload_attachment;
pub mod upload_binary;
pub mod upload_file;
pub mod upload_photo;
pub mod upload_video;
pub mod upload_voice;
pub mod upload_voice_stream;
pub mod upload_zfcloud;
pub mod upload_zpcloud;
pub mod upload_zvc;
