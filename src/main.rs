use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, self}, cursor,
};
use rand::Rng;
use std::{collections::VecDeque, io::stdout, time::Duration};

const SCREENSIZE: (usize, usize) = (40, 20);
type Screen = [[char; SCREENSIZE.1]; SCREENSIZE.0];
type Player = VecDeque<(usize, usize)>;

fn main() {
    enable_raw_mode().unwrap();

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All)
    )
    .unwrap();

    let mut screen: Screen = [[' '; SCREENSIZE.1]; SCREENSIZE.0];
    let mut dir: (i8, i8) = (0, -1);

    let initial_player_pos = (SCREENSIZE.0 / 2, SCREENSIZE.1 / 2);
    let mut player: Player = Player::new();
    player.push_front(initial_player_pos);
    screen[initial_player_pos.0][initial_player_pos.1] = 'O';

    draw_border(&mut screen);
    spawn_food(&mut screen);

    loop {
        execute!(
            stdout(),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        ).unwrap();

        read_input(&mut dir);
        move_player(&mut screen, dir, &mut player);
        print_screen(&screen);
        print!("Length: {}\r\n", player.len());

        std::thread::sleep(Duration::from_millis(125));
    }
}

fn move_player(screen: &mut Screen, dir: (i8, i8), player: &mut Player) {
    if player.is_empty() {
        return;
    }

    let old_pos = player.front().unwrap().clone();
    let new_pos = (
        (old_pos.0 as i8 + dir.0) as usize,
        (old_pos.1 as i8 + dir.1) as usize,
    );

    let next_slot = screen[new_pos.0][new_pos.1];
    if next_slot == 'O' || next_slot == '|' || next_slot == '-' {
        death(player);
    }

    if next_slot == 'X' {
        screen[new_pos.0][new_pos.1] = 'O';
        player.push_front(new_pos);
        spawn_food(screen);
    } else {
        screen[old_pos.0][old_pos.1] = ' ';
        move_next(screen, player);
        screen[new_pos.0][new_pos.1] = 'O';
        player.pop_front();
        player.push_front(new_pos);
    }
}

fn move_next(screen: &mut Screen, player: &mut Player) {
    if player.len() < 2 {
        return;
    }

    let front = player.pop_front().unwrap();
    let next = player.front().unwrap();
    let dir = (front.0 as i8 - next.0 as i8, front.1 as i8 - next.1 as i8);
    move_player(screen, dir, player);
    player.push_front(front);
}

fn spawn_food(screen: &mut Screen) {
    let mut rng = rand::thread_rng();
    let mut pos = (0, 0);

    while screen[pos.0][pos.1] != ' ' {
        let x = rng.gen_range(1..(SCREENSIZE.0 - 1));
        let y = rng.gen_range(1..(SCREENSIZE.1 - 1));
        pos = (x, y);
    }

    screen[pos.0][pos.1] = 'X';
}

fn death(player: &Player) {
    execute!(stdout(), cursor::Show).unwrap();
    disable_raw_mode().unwrap();
    panic!("You died! Final length: {}", player.len());
}

fn read_input(dir: &mut (i8, i8)) {
    use crossterm::event::{poll, read, Event, KeyCode};

    let mut new_dir = dir.clone();

    while poll(Duration::ZERO).unwrap() {
        new_dir = match read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char('w') | KeyCode::Up => (0, -1),
                KeyCode::Char('a') | KeyCode::Left => (-1, 0),
                KeyCode::Char('s') | KeyCode::Down => (0, 1),
                KeyCode::Char('d') | KeyCode::Right => (1, 0),
                KeyCode::Char('q') | KeyCode::Char('c') => quit_game(),
                _ => new_dir,
            },
            _ => new_dir,
        }
    }

    if new_dir.0 != -dir.0 && new_dir.1 != -dir.1 {
        *dir = new_dir;
    }
}

fn quit_game() -> (i8, i8) {
    execute!(stdout(), cursor::Show).unwrap();
    disable_raw_mode().unwrap();
    std::process::exit(0)
}

fn draw_border(screen: &mut Screen) {
    let cols = screen.len();
    let rows = screen[0].len();

    for (x, col) in screen.iter_mut().enumerate() {
        for (y, cell) in col.iter_mut().enumerate() {
            if y == 0 || y == rows - 1 {
                *cell = '-';
            } else if x == 0 || x == cols - 1 {
                *cell = '|';
            }
        }
    }
}

fn print_screen(screen: &Screen) {
    for y in 0..screen[0].len() {
        for x in 0..screen.len() {
            print!("{}", screen[x][y]);
        }
        print!("\r\n");
    }
}
