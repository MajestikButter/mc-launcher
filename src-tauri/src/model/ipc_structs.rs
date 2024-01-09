use serde::Serialize;

#[derive(Serialize)]
pub struct ProfileInfo {
    pub game: String,
    pub name: String,
    pub icon: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct GameProfilesInfo {
    pub game: String,
    pub profiles: Vec<ProfileInfo>,
    pub selected: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct LimitedGameInfo {
    pub name: String,
    pub icon: String,
    pub background: String,
    pub versionType: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct VersionInfo {
    pub name: String,
    pub versionType: String,
    pub installed: bool,
}
impl VersionInfo {
    pub fn new(version_type: String, name: String, installed: bool) -> Self {
        Self { versionType: version_type, name: name, installed: installed }
    }
}
