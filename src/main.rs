use io::stdin;
use std::{io, time::SystemTime, time::UNIX_EPOCH};
use std::io::BufRead;
use tmux_interface::session::SESSION_ALL;
use tmux_interface::{AttachSession, NewSession, Sessions, TmuxInterface};

fn main() {
    ctrlc::set_handler(move || {
        println!("\rEnter nil to drop to normal prompt");
    })
    .expect("Error setting Ctrl-C handler");
    let sessions: Vec<_> = Sessions::get(SESSION_ALL)
        .expect("Could not obtain tmux sessions")
        .into_iter()
        .collect();
    if sessions.len() == 0 {
        println!("No existing sessions.")
    } else {
        println!("Choose the terminal to attach:");
        for (id, session) in sessions.iter().enumerate() {
            if let Some(name) = &session.name {
                let attached = if let Some(x) = session.attached {
                    x >= 1
                } else {
                    false
                };
                let attached = 
                    if attached { "(attached)" } else { "" };
                let creation = if let Some(dur) = session.created {
                    let timestamp = dur.as_millis() as u64; // tmux_interface is weird
                    let now = SystemTime::now();
                    let secs = now.duration_since(UNIX_EPOCH).expect("Could not compute current time").as_secs() - timestamp;

                    if secs < 60 {
                        format!("{}s", secs)
                    } else if secs/60 < 60 {
                        format!("{}m", secs/60)
                    } else if secs/(60*60) < 24 {
                        format!("{}h", secs/(60*60))
                    } else {
                        format!("{}d", secs/(60*60*24))
                    }
                } else {
                    "".to_string()
                };
                println!(
                    "{} - {} {} {}",
                    id + 1,
                    name,
                    creation,
                    attached,
                );
            } else {
                println!("{} - [no name]", id + 1);
            }
        }
    }
    println!("\nCreate a new session by entering a name for it:");

    loop {
        let mut input = String::new();
        stdin()
            .lock()
            .read_line(&mut input)
            .expect("Could not read line from stdin");

        let input = input.trim();

        if input == "nil" {
            std::process::exit(1);
        }

        let mut tmux = TmuxInterface::new();

        match input.parse::<usize>() {
            Ok(idx) => {
                if idx == 0 || idx - 1 >= sessions.len() {
                    println!("Invalid index");
                    continue;
                }
                tmux.attach_session(Some(&AttachSession {
                    target_session: Some(&sessions[idx - 1].name.as_ref().unwrap()),
                    ..Default::default()
                }))
                .expect("Could not attach session");
                return;
            }
            Err(_) => {
                let new_session = if input.len() == 0 {
                    NewSession {
                        session_name: None,
                        ..Default::default()
                    }
                } else {
                    NewSession {
                        session_name: Some(input),
                        ..Default::default()
                    }
                };
                match tmux.new_session(Some(&new_session)) {
                    Ok(_) => {}
                    Err(tmux_interface::error::Error { message, .. }) => {
                        if message.starts_with("duplicate session: ") {
                            // Try to attach to named session
                            tmux.attach_session(Some(&AttachSession {
                                target_session: Some(&input),
                                ..Default::default()
                            }))
                            .expect("Could not attach session");
                        } else {
                            panic!("Unexpected error: {}", message);
                        }
                    }
                }
                return;
            }
        }
    }
}
