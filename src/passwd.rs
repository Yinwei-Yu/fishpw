use crate::encrypt::util::encrypt_string_with_argon2;
use chrono::Utc;
use uuid::Uuid;
use serde::{self, Serialize};

#[derive(Serialize,Debug)]
pub struct PassWord {
    /// uuid
    pub uuid: String,
    /// user account
    pub account: String,
    /// password after encrypted
    pub encrypted_password: Vec<u8>,
    pub url: Option<String>,
    pub note: Option<String>,
    pub tags: Option<Vec<String>>,
    pub create_at: i64,
    pub update_at: i64,
}

impl PassWord {
    ///create a new password
    pub fn new(
        passwd: &str,
        account: &str,
        url: Option<String>,
        note: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Self {
        PassWord {
            uuid: Uuid::new_v4().to_string(),
            account: account.to_string(),
            encrypted_password: encrypt_string_with_argon2("master_password", passwd),
            url,
            note,
            tags,
            create_at: Utc::now().timestamp(),
            update_at: Utc::now().timestamp(),
        }
    }

    pub fn update_passwd(&mut self, input: &str) {
        self.encrypted_password = encrypt_string_with_argon2("master_password", input);
        self.update();
    }

    pub fn update_note(&mut self, input: &str) {
        self.note = Some(input.to_string());
        self.update();
    }

    pub fn update_tags(&mut self, input: &str) {
        if self.tags.is_none() {
            self.tags = Some(vec![input.to_string()]);
        } else {
            self.tags.as_mut().unwrap().push(input.to_string());
        }
        self.update();
    }

    fn update(&mut self) {
        self.update_at = Utc::now().timestamp();
    }
}
