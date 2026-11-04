use std::sync::Arc;
use crate::core::base::BaseContext;

#[derive(Clone)]
pub struct IdentityEndpoint {
    pub ctx: Arc<BaseContext>,
}

impl IdentityEndpoint {
    pub fn new(ctx: Arc<BaseContext>) -> Self {
        Self { ctx }
    }
}

pub mod accept_friend_request;
pub mod add_friend;
pub mod batch_get_user_presence;
pub mod block_user;
pub mod cancel_friend_request;
pub mod change_account_avatar;
pub mod change_account_avatar_social;
pub mod change_cover;
pub mod change_name;
pub mod change_password_account;
pub mod check2_fa_challenge;
pub mod check_friend_relation;
pub mod check_security_credential;
pub mod check_username;
pub mod delete_friend;
pub mod download_chat_wallpaper;
pub mod fetch_account_info;
pub mod fetch_phone_number;
pub mod fetch_user_info;
pub mod find_nearby_friends;
pub mod get_account_auth_token;
pub mod get_account_deactivate_status;
pub mod get_account_devices;
pub mod get_account_security;
pub mod get_alias;
pub mod get_alt_user_qr;
pub mod get_avatar;
pub mod get_avatar_album;
pub mod get_bio_and_song;
pub mod get_blocked_bot_config;
pub mod get_blocked_bot_list;
pub mod get_business_catalog;
pub mod get_contact_timeline_state;
pub mod get_custom_chat_theme;
pub mod get_discover_friend_list;
pub mod get_ext_profile;
pub mod get_fast_profile;
pub mod get_featured_official_accounts;
pub mod get_feed_priority_friends;
pub mod get_friend_board;
pub mod get_friend_recommendations;
pub mod get_friend_requests;
pub mod get_mini_app_session_token;
pub mod get_mini_app_token;
pub mod get_my_sticker_packages;
pub mod get_oa_info;
pub mod get_online_status;
pub mod get_profile;
pub mod get_profile_extra;
pub mod get_quick_access;
pub mod get_recommended_friends;
pub mod get_relation_status;
pub mod get_security_status;
pub mod get_sticker_categories;
pub mod get_sticker_item_download;
pub mod get_sticker_keyword_suggestions;
pub mod get_sticker_keywords;
pub mod get_sticker_package_detail;
pub mod get_sticker_store_catalog;
pub mod get_suggested_friends;
pub mod get_suggested_songs;
pub mod get_timeline_feed;
pub mod get_user_profile;
pub mod get_user_profile_full;
pub mod get_user_profile_socket;
pub mod get_user_qr;
pub mod pin_chat;
pub mod query_devices;
pub mod reject_friend_request;
pub mod remote_logout_other_devices;
pub mod remove_profile_song;
pub mod report_device_security_audit;
pub mod report_network_qos;
pub mod request_2fa_auth;
pub mod request_change_email;
pub mod request_setup_email;
pub mod resend_email_otp;
pub mod reset_user_qr;
pub mod revoke_device_session;
pub mod search_user_by_phone;
pub mod search_zing_songs;
pub mod send_friend_request;
pub mod send_mini_app_tracking;
pub mod send_oa_menu_action;
pub mod send_tracking_events;
pub mod set_alias;
pub mod set_profile_song;
pub mod set_profile_song_by_search;
pub mod setup_email;
pub mod subscribe_oa;
pub mod subscribe_presence;
pub mod suggest_usernames;
pub mod sync_contact_alias;
pub mod sync_multi_device_login;
pub mod sync_phonebook;
pub mod toggle_group_invite_block;
pub mod unblock_user;
pub mod unfriend_user;
pub mod unpin_chat;
pub mod unsubscribe_presence;
pub mod update_bio;
pub mod update_email;
pub mod update_privacy;
pub mod update_privacy_whitelist;
pub mod verify_account_kyc;
pub mod verify_email_otp;
pub mod verify_password;
