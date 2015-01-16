extern crate libc;
extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::{random, thread_rng, sample};
use rand::distributions::{IndependentSample, Range};
use self::libc::funcs::posix88::unistd::usleep;

use rain::Rain;


static MAP: &'static str = concat!(
    "`1234567890-=~!@#$%^&*()_+",
    "qwertyuiop[]asdfghjkl;'zxcvbnm,./",
    "\\QWERTYUIOP{}|ASDFGHJKL:\"ZXCVBNM<>?");
static BLANK_CHAR: char = ' ';
static ESCAPE_CHAR: char = 'q';
static MIN_TRAIL: isize = 10;
static MAX_TRAIL: isize = 20;
static NEW_RAIN_PROB: f64 = 0.2;
static REFRESH_RATE: isize = 30;

pub struct Matrix {
    width: isize,
    height: isize,
    index: isize,

    rain_list: Vec<Rain>,
    window: WINDOW,
}

impl Drop for Matrix {
    fn drop(&mut self) {
        nodelay(self.window, false);
        cbreak();
        echo();
        endwin();
    }
}

impl Matrix {
    pub fn new() -> Matrix {
        let w = initscr();
        noecho();
        raw();
        nodelay(w, true);
        start_color();
        init_pair(1, COLOR_WHITE, COLOR_BLACK);
        init_pair(2, COLOR_GREEN, COLOR_BLACK);
        Matrix { width: COLS as isize, height: LINES as isize, index: 0, rain_list: vec![], window: w }
    }

    pub fn run(&mut self) {
        'main_loop: loop {
            let input = getch();
            if (input & 0xff) as u8 as char == ESCAPE_CHAR {
                break 'main_loop;
            }
            self.refresh();
            self.draw();
            unsafe {
                usleep((1000000.0 / REFRESH_RATE as f64) as u32);
            }
        };
    }

    fn refresh(&mut self) {
        for i in self.rain_list.as_mut_slice()  {
            i.refresh();
        }
        let height = self.height;
        self.rain_list.retain(|ref rain| rain.y - rain.trail_length < height);

        let mut rng = thread_rng();
        let prob = rand::random::<f64>();
        if prob < NEW_RAIN_PROB {
            let x = Range::new(0, self.width).ind_sample(&mut rng);
            let trail_length = Range::new(MIN_TRAIL, MAX_TRAIL).ind_sample(&mut rng);
            let inv_velocity = Range::new(3, 6).ind_sample(&mut rng);
            let r = Rain::new(x, trail_length, inv_velocity);
            self.rain_list.push(r);
        }

        self.index += 1;
        self.index %= 10;
    }

    fn draw(&self) {
        for rain in self.rain_list.as_slice() {
            if rain.is_drawble() {
                let x = rain.x;
                let y = rain.y;
                let trail_length = rain.trail_length;

                self.set_color(1);
                let mut rng = thread_rng();
                let c = sample(&mut rng, MAP.chars(), trail_length as usize);
                mvaddch(y as i32, x as i32, c[0] as u8 as u64);

                for i in 1..trail_length {
                    self.set_color(2);
                    mvaddch((y - i) as i32, x as i32, c[i as usize] as u8 as u64);
                }

                mvaddch((y - trail_length) as i32, x as i32, BLANK_CHAR as u8 as u64);
            }
        }
    }

    fn set_color(&self, color: i16) {
        if color >= 0 && color < (COLORS as i16) {
            wcolor_set(self.window, color);
        }
    }
}
