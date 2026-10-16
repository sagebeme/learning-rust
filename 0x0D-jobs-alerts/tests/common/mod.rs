//! A fake sender: records what it was asked to send, and fails on cue. No real messages, ever.
#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::Mutex;

use chrono::NaiveDate;
use jobs_alerts::model::{Alert, AlertSender, Channel, SendError};

#[derive(Default)]
pub struct FakeSender {
    /// What to answer for each send, in order. When it's empty, every send works.
    answers: Mutex<VecDeque<Result<(), SendError>>>,
    pub sent: Mutex<Vec<Alert>>,
}

impl FakeSender {
    pub fn answering(answers: Vec<Result<(), SendError>>) -> FakeSender {
        FakeSender {
            answers: Mutex::new(answers.into()),
            sent: Mutex::new(Vec::new()),
        }
    }

    pub fn attempts(&self) -> usize {
        self.sent.lock().expect("not poisoned").len()
    }
}

impl AlertSender for FakeSender {
    async fn send(&self, alert: &Alert) -> Result<(), SendError> {
        self.sent.lock().expect("not poisoned").push(alert.clone());
        self.answers
            .lock()
            .expect("not poisoned")
            .pop_front()
            .unwrap_or(Ok(()))
    }
}

pub fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

pub fn alert(staff_number: u32, code: &str, channel: Channel) -> Alert {
    Alert {
        staff_number,
        code: code.to_string(),
        valid_until: d(2026, 11, 15),
        days_left: 30,
        channel,
        to: "+254700000000".to_string(),
        message: format!("Your {code} expires in 30 days."),
    }
}
