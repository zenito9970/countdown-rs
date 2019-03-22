use rustbox::{self, Color, Event, InitOptions, Key, RustBox};
use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time;
use regex::Regex;

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
    let rb = RustBox::init(InitOptions::default()).unwrap();
    let rb = Arc::new(rb);

    let (tx, rx) = mpsc::channel();
    {
        let rb = Arc::clone(&rb);
        thread::spawn(move || loop {
            tx.send(rb.poll_event(false).unwrap()).unwrap();
        });
    }

    let table = fonts::symbol_table();

    'mainloop: loop {
        thread::sleep(time::Duration::from_millis(10));

        let rb = Arc::clone(&rb);
        let remain = deadline - start.elapsed();
        draw(rb, remain.as_secs(), &table);

        if remain.as_millis() < 21 {
            exit(0);
        }

        if let Ok(event) = rx.try_recv() {
            if let Event::KeyEvent(key) = event {
                if key == Key::Esc || key == Key::Ctrl('c') {
                    exit(1);
                }
            }
        }
    }
}

fn parse_duration(duration: &str) -> Result<time::Duration, regex::Error> {
    let re = Regex::new(r"((?P<hour>\d+)h)?((?P<minute>\d+)m)?((?P<second>\d+)s)?")?;
    let caps = re.captures(duration).unwrap();
    let h: u64 = caps.name("hour").map_or(0, |m| m.as_str().parse().unwrap());
    let m: u64 = caps.name("minute").map_or(0, |m| m.as_str().parse().unwrap());
    let s: u64 = caps.name("second").map_or(0, |m| m.as_str().parse().unwrap());
    Ok(time::Duration::new(3600 * h + 60 * m + s, 0))
}

fn draw(rb: Arc<RustBox>, remain: u64, table: &HashMap<char, ([&str; 6], usize)>) {
    let fmt = remain_to_fmt(remain);
    let symbols = fmt_to_symbols(fmt, table);

    let w_sum = symbols.iter().map(|(_, w)| *w).fold(0, |sum, w| sum + w);
    let start_x = rb.width() / 2 - w_sum / 2;
    let start_y = rb.height() / 2 - 3;

    rb.clear();

    let mut x = start_x;
    for (symbol, w) in &symbols {
        let rb = Arc::clone(&rb);
        echo(rb, symbol, x, start_y);
        x += w;
    }

    rb.present();
}

fn echo(rb: Arc<RustBox>, symbol: &[&str; 6], start_x: usize, start_y: usize) {
    let (mut x, mut y) = (start_x, start_y);
    for line in symbol {
        for c in line.chars() {
            rb.print_char(x, y, rustbox::RB_NORMAL, Color::White, Color::Default, c);
            x += 1;
        }
        y += 1;
        x = start_x;
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

fn fmt_to_symbols<'a>(
    fmt: String,
    table: &HashMap<char, ([&'a str; 6], usize)>,
) -> Vec<([&'a str; 6], usize)> {
    let mut ret = vec![];
    for c in fmt.chars() {
        ret.push(table[&c]);
    }
    ret
}
