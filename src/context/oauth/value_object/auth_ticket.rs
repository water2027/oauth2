pub struct AuthTicket(String);

impl AsRef<str> for AuthTicket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AuthTicket {
    pub fn parse(id: String) -> Self {
        AuthTicket(id)
    }

    pub fn from_trusted(id: String) -> Self {
        AuthTicket(id)
    }
}
