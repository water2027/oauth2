use super::super::super::value_object::{Email,RawPassword,Username,ValidationCode};

pub struct RegisterCommand {
    pub username: Username,
    pub email: Email,
    pub password: RawPassword,
    pub password_confirm: RawPassword,
    pub validation_code: ValidationCode,
}
