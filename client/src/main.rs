use pancurses::{Input, Window, A_REVERSE};

mod user;

fn main() {
    use clap::Arg;

    let matches = clap::App::new("Dice Game TUI client")
        .version("0.1")
        .author("Nils André-Chang <nils@nilsand.re>")
        .about("TODO")
        .arg(
            Arg::with_name("server")
                .takes_value(true)
                .default_value("localhost:8000"),
        )
        .get_matches();

    let server_connection = user::ServerConnection::new(matches.value_of("server").unwrap());

    let stdscr = pancurses::initscr();
    if pancurses::has_colors() {
        pancurses::start_color();
    }
    pancurses::curs_set(0);
    pancurses::noecho();

    const DISCONNECTED_CHOICES: &[&str] = &["Sign up", "Sign in", "Leaderboard", "About", "Quit"];
    const LOGGEDIN_CHOICES: &[&str] = &["Play", "Leaderboard", "Disconnect", "Quit"];
    const PLAY_MENU: &[&str] = &["Create Game", "Join Game", "Back"];
    let mut credentials: Option<user::User> = None;

    loop {
        match credentials {
            Some(ref user) => match main_menu(
                &stdscr,
                &format!("User: {}", user.username),
                LOGGEDIN_CHOICES,
            ) {
                Some(0) => loop {
                    match main_menu(&stdscr, "", PLAY_MENU) {
                        Some(0) => {
                            let game_id = server_connection.create_game(user);
                        }
                        Some(1) => {
                            let games = server_connection.get_games();
                            // TODO: make it so only games that aren't created by us are returned from the server
                            main_menu(
                                &stdscr,
                                "Select game to join",
                                games.into_iter().filter_map(|game| {
                                    if game.player1 == user.username {
                                        None
                                    } else {
                                        Some(format!(
                                            "#{} Created by {}",
                                            game.game_id, game.player1
                                        ))
                                    }
                                }),
                            );
                        }
                        Some(2) | None => break,
                        Some(_) => unreachable!(),
                    }
                },
                Some(1) => leaderboard(&stdscr, &server_connection),
                Some(2) => credentials = None,
                Some(3) | None => break,
                Some(_) => unreachable!(),
            },
            None => match main_menu(&stdscr, "Main Menu", DISCONNECTED_CHOICES) {
                Some(0) => signup(&stdscr, &mut credentials, &server_connection),
                Some(1) => signin(&stdscr, &mut credentials, &server_connection),
                Some(2) => leaderboard(&stdscr, &server_connection),
                Some(3) => about(),
                Some(4) | None => break,
                Some(_) => unreachable!(),
            },
        }
    }
    pancurses::endwin();
}

fn leaderboard(stdscr: &Window, server_connection: &user::ServerConnection) {
    pancurses::curs_set(0);
    const MENU_WIDTH: i32 = 20;
    const MENU_LENGTH: i32 = 40;
    let menu = pancurses::newwin(
        MENU_WIDTH,
        MENU_LENGTH,
        (stdscr.get_max_y() - MENU_WIDTH) / 2,
        (stdscr.get_max_x() - MENU_LENGTH) / 2,
    );
    menu.draw_box(0, 0);
    match server_connection.leaderboard(10) {
        Ok(players) => {
            menu.addstr("Leaderboard");
            for (i, player) in players.iter().enumerate() {
                menu.mvaddstr(
                    i as i32 + 1,
                    1,
                    format!("{}. {}: {}\n", i, player.username, player.score),
                );
                // TODO: leaderboard starts at 0 or 1
            }
        }
        Err(_) => unimplemented!(),
    }
    menu.refresh();
    menu.getch();
}

fn signup(
    stdscr: &Window,
    credentials: &mut Option<user::User>,
    server_connection: &user::ServerConnection,
) {
    loop {
        let user_creds = ask_credentials(&stdscr, true, server_connection);
        match server_connection.register_user(&user_creds) {
            Ok(_) => {
                *credentials = Some(user_creds);
                break;
            }
            Err(_) => (),
        }
    }
}

fn signin(
    stdscr: &Window,
    credentials: &mut Option<user::User>,
    server_connection: &user::ServerConnection,
) {
    loop {
        *credentials = Some(ask_credentials(stdscr, false, server_connection));
        if server_connection.authenticate(credentials.as_ref().unwrap()) {
            break;
        }
    }
}

fn about() {}

fn main_menu<'a, S: AsRef<str>, C: 'a + AsRef<str>, T>(
    stdscr: &Window,
    title: S,
    choices: T,
) -> Option<usize>
where
    T: IntoIterator<Item = &'a C>,
{
    const LENGTH: i32 = 40;
    const WIDTH: i32 = 20;
    let menu = pancurses::newwin(
        WIDTH,
        LENGTH,
        (stdscr.get_max_y() - WIDTH) / 2,
        (stdscr.get_max_x() - LENGTH) / 2,
    );
    menu.draw_box(0, 0);
    menu.mvaddstr(0, 3, title); // TODO: should title be passed by reference
    let mut chosen = 0;
    loop {
        let mut length = 0;
        for (y, choice) in choices.into_iter().enumerate() {
            if y == chosen {
                menu.attron(A_REVERSE);
            }
            menu.mvaddstr(y as i32 + 1, 1, &choice);
            if y == chosen {
                menu.attroff(A_REVERSE);
            }
            length = y + 1;
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
                if chosen >= length {
                    chosen = length - 1;
                }
            }
            Input::Character('\n') | Input::Character('l') => {
                menu.delwin();
                return Some(chosen);
            }
            Input::Character('q') => return None,
            _ => (),
        }
    }
}

fn ask_credentials(
    stdscr: &Window,
    signup: bool,
    server_connection: &user::ServerConnection,
) -> user::User {
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
    username.mv(1, 1);
    password.mv(1, 1);
    menu.noutrefresh();
    let mut userdetail = user::User::new();
    let mut username_selected = true;
    loop {
        if username_selected {
            username.noutrefresh();
        } else {
            password.noutrefresh();
        }
        pancurses::doupdate();
        match if username_selected {
            username.getch().unwrap()
        } else {
            password.getch().unwrap()
        } {
            Input::Character('\n') => break,
            Input::KeyBackspace => {
                if username_selected {
                    userdetail.username.pop();
                    username.delch();
                // username.draw_box(0, 0);
                } else {
                    userdetail.password.pop();
                    password.delch();
                    // password.draw_box(0, 0);
                }
            }
            Input::Character('\t') => {
                if username_selected && signup {
                    match server_connection.user_exists(&userdetail.username) {
                        Ok(exists) => {
                            if exists {
                                menu.color_set(RED);
                                menu.mvaddstr(9, (40 - 30) / 2, "Username unavailable");
                            } else {
                                menu.color_set(GREEN);
                                // TODO: find a solution to the need of overwritting with spaces
                                menu.mvaddstr(9, (40 - 30) / 2, "Username available  ");
                            }
                        }
                        Err(_) => unimplemented!(),
                    }
                    menu.noutrefresh();
                }
                username_selected = !username_selected;
            }
            Input::Character(character) => {
                if username_selected {
                    userdetail.username.push(character);
                    username.addch(character);
                } else {
                    userdetail.password.push(character);
                    password.addch(character);
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
