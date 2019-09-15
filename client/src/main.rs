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
    let disconnected_choices = ["Sign up", "Sign in", "Leaderboard", "About", "Quit"];
    let loggedin_choices = ["Play", "Leaderboard", "Disconnect", "Quit"];
    let play_menu = ["Create Game", "Join Game", "Back"];
    let mut credentials: Option<user::User> = None;

    loop {
        match credentials {
            Some(_) => match main_menu(
                &stdscr,
                &format!("User: {}", credentials.as_ref().unwrap().username),
                &loggedin_choices,
            ) {
                0 => loop {
                    match main_menu(&stdscr, "", &play_menu) {
                        0 => {
                            let game_id = credentials.as_ref().unwrap().create_game();
                        }
                        1 => {
                            
                        },
                        _ => break,
                    }
                },
                1 => leaderboard(&stdscr),
                2 => credentials = None,
                _ => break,
            },
            None => match main_menu(&stdscr, "Main Menu", &disconnected_choices) {
                0 => signup(&stdscr, &mut credentials),
                1 => signin(&stdscr, &mut credentials),
                2 => leaderboard(&stdscr),
                3 => about(),
                _ => break,
            },
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
        menu.addstr(format!(" {}. {}: {}\n", i, player.username, player.score));
        // TODO: leaderboard starts at 0 or 1
    }
    menu.draw_box(0, 0);
    menu.getch();
}

fn signup(stdscr: &Window, credentials: &mut Option<user::User>) {
    loop {
        let user_creds = ask_credentials(&stdscr, true);
        match user_creds.register() {
            Ok(_) => {
                *credentials = Some(user_creds);
                break;
            }
            Err(_) => (),
        }
    }
}

fn signin(stdscr: &Window, credentials: &mut Option<user::User>) {
    loop {
        *credentials = Some(ask_credentials(stdscr, false));
        if credentials.as_ref().unwrap().authenticate() {
            break;
        }
    }
}

fn about() {}

fn main_menu(stdscr: &Window, title: &str, choices: &[&str]) -> usize {
    const LENGTH: i32 = 40;
    const WIDTH: i32 = 20;
    let menu = pancurses::newwin(
        WIDTH,
        LENGTH,
        (stdscr.get_max_y() - WIDTH) / 2,
        (stdscr.get_max_x() - LENGTH) / 2,
    );
    menu.draw_box(0, 0);
    menu.keypad(true);
    menu.mvaddstr(0, 3, title);
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
    }
}

fn ask_credentials(stdscr: &Window, signup: bool) -> user::User {
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
    menu.addstr(if signup {
        "Create an account:"
    } else {
        "Please enter your credentials"
    });
    menu.mvaddstr(3, 4, "Press enter when you're done.");
    menu.keypad(true);
    let mut userdetail = user::User::new();
    let mut username_selected = true;
    loop {
        username.mvaddstr(1, 1, &userdetail.username);
        password.mvaddstr(1, 1, "*".repeat(userdetail.password.len()));
        menu.noutrefresh();
        if username_selected {
            username.noutrefresh();
        } else {
            password.noutrefresh();
        }
        pancurses::doupdate();
        match menu.getch().unwrap() {
            Input::Character('\n') => break,
            Input::KeyBackspace => {
                if username_selected {
                    userdetail.username.pop();
                    username.erase();
                    username.draw_box(0, 0);
                } else {
                    userdetail.password.pop();
                    password.erase();
                    password.draw_box(0, 0);
                }
            }
            Input::Character('\t') => {
                if username_selected && signup {
                    if user::User::user_exists(&userdetail.username) {
                        menu.color_set(RED);
                        menu.mvaddstr(9, (40 - 30) / 2, "Username unavailable");
                    } else {
                        menu.color_set(GREEN);
                        menu.mvaddstr(9, (40 - 30) / 2, "Username available  ");
                    }
                }
                username_selected = !username_selected;
            }
            Input::Character(character) => {
                if username_selected {
                    userdetail.username.push(character);
                } else {
                    userdetail.password.push(character);
                }
            }
            _ => (),
        }
    }
    username.delwin();
    password.delwin();
    menu.delwin();
    pancurses::curs_set(0);
    userdetail
}
