use crate::core::session::credential::SessionProfile;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn load_session(&self, path: &str) -> Result<SessionProfile, String> {
        SessionProfile::load_from_file(path)
    }

    pub fn save_session(&self, profile: &SessionProfile, path: &str) -> Result<(), String> {
        profile.save_to_file(path)
    }
}
