//! A Rust library to interact with the Pushover.net API.
//!
//! ## Note
//!
//! This library is woefully incomplete and primarily built on a "want to have"
//! basis. If you find yourself missing something from the Pushover API and
//! want it implemented, please
//! [create an issue](https://github.com/Bauke/pushover/issues) for it
//! (or email me at [me@bauke.xyz](mailto:me@bauke.xyz)).
//!
//! ## Examples
//!
//! To send just a message to a user, you can use the convenience
//! [`send_simple_message`](fn.send_simple_message.html) function.
//!
//! ```rust,no_run
//! use pushover_api::send_simple_message;
//!
//! send_simple_message("application token", "user key", "Message").unwrap();
//! ```
//!
//! To send a more complex message, create a [`Message`](struct.Message.html)
//! and [`send()` it](struct.Message.html#method.send).
//!
//! ```rust,no_run
//! use pushover_api::Message;
//!
//! let message = Message {
//!   token: "application token".to_string(),
//!   user: "user key".to_string(),
//!   message: "Message".to_string(),
//!   title: Some("Title".to_string()),
//!   url: Some("https://example.com".to_string()),
//!   ..Message::default()
//! };
//!
//! let response = message.send().unwrap();
//! dbg!(response);
//! ```

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_repr::*;

lazy_static! {
  /// Reusable Reqwest client to make HTTP requests with.
  pub(crate) static ref REQWEST: Client = Client::builder()
    .user_agent("Rust Pushover API Library")
    .build()
    .unwrap();
}

/// The base URL for the Pushover API.
pub(crate) const PUSHOVER_API: &str = "https://api.pushover.net/1";

/// Convenience function to create a full URL with the API base URL.
pub(crate) fn api_url(path: &str) -> String {
  format!("{}/{}", PUSHOVER_API, path)
}

/// The full message body to send to the Pushover API.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Message {
  /// The application's API token. You can
  /// [register one here](https://pushover.net/apps/build) or
  /// [view your existing ones here](https://pushover.net/apps).
  pub token: String,
  /// The user or group identifier to send the message to.
  pub user: String,
  /// The actual message to send.
  pub message: String,
  /// A comma-separated list of devices to send the message to.
  ///
  /// If any of the devices for the specified user/group is disabled or invalid,
  /// or is set to `None`, the message will be sent to all active devices
  /// for that user/group.
  pub device: Option<String>,
  /// The title for the message, if set to `None` the application's name will
  /// be shown instead.
  pub title: Option<String>,
  /// A supplementary URL to show with the message.
  pub url: Option<String>,
  /// A title to use for the supplementary URL.
  pub url_title: Option<String>,
  /// The priority of the message.
  pub priority: Option<MessagePriority>,
  /// The name of one of the sounds to use, see the
  /// [Pushover documentation](https://pushover.net/api#sounds) for a list of
  /// all sounds.
  pub sound: Option<String>,
  /// A Unix timestamp to use as the date time for the message instead of when
  /// the Pushover API received it.
  pub timestamp: Option<i64>,
}

impl Message {
  /// Send this message to the Pushover API.
  pub fn send(&self) -> Result<MessageResponse> {
    let response = REQWEST
      .post(&api_url("messages.json"))
      .header("content-type", "application/json")
      .body(self.to_json()?)
      .send()?;

    let status = response.status();
    let raw: RawMessageResponse = serde_json::from_str(&response.text()?)?;

    if raw.errors.is_empty() {
      Ok(MessageResponse {
        http_status: status,
        request: raw.request,
        status: raw.status,
      })
    } else {
      Err(anyhow!("{}", raw.errors.join(", ")))
    }
  }

  /// Serializes this message to JSON.
  pub(crate) fn to_json(&self) -> Result<String> {
    serde_json::to_string(self).map_err(Into::into)
  }
}

/// The [message priority](https://pushover.net/api#priority).
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum MessagePriority {
  /// From the Pushover documentation:
  ///
  /// > When the `priority` parameter is specified with a value of `-2`, messages
  /// > will be considered lowest priority and will not generate any notification.
  /// > On iOS, the application badge number will be increased.
  Lowest = -2,
  /// From the Pushover documentation:
  ///
  /// > Messages with a `priority` parameter of `-1` will be considered low priority
  /// > and will not generate any sound or vibration, but will still generate a
  /// > popup/scrolling notification depending on the client operating system.
  /// > Messages delivered during a user's quiet hours are sent as though they
  /// > had a priority of (`-1`).
  Low = -1,
  /// From the Pushover documentation:
  ///
  /// > Messages sent without a `priority` parameter, or sent with the parameter
  /// > set to `0`, will have the default priority. These messages trigger sound,
  /// > vibration, and display an alert according to the user's device settings.
  /// > On iOS, the message will display at the top of the screen or as a modal
  /// > dialog, as well as in the notification center. On Android, the message
  /// > will scroll at the top of the screen and appear in the notification
  /// > center.
  ///
  /// > If a user has quiet hours set and your message is received during those times, your message will be delivered as though it had a priority of `-1`.
  Normal = 0,
  /// From the Pushover documentation:
  ///
  /// > Messages sent with a `priority` of `1` are high priority messages that bypass
  /// > a user's quiet hours. These messages will always play a sound and vibrate
  /// > (if the user's device is configured to) regardless of the delivery time.
  /// > High-priority should only be used when necessary and appropriate.
  ///
  /// > High-priority messages are highlighted in red in the device clients.
  High = 1,
}

impl From<&str> for MessagePriority {
  fn from(input: &str) -> Self {
    match input {
      "-2" | "lowest" => MessagePriority::Lowest,
      "-1" | "low" => MessagePriority::Low,
      "0" | "normal" => MessagePriority::Normal,
      "1" | "high" => MessagePriority::High,
      _ => unreachable!(),
    }
  }
}

/// The response from Pushover and the HTTP status code after a message was
/// successfully sent.
#[derive(Debug)]
pub struct MessageResponse {
  pub http_status: StatusCode,
  pub request: String,
  pub status: i32,
}

/// The response from Pushover after an API call is made, including any errors.
///
/// Only used internally, errors should be returned with `Err`.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct RawMessageResponse {
  #[serde(default)]
  pub errors: Vec<String>,
  pub request: String,
  pub status: i32,
}

/// Convenience function to send a simple message without having to construct
/// the [`Message`](struct.Message.html) yourself.
pub fn send_simple_message(
  token: &str,
  user: &str,
  message: &str,
) -> Result<MessageResponse> {
  Message {
    token: token.to_string(),
    user: user.to_string(),
    message: message.to_string(),
    ..Message::default()
  }
  .send()
}
