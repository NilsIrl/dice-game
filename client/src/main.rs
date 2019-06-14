extern crate pancurses;

use pancurses::Input;
use pancurses::Window;

use pancurses::A_REVERSE;

mod user;

struct Status {
    window: Window,
    logged_in: bool,
}

impl Status {
    fn new(stdscr: &Window, logged_in: bool) -> Status {
        const STATUS_LENGTH: i32 = 50;
        let mut status = Status {
            window: stdscr
                .subwin(3, STATUS_LENGTH, 0, stdscr.get_max_x() - STATUS_LENGTH)
                .unwrap(),
            logged_in: logged_in,
        };
        status.set_status(logged_in);
        status
    }
    fn set_status(&mut self, logged_in: bool) {
        self.logged_in = logged_in;
        self.window.clear();
        self.window.draw_box(0, 0);
        self.window.mvaddstr(
            1,
            1,
            if self.logged_in {
                "logged_in: username"
            } else {
                "logged out"
            },
        );
        self.window.refresh();
    }
    fn get_status(&self) -> bool {
        self.logged_in
    }
}

fn main() {
    let stdscr = pancurses::initscr();
    pancurses::curs_set(0);
    pancurses::noecho();
    let _status_bar = Status::new(&stdscr, false);
    let choices = ["Sign up", "Sign in", "Leaderboard", "About", "Quit"];
    match main_menu(&stdscr, &choices) {
        0 => signup(&stdscr),
        1 => signin(),
        2 => leaderboard(),
        3 => about(),
        _ => (),
    }
    pancurses::endwin();
}

fn leaderboard() {}

fn signup(stdscr: &Window) {
    const MENU_WIDTH: i32 = 20;
    const MENU_LENGTH: i32 = 40;
    let menu = pancurses::newwin(
        MENU_WIDTH,
        MENU_LENGTH,
        (stdscr.get_max_y() - MENU_WIDTH) / 2,
        (stdscr.get_max_x() - MENU_LENGTH) / 2,
    );
    menu.draw_box(0, 0);
    const FIELD_LENGTH: i32 = 30;
    let username = menu
        .subwin(
            3,
            FIELD_LENGTH,
            menu.get_beg_y() + 6,
            menu.get_beg_x() + (MENU_LENGTH - FIELD_LENGTH) / 2,
        )
        .unwrap();
    let password = menu
        .subwin(
            3,
            FIELD_LENGTH,
            menu.get_beg_y() + 12,
            menu.get_beg_x() + (MENU_LENGTH - FIELD_LENGTH) / 2,
        )
        .unwrap();
    username.draw_box(0, 0);
    password.draw_box(0, 0);
    menu.addstr("Create an account:");
    menu.mvaddstr(3, 4, "Press enter when you're done.");
    username.keypad(true);
    password.keypad(true);
    username.mv(1, 1);
    password.mv(1, 1);
    password.noutrefresh();
    menu.noutrefresh();
    username.noutrefresh();
    pancurses::doupdate();
    let mut userdetail = user::User::new();
    let mut username_selected = true;
    loop {
        if username_selected {
            match username.getch().unwrap() {
                Input::Character('\n') => {}
                Input::KeyBackspace => {
                    userdetail.username.pop();
                    username.erase();
                    username.draw_box(0, 0);
                }
                Input::Character('\t') => {
                    username_selected = !username_selected;
                    if user::User::user_exists(&userdetail.username) {
                        menu.mvaddstr(9, (40 - 30) / 2, "Username unavailable");
                    } else {
                        menu.mvaddstr(9, (40 - 30) / 2, "Username available  ");
                    }
                }
                Input::Character(character) => {
                    userdetail.username.push(character);
                }
                _ => (),
            }
        } else {
            match password.getch().unwrap() {
                Input::Character('\n') => match userdetail.register() {
                    Ok(_) => (), // TODO
                    Err(message) => (),
                },
                Input::KeyBackspace => {
                    userdetail.password.pop();
                    password.erase();
                    password.draw_box(0, 0);
                }
                Input::Character('\t') => username_selected = !username_selected,
                Input::Character(character) => {
                    userdetail.password.push(character);
                }
                _ => (),
            }
        }
        username.mvaddstr(1, 1, &userdetail.username);
        password.mvaddstr(1, 1, &userdetail.password);
        if username_selected {
            username.noutrefresh();
        } else {
            password.noutrefresh();
        }
        pancurses::doupdate();
    }
}

fn signin() {}

fn about() {}

fn main_menu(stdscr: &Window, choices: &[&str]) -> usize {
    const LENGTH: i32 = 40;
    const WIDTH: i32 = 20;
    let menu = stdscr
        .subwin(
            WIDTH,
            LENGTH,
            (stdscr.get_max_y() - WIDTH) / 2,
            (stdscr.get_max_x() - LENGTH) / 2,
        )
        .unwrap();
    menu.draw_box(0, 0);
    menu.keypad(true);
    menu.mvaddstr(0, 3, "Main menu");
    let mut chosen = 0;
    loop {
        for y in 0..choices.len() {
            if y == chosen {
                menu.attron(A_REVERSE);
            }
            menu.mvaddstr(y as i32 + 1, 1, choices[y]);
            if y == chosen {
                menu.attroff(A_REVERSE);
            }
        }
        menu.refresh();
        match menu.getch().unwrap() {
            Input::KeyUp | Input::Character('k') => {
                if chosen != 0 {
                    chosen -= 1;
                }
            }
            Input::KeyDown | Input::Character('j') => {
                chosen += 1;
                if chosen >= choices.len() {
                    chosen = choices.len() - 1;
                }
            }
            Input::Character('\n') | Input::Character('l') => {
                menu.delwin();
                return chosen;
            }
            _ => (),
        }
        menu.refresh();
    }
}
