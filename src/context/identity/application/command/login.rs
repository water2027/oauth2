use super::super::super::value_object::{Email, RawPassword};

pub struct LoginCommand {
    pub email: Email,
    pub password: RawPassword,
}
