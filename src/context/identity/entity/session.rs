use crate::context::identity::value_object::user_id::UserID;

const SESSION_MAX_AGE: i64 = 7 * 24 * 3600; // 7 days

pub struct Session {
    pub user_id: UserID,
    pub cookie: String,
    pub expired_at: i64
}

impl Session {
    pub fn new(user_id: UserID, cookie: String) -> Self {
        let expired_at = chrono::Utc::now().timestamp() + SESSION_MAX_AGE;
        Session { user_id, cookie, expired_at }
    }
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        self.expired_at < now
    }
    pub fn refresh(&mut self) -> bool {
        // 如果剩余时间少于总时长的一半，就刷新
        let now = chrono::Utc::now().timestamp();
        let remaining = self.expired_at - now;
        if remaining < SESSION_MAX_AGE / 2 {
            self.expired_at = now + SESSION_MAX_AGE;
            true
        } else {
            false
        }
    }
}