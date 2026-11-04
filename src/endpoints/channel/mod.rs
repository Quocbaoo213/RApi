use std::sync::Arc;
use crate::core::base::BaseContext;

#[derive(Clone)]
pub struct ChannelEndpoint {
    pub ctx: Arc<BaseContext>,
}

impl ChannelEndpoint {
    pub fn new(ctx: Arc<BaseContext>) -> Self {
        Self { ctx }
    }
}

pub mod add_admins;
pub mod add_users_to_group;
pub mod approve_member;
pub mod batch_get_group_qr;
pub mod block_users;
pub mod change_group_avatar;
pub mod change_group_avatar_legacy;
pub mod change_group_background;
pub mod change_group_desc;
pub mod change_group_name;
pub mod change_group_owner;
pub mod change_group_setting;
pub mod check_group_invite;
pub mod create_calendar_schedule;
pub mod create_group;
pub mod create_note;
pub mod create_reminder;
pub mod delete_note;
pub mod delete_reminder;
pub mod disperse_group;
pub mod edit_note;
pub mod edit_reminder;
pub mod fetch_group_info;
pub mod generate_new_link;
pub mod get_board_pinned_notices;
pub mod get_board_pinned_topics;
pub mod get_board_topic_detail;
pub mod get_disappearing_config;
pub mod get_group_board_list;
pub mod get_group_info_by_link;
pub mod get_group_info_by_socket;
pub mod get_group_link;
pub mod get_group_qr;
pub mod get_link_info;
pub mod get_pending_members;
pub mod get_recent_active_groups;
pub mod get_reminder;
pub mod get_reminders;
pub mod join_by_id;
pub mod join_by_link;
pub mod join_group;
pub mod join_group_by_link;
pub mod kick_users;
pub mod leave_group;
pub mod mute_group_notifications;
pub mod parse_group_target;
pub mod pin_message;
pub mod preview_group_link;
pub mod query_group_shortlink;
pub mod react_board_topic;
pub mod reject_member;
pub mod remove_admins;
pub mod resolve_group_id_from_link;
pub mod search_public_group;
pub mod set_admin;
pub mod set_disappearing_config;
pub mod set_mute;
pub mod set_nickname;
pub mod toggle_block_group_invite;
pub mod toggle_link;
pub mod transfer_owner;
pub mod unblock_users;
pub mod unpin_message;
pub mod update_group_discovery;
