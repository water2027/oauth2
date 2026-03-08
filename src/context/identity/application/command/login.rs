use super::super::super::value_object::{email::Email, raw_password::RawPassword};

pub struct LoginCommand {
    pub email: Email,
    pub password: RawPassword,
}
