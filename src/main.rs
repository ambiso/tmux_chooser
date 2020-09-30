use io::stdin;
use std::io;
use std::io::BufRead;
use tmux_interface::session::SESSION_ALL;
use tmux_interface::{AttachSession, NewSession, Sessions, TmuxInterface};

fn main() {
    ctrlc::set_handler(move || {
        println!("\renter nil to drop to normal prompt");
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
                println!("{} - {}", id + 1, name);
            } else {
                println!("{} - [no name]", id + 1);
            }
        }
    }
    println!("Create a new session by entering a name for it");

    loop {
        let mut input = String::new();
        stdin()
            .lock()
            .read_line(&mut input)
            .expect("Could not read line from stdin");

        let input = input.trim();

        if input == "nil" {
            return;
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
                    Ok(_) => {},
                    Err(tmux_interface::error::Error {
                        message,
                        ..
                    }) => {
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
