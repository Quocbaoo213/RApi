use std::sync::Arc;
use crate::core::base::BaseContext;

#[derive(Clone)]
pub struct TransmissionEndpoint {
    pub ctx: Arc<BaseContext>,
}

impl TransmissionEndpoint {
    pub fn new(ctx: Arc<BaseContext>) -> Self {
        Self { ctx }
    }
}

pub mod audit_revoke_message;
pub mod cancel_call;
pub mod change_group_background;
pub mod clear_chat_history;
pub mod confirm_disappearing_expire;
pub mod control_group_call_media;
pub mod create_poll;
pub mod create_reminder;
pub mod delete_conversation;
pub mod delete_message;
pub mod delete_reminder;
pub mod delete_server_message_log;
pub mod forward_message;
pub mod get_chat_history;
pub mod get_chat_media_history;
pub mod get_direct_chat_board;
pub mod get_disappearing_messages_config;
pub mod get_disappearing_ttl_status;
pub mod get_featured_official_accounts;
pub mod get_last_sync_message;
pub mod get_live_location_config;
pub mod get_message_reactions;
pub mod get_nearby_places;
pub mod get_pinned_messages;
pub mod get_poll_detail;
pub mod get_reactions_summary;
pub mod get_recent_conversations;
pub mod get_reminders;
pub mod get_shared_files;
pub mod get_shared_links;
pub mod get_shared_media;
pub mod get_thread_suggestion;
pub mod hangup_call;
pub mod heartbeat_group_call;
pub mod heartbeat_live_location;
pub mod invite_group_call;
pub mod join_group_call;
pub mod leave_group_call;
pub mod lock_poll;
pub mod mark_as_delivered;
pub mod mark_as_read;
pub mod mark_as_unread;
pub mod mute_conversation;
pub mod pin_conversation;
pub mod pin_message;
pub mod ping_gps_location;
pub mod query_live_location;
pub mod query_nearby_poi;
pub mod reject_call;
pub mod reply_message;
pub mod resolve_link_preview;
pub mod ring_call;
pub mod search_location;
pub mod search_messages;
pub mod search_place_autocomplete;
pub mod send_batch_reactions;
pub mod send_business_card;
pub mod send_card_bank;
pub mod send_contact;
pub mod send_custom_sticker;
pub mod send_direct_message;
pub mod send_direct_text_msg_1238;
pub mod send_file;
pub mod send_group_delete_variant3;
pub mod send_group_event_sub57;
pub mod send_group_live_stream;
pub mod send_group_message;
pub mod send_group_video_message;
pub mod send_image;
pub mod send_link;
pub mod send_location;
pub mod send_message;
pub mod send_message_sync;
pub mod send_multi_reaction;
pub mod send_product_catalog;
pub mod send_reaction;
pub mod send_split_bill;
pub mod send_sticker;
pub mod send_video;
pub mod send_voice;
pub mod send_voip_feedback_rating;
pub mod set_disappearing_message_ttl;
pub mod set_disappearing_messages;
pub mod set_typing;
pub mod start_call;
pub mod start_group_call;
pub mod start_live_location;
pub mod stop_live_location;
pub mod sync_chat_background;
pub mod sync_chat_keywords;
pub mod sync_offline_messages;
pub mod tag_all;
pub mod undo_message;
pub mod unmute_conversation;
pub mod unpin_conversation;
pub mod unpin_message;
pub mod unreact_message;
pub mod update_live_location;
pub mod vote_poll;
