use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, thread};

use text_to_ascii_art::to_art;

use crossterm::{cursor, execute, terminal};

fn wait_until_next_second() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let nanos = now.subsec_nanos();
    let wait = 1_000_000_000 - nanos;

    thread::sleep(Duration::new(0, wait));
}

fn on_second(target_timestamp: i64, should_center: bool, right_align_length: usize) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = now.as_secs() as i64;

    if !should_center {
        draw_countdown(
            timestamp,
            target_timestamp,
            0,
            0,
            should_center,
            right_align_length,
        );
        return;
    }

    if let Some((w, h)) = term_size::dimensions() {
        draw_countdown(
            timestamp,
            target_timestamp,
            w,
            h,
            should_center,
            right_align_length,
        );
    } else {
        println!("Unable to get term size.")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please pass in the target UNIX timestamp as an argument.");
    }
    let target_str = args.get(1).unwrap();

    let target_timestamp = if target_str.to_lowercase() == "deltarune" {
        1_782_313_200
    } else {
        target_str.parse::<i64>().unwrap()
    };

    let mut should_center = true;
    let mut dms_override = false;
    let mut right_align_length = 0;

    if args.len() > 2 {
        for argument in args[2..].iter() {
            // Take only the name of the argument.
            let mut parts = argument.split("=");
            // Match the name of the argument.
            let arg_name = parts.next();
            match arg_name {
                Some("--no-center") => should_center = false,
                // Render in DMS Desktop Command plugin compatible mode.
                // Does not center, and only renders a single frame, then exits.
                // The plugin handles calling at intervals.
                Some("--dms-desktop-command") => dms_override = true,
                Some("--right-align-length") => {
                    right_align_length = parts
                        .next()
                        .expect("Please pass in the length to --right-align-length=N")
                        .parse::<usize>()
                        .unwrap()
                }
                Some(&_) => {
                    let argname_str = arg_name.unwrap();
                    println!("Did not recognize argument: {argname_str}");
                }
                None => panic!("Found no arguments where there should be."),
            }
        }
    }

    if dms_override {
        on_second(target_timestamp, false, right_align_length);
        return;
    }

    let mut stdout = std::io::stdout();
    execute!(stdout, cursor::Hide).expect("Failed to hide cursor.");

    ctrlc::set_handler(move || {
        // When the user quits with Ctrl-C, show the cursor again.
        let mut stdout = std::io::stdout();
        execute!(stdout, cursor::Show).expect("Could not show cursor.");
        // Then exit the application.
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler.");

    loop {
        wait_until_next_second();

        on_second(target_timestamp, should_center, right_align_length);
    }
}

fn draw_countdown(
    timestamp: i64,
    target_timestamp: i64,
    width: usize,
    height: usize,
    should_center: bool,
    right_align_length: usize,
) {
    // Clear the screen before drawing the next time.
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::Purge),
        cursor::MoveTo(0, 0)
    )
    .expect("Could not clear terminal and move cursor.");
    // Draw the next time.

    if should_center {
        const ART_HEIGHT: usize = 6;
        let vertical_spaces = (height - ART_HEIGHT) / 2;
        // Print newlines to center the countdown vertically.
        let newlines = "\n".repeat(vertical_spaces);
        print!("{newlines}");
    }

    // Also center the countdown horizontally.
    const CHAR_WIDTH: usize = 8;
    let countdown_string = (target_timestamp - timestamp).to_string();
    let art_length = countdown_string.len() * CHAR_WIDTH;
    let computed_right_align_length = if right_align_length == 0 {
        art_length
    } else {
        right_align_length * CHAR_WIDTH
    };
    let leading_spaces = if should_center {
        (width - art_length) / 2 + (computed_right_align_length - art_length)
    } else {
        computed_right_align_length - art_length
    };

    match to_art(countdown_string, "default", leading_spaces, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }
}
