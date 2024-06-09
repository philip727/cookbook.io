use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserDetailsPayload {
    pub bio: Option<String>,
    pub display_name: Option<String>,
    pub pronouns: Option<String>,
    pub location: Option<String>,
}
