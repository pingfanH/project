#[derive(Debug)]
pub struct User {
    user_token:String,
    user_id: String,
    user_account: String,
    user_password: String,
    user_name: String,
    user_gender: String,
    user_age: String,
    user_intro: String,
    user_sign_date: String,
    user_music_number: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Debug,Clone)]
pub struct MusicList {
    pub(crate) user: String,
    pub(crate) name: String,
    pub(crate) date: String,
    pub(crate) public: bool,
}