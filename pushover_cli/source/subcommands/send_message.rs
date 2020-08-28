//! The `send-message` subcommand definition.

use clap::{App, Arg, SubCommand};

/// The `send-message` subcommand definition.
pub fn send_message<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("send-message")
    .about("Send a message with Pushover")
    .args(&[
      Arg::with_name("token")
        .long("token")
        .short("t")
        .help("The application API token.")
        .takes_value(true)
        .required(true),
      Arg::with_name("user")
        .long("user")
        .short("u")
        .help("The user/group identifier to send the message to.")
        .takes_value(true)
        .required(true),
      Arg::with_name("device")
        .long("device")
        .help("A device to send the message to.")
        .multiple(true)
        .takes_value(true),
      Arg::with_name("title")
        .long("title")
        .help("The title to use for the message.")
        .takes_value(true),
      Arg::with_name("url")
        .long("url")
        .help("A supplementary URL to include with the message.")
        .takes_value(true),
      Arg::with_name("url-title")
        .long("url-title")
        .help("The title to use for the supplementary URL.")
        .requires("url")
        .takes_value(true),
      Arg::with_name("priority")
        .long("priority")
        .help("The message's priority.")
        .takes_value(true)
        .possible_values(&["lowest", "low", "normal", "high"]),
      Arg::with_name("sound")
        .long("sound")
        .help("The sound to play with the notification.")
        .takes_value(true),
      Arg::with_name("timestamp")
        .long("timestamp")
        .help(
          "A Unix timestamp to use for the message, rather than the \
            time the message is received by the Pushover API.",
        )
        .takes_value(true),
      Arg::with_name("message").required(true),
    ])
}
