use blake2::{Blake2b512, Digest};

// 写好了就不要改了
// don't change it

pub fn get_passwd_hash(passwd: impl AsRef<str>) -> String {
    let passwd = passwd.as_ref();
    let mut hasher = Blake2b512::new();
    const SALT: &[u8; 23] = b"senyoshu-user-password-";
    hasher.update(SALT);
    hasher.update(passwd.as_bytes());
    let result = hasher.finalize();
    let result = hex::encode(result);
    result
}

pub const USERNAME_CHAR_BOUND: &str = "";

pub fn is_legal_username(username: impl AsRef<str>) -> bool {
    username
        .as_ref()
        .chars()
        .all(|c| (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9'))
}
