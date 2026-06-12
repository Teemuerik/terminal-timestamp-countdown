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

fn on_second(target_timestamp: i64) {
    if let Some((w, h)) = term_size::dimensions() {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let timestamp = now.as_secs() as i64;

        draw_countdown(timestamp, target_timestamp, w, h);
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

        on_second(target_timestamp);
    }
}

fn draw_countdown(timestamp: i64, target_timestamp: i64, width: usize, height: usize) {
    // Clear the screen before drawing the next time.
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::Purge),
        cursor::MoveTo(0, 0)
    )
    .expect("Could not clear terminal and move cursor.");
    // Draw the next time.

    const ART_HEIGHT: usize = 6;
    let vertical_spaces = (height - ART_HEIGHT) / 2;
    // Print newlines to center the countdown vertically.
    let newlines = "\n".repeat(vertical_spaces);
    print!("{newlines}");

    // Also center the countdown horizontally.
    const CHAR_WIDTH: usize = 8;
    let countdown_string = (target_timestamp - timestamp).to_string();
    let art_length = countdown_string.len() * CHAR_WIDTH;
    let leading_spaces = (width - art_length) / 2;

    match to_art(countdown_string, "default", leading_spaces, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }
}
