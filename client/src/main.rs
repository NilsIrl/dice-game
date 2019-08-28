
use pancurses::Input;
use pancurses::Window;

use pancurses::A_REVERSE;

mod user;

fn main() {
    let stdscr = pancurses::initscr();
    if pancurses::has_colors() {
        pancurses::start_color();
    }
    pancurses::curs_set(0);
    pancurses::noecho();
    let choices = ["Sign up", "Sign in", "Leaderboard", "About", "Quit"];
    loop {
        match main_menu(&stdscr, &choices) {
            0 => signup(&stdscr),
            1 => signin(),
            2 => leaderboard(&stdscr),
            3 => about(),
            _ => break,
        }
    }
    pancurses::endwin();
}

fn leaderboard(stdscr: &Window) {
    pancurses::curs_set(0);
    const MENU_WIDTH: i32 = 20;
    const MENU_LENGTH: i32 = 40;
    let menu = pancurses::newwin(
        MENU_WIDTH,
        MENU_LENGTH,
        (stdscr.get_max_y() - MENU_WIDTH) / 2,
        (stdscr.get_max_x() - MENU_LENGTH) / 2,
    );
    let players = user::User::leaderboard(10);
    menu.addstr("\n");
    for (i, player) in players.iter().enumerate() {
        menu.addstr(format!(" {}. {}: {}\n", i, player.username, player.score)); // TODO: leaderboard starts at 0 or 1
    }
    menu.draw_box(0, 0);
    menu.getch();
}

fn signup(stdscr: &Window) {
    const RED: i16 = 1;
    const GREEN: i16 = 2;
    pancurses::init_pair(RED, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
    pancurses::init_pair(GREEN, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    pancurses::curs_set(1);
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
    let mut userdetail = user::User::new();
    let mut username_selected = true;
    loop {
        menu.noutrefresh();
        if username_selected {
            username.noutrefresh();
        } else {
            password.noutrefresh();
        }
        pancurses::doupdate();
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
                        menu.color_set(RED);
                        menu.mvaddstr(9, (40 - 30) / 2, "Username unavailable");
                    } else {
                        menu.color_set(GREEN);
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
        password.mvaddstr(1, 1, "*".repeat(userdetail.password.len()));
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
            Input::Character('q') => {
                return 9999; // Any number that isn't chosen works
            }
            _ => (),
        }
        menu.refresh();
    }
}
