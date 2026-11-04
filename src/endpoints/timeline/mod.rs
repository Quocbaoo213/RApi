use std::sync::Arc;
use crate::core::base::BaseContext;

#[derive(Clone)]
pub struct TimelineEndpoint {
    pub ctx: Arc<BaseContext>,
}

impl TimelineEndpoint {
    pub fn new(ctx: Arc<BaseContext>) -> Self {
        Self { ctx }
    }
}

pub mod add_poll_option;
pub mod create_poll;
pub mod delete_feed;
pub mod edit_feed;
pub mod get_feed_detail;
pub mod get_feed_privacy;
pub mod get_feed_relation_status;
pub mod lock_poll;
pub mod post_feed;
pub mod post_feed_photo;
pub mod post_feed_styled;
pub mod post_feed_video;
pub mod post_story;
pub mod query_feed_by_tag;
pub mod query_feed_metrics;
pub mod query_story_reactions;
pub mod query_timeline_metadata;
pub mod react_story;
pub mod react_topic;
pub mod record_mini_app_metrics;
pub mod request_upload_url;
pub mod set_feed_privacy;
pub mod update_feed_status;
pub mod view_poll_detail;
pub mod vote_poll;
