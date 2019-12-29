use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use crate::data::record::Record;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    overall: f32,
    verified: bool,
    #[serde(rename = "reviewTime")]
    review_time: String,
    #[serde(rename = "reviewerID")]
    reviewer_id: String,
    asin: String,
    style: Option<Style>,
    #[serde(rename = "reviewerName")]
    reviewer_name: Option<String>,
    #[serde(rename = "reviewText")]
    review_text: Option<String>,
    #[serde(rename = "unixReviewTime")]
    unix_review_time: u128,
}

impl Record for Review {}

impl Review {
    pub fn new(overall: f32,
               verified: bool,
               review_time: String,
               reviewer_id: String,
               asin: String,
               style: Option<Style>,
               reviewer_name: Option<String>,
               review_text: Option<String>,
               unix_review_time: u128) -> Self {
        Review {
            overall,
            verified,
            review_time,
            reviewer_id,
            asin,
            style,
            reviewer_name,
            review_text,
            unix_review_time,
        }
    }
}

impl PartialOrd for Review {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.overall != other.overall { return self.overall.partial_cmp(&other.overall); }

        match (self.verified, other.verified) {
            (true, false) => return Some(Ordering::Greater),
            (false, true) => return Some(Ordering::Less),
            _ => ()
        };

        if self.review_time != other.review_time { return self.review_time.partial_cmp(&other.review_time); }

        if self.reviewer_id != other.reviewer_id { return self.reviewer_id.partial_cmp(&other.reviewer_id); }

        if self.asin != other.asin { return self.asin.partial_cmp(&other.asin); }

        match (&self.style, &other.style) {
            (Some(_), None) => return Some(Ordering::Greater),
            (None, Some(_)) => return Some(Ordering::Less),
            (Some(s), Some(o)) => {
                if s != o { return s.partial_cmp(o); }
            }
            _ => ()
        }

        match (&self.reviewer_name, &other.reviewer_name) {
            (Some(_), None) => return Some(Ordering::Greater),
            (None, Some(_)) => return Some(Ordering::Less),
            (Some(s), Some(o)) => {
                if s != o { return s.partial_cmp(o); }
            }
            _ => ()
        }

        match (&self.review_text, &other.review_text) {
            (Some(_), None) => return Some(Ordering::Greater),
            (None, Some(_)) => return Some(Ordering::Less),
            (Some(s), Some(o)) => {
                if s != o { return s.partial_cmp(o); }
            }
            _ => ()
        }

        if self.unix_review_time != other.unix_review_time { return self.unix_review_time.partial_cmp(&other.unix_review_time); }

        Some(Ordering::Equal)
    }
}

impl PartialEq for Review {
    fn eq(&self, other: &Self) -> bool {
        return self.overall == other.overall &&
            self.verified == other.verified &&
            self.review_time == other.review_time &&
            self.reviewer_id == other.reviewer_id &&
            self.asin == other.asin &&
            match (&self.style, &other.style) {
                (None, None) => true,
                (Some(s), Some(o)) => s == o,
                _ => false
            } &&
            match (&self.reviewer_name, &other.reviewer_name) {
                (None, None) => true,
                (Some(s), Some(o)) => s == o,
                _ => false
            } &&
            match (&self.review_text, &other.review_text) {
                (None, None) => true,
                (Some(s), Some(o)) => s == o,
                _ => false
            } &&
            self.unix_review_time == other.unix_review_time
        ;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Style {
    #[serde(rename = "Format:")]
    format: Option<String>
}

impl Style {
    pub fn new(format: Option<String>) -> Self {
        Style { format }
    }
}

impl PartialOrd for Style {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.format, &other.format) {
            (None, None) => Some(Ordering::Equal),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(s), Some(o)) => s.partial_cmp(o)
        }
    }
}

impl PartialEq for Style {
    fn eq(&self, other: &Self) -> bool {
        match (&self.format, &other.format) {
            (Some(s), Some(o)) => {
                s == o
            }
            (None, None) => true,
            _ => false
        }
    }
}