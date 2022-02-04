use std::{io::stdout, thread::sleep, time::Duration};

use better_term::Color;
use crossterm::{execute, cursor::MoveTo, terminal::{Clear, self}};
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};

fn rnd_char(arr: Vec<char>) -> char {
    if arr.is_empty() {
        return ' ';
    }
    arr.get(thread_rng().gen_range(0..arr.len())).unwrap().clone()
}

fn main() {
    let mut stdout = stdout();
    let (mut last_width, mut last_height): (u16, u16) = (0, 0);
    loop {
        // get the current terminal size for dynamic resizing
        let size_res = crossterm::terminal::size();
        if size_res.is_err() {
            panic!("Failed to get terminal size. Are you using a supported terminal?");
        }
        let (width, height) = size_res.unwrap();
        // handle if the size of the window has changed
        if width != last_width || height != last_height {
            last_width = width;
            last_height = height;
            execute!(stdout, Clear(terminal::ClearType::All)).unwrap();
        }

        // define the normal distribution
        let width_mean = 1.0 * ((width / 2) as f64);
        let height_mean = 1.0 * ((height / 2) as f64);

        let width_std = (width / 10) as f64;
        let height_std = (height / 10) as f64;

        let loc_w_normal = Normal::new(width_mean, width_std).expect("Failed to create new normal");
        let loc_h_normal = Normal::new(height_mean, height_std).expect("Failed to create new normal");

        // Select a random location based on the normal distributions
        let x = loc_w_normal.sample(&mut thread_rng()) as u16;
        let y = loc_h_normal.sample(&mut thread_rng()) as u16;

        // move the cursor to the selected location
        let res = execute!(stdout, MoveTo(x, y));
        if res.is_err() {
            panic!("Failed to move cursor to correct position. Are you using a valid terminal?");
        }

        // generate a color
        let r = thread_rng().gen_range(20..200);
        let g = thread_rng().gen_range(0..20);
        let b = thread_rng().gen_range(0..20);

        let color = Color::RGB(r, g, b);

        // let grayscale = thread_rng().gen_range(0..255);
        // let gs_color = Color::RGB(grayscale, grayscale, grayscale);

        let print_c = rnd_char(vec!['▄', '▀', '█']);
        
        // print the character
        print!("{}{}", color, print_c);

        // sleep until the next character should be drawn
        //sleep(Duration::from_millis(10));
    }
}
