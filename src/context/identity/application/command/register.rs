use super::super::super::value_object::{email::Email, raw_password::RawPassword, username::Username, validation_code::ValidationCode};

pub struct RegisterCommand {
    pub username: Username,
    pub email: Email,
    pub password: RawPassword,
    pub password_confirm: RawPassword,
    pub validation_code: ValidationCode,
}
