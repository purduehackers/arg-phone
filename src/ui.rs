use std::sync::mpsc::{Receiver, Sender};

use crate::{config::KNOWN_NUMBERS, hardware};

pub fn ui_entry(web_sender: Sender<(i32, String)>, _web_reciever: Receiver<i32>) {
    let mut hardware = hardware::create();

    hardware.ring(false);
    hardware.enable_dialing(true);

    let mut in_call = false;
    let mut last_hook_state = true;

    let mut last_dialed_number: String = "".to_string();

    loop {
        hardware.update();

        let hook_state = hardware.get_hook_state();

        if last_hook_state != hook_state {
            if hook_state {
                println!("Cleared number");
                hardware.dialed_number.clear();

                let _ = web_sender.send((0, String::new()));

                if in_call {
                    in_call = false;

                    println!("Call Ended.");
                }
            } else {
                let _ = web_sender.send((1, String::new()));
            }
        }

        if !hardware.dialed_number.is_empty() {
            if last_dialed_number != hardware.dialed_number && in_call {
                let _ = web_sender.send((99, hardware.dialed_number.clone()));

                last_dialed_number = hardware.dialed_number.clone();
            }

            let mut contains = false;

            for number in KNOWN_NUMBERS {
                if number == hardware.dialed_number {
                    contains = true;
                }
            }

            if contains && !hook_state {
                in_call = true;

                println!("Call Started");
                let _ = web_sender.send((2, String::new()));

                hardware.dialed_number.clear();
            }
        }


        last_hook_state = hook_state;
    }
}
