use super::super::value_object::AuthTicket;

pub trait IAuthTicketGenerator: Send + Sync {
    fn generate(&self) -> AuthTicket;
}
