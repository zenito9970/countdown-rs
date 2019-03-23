use regex::Regex;
use rustbox::{self, Color, Event, InitOptions, Key, RustBox};
use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::time;

mod fonts;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        let program: String = env::args().next().unwrap();
        eprintln!("Usage:");
        eprintln!("  {} 25s", program);
        eprintln!("  {} 1m50s", program);
        eprintln!("  {} 2h45m50s", program);
        exit(2);
    }

    let deadline = parse_duration(&args[0]).unwrap();
    let start = time::Instant::now();

    let mut exit_code = 0;

    if let Ok(rb) = RustBox::init(InitOptions::default()) {
        let table = fonts::symbol_table();
        let frame_millis = time::Duration::from_millis(16);

        loop {
            let elapsed = start.elapsed();
            if deadline < elapsed {
                exit_code = 0;
                break;
            }
            let remain = deadline - elapsed;

            draw(&rb, remain.as_secs(), &table);

            if let Event::KeyEvent(key) = rb.peek_event(frame_millis, false).unwrap() {
                if key == Key::Esc || key == Key::Ctrl('c') {
                    exit_code = 1;
                    break;
                }
            }
        }
    }

    exit(exit_code);
}

fn parse_duration(duration: &str) -> Result<time::Duration, regex::Error> {
    let re = Regex::new(r"((?P<hour>\d+)h)?((?P<minute>\d+)m)?((?P<second>\d+)s)?")?;
    let caps = re.captures(duration).unwrap();
    let h: u64 = caps.name("hour").map_or(0, |m| m.as_str().parse().unwrap());
    let m: u64 = caps.name("minute").map_or(0, |m| m.as_str().parse().unwrap());
    let s: u64 = caps.name("second").map_or(0, |m| m.as_str().parse().unwrap());
    Ok(time::Duration::new(3600 * h + 60 * m + s, 0))
}

fn draw(rb: &RustBox, remain: u64, table: &HashMap<char, ([&str; 6], usize)>) {
    let fmt = remain_to_fmt(remain);
    let symbols = fmt.chars().map(|c| table[&c]);

    let w_sum = symbols.clone().map(|(_, w)| w).fold(0, |sum, w| sum + w);
    let start_x = rb.width() / 2 - w_sum / 2;
    let start_y = rb.height() / 2 - 3;

    rb.clear();

    let mut x = start_x;
    for (symbol, w) in symbols {
        echo(rb, &symbol, x, start_y);
        x += w;
    }

    rb.present();
}

fn echo(rb: &RustBox, symbol: &[&str; 6], start_x: usize, start_y: usize) {
    let mut y = start_y;
    for line in symbol {
        rb.print(start_x, y, rustbox::RB_NORMAL, Color::Default, Color::Default, line);
        y += 1;
    }
}

fn remain_to_fmt(remain: u64) -> String {
    let (hours, minutes, seconds) = (remain / 3600, (remain % 3600) / 60, remain % 60);
    if hours == 0 {
        format!("{:02}:{:02}", minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
