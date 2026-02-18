use core::time;
use discord_presence::{Client, Event};
use regex::Regex;
use std::{
    process::{Command, ExitStatus},
    thread::sleep,
};

fn fetch_running(name: &str) -> Result<(ExitStatus, String, String), String> {
    let _p_command = Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .output()
        .expect("Failed process command");

    let file_command = Command::new("pgrep")
        .arg("-a")
        .arg(name)
        .output()
        .expect("Failed file command");

    let file_command_output = String::from_utf8(file_command.stdout)
        .map_err(|_| "Output was not valid UTF-8".to_string())?;

    let re = Regex::new(r"^[^ ]* *[^ ]* *").unwrap();

    let file_name = re.replace(&file_command_output, "");
    let file_ext = file_name.split(".").last().unwrap().to_string();
    Ok((file_command.status, file_ext, file_name.to_string()))
}

fn set_rpc(rpc_client: &mut Client, file_name: &str, file_ext: &str) {
    let top_msg = format!("Currently editing: {file_name}");
    let trimmed_ext = file_ext.trim();

    let icon = match trimmed_ext {
        "rs" => "ferris",
        "py" => "python",
        "sh" => "bash",
        _ => "none",
    };

    // println!("{icon}");

    rpc_client
        .set_activity(|act| act.state(top_msg).assets(|ass| ass.small_image(icon)))
        .expect("Failed to set activity. :c");
}

fn main() {
    let mut rpc_client = Client::new(1385047166632071379);

    rpc_client
        .on_event(Event::Ready, |ctx| println!("Ready!"))
        .persist();

    rpc_client.start();

    loop {
        sleep(time::Duration::from_secs(5));
        match fetch_running("helix") {
            Ok((status, file_ext, file_name)) => {
                if status.success() {
                    set_rpc(&mut rpc_client, &file_name, &file_ext);
                }
            }
            Err(e) => eprintln!("Error occured: {e}"),
        }
    }
}
