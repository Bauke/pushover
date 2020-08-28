use clap::{
  crate_authors, crate_description, crate_name, crate_version, App, Arg,
};
use pushover_api::{Message, MessagePriority};

/// CLI subcommands.
mod subcommands;

/// The main function.
fn main() {
  let cli = App::new(crate_name!())
    .about(crate_description!())
    .author(crate_authors!())
    .version(crate_version!())
    .args(&[Arg::with_name("verbose")
      .global(true)
      .long("verbose")
      .help("Output extra information when running.")])
    .subcommand(subcommands::send_message())
    .get_matches();

  let verbose = cli.is_present("verbose");

  if let Some(sub_cli) = cli.subcommand_matches("send-message") {
    let message = sub_cli.value_of("message").map(String::from).unwrap();
    let token = sub_cli.value_of("token").map(String::from).unwrap();
    let user = sub_cli.value_of("user").map(String::from).unwrap();

    let title = sub_cli.value_of("title").map(String::from);
    let url = sub_cli.value_of("url").map(String::from);
    let url_title = sub_cli.value_of("url-title").map(String::from);
    let sound = sub_cli.value_of("sound").map(String::from);

    // Pushover expects devices to be a comma-separated list.
    let device = sub_cli
      .values_of("device")
      .map(|values| values.collect::<Vec<&str>>().join(","));
    let priority = sub_cli.value_of("priority").map(MessagePriority::from);
    let timestamp = sub_cli
      .value_of("timestamp")
      .map(|value| value.parse().expect("Failed to parse timestamp to i64"));

    let response = Message {
      message,
      token,
      user,
      device,
      title,
      url,
      url_title,
      priority,
      sound,
      timestamp,
    }
    .send()
    .expect("Error sending message");

    if verbose {
      println!("MessageResponse: {:#?}", response);
    }
  } else {
    println!("No subcommand used. 🤷");
  }
}
