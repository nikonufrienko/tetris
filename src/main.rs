use bitmaps::TetraminoBitmap;
use rand::rngs::ThreadRng;
use std::io::Stdout;
use std::thread::sleep;
use std::{io::stdout, time::Duration};

#[macro_use]
extern crate crossterm;
use crossterm::cursor;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

mod bitmaps;

const BRICK_CELL: &str = "[ ]";
const EMPTY_CELL: &str = " . ";
const FIELD_WIDTH: u8 = 10;
const FIELD_HEIGHT: u8 = 20;
const FIELD_CHAR_WIDTH: u8 = FIELD_WIDTH * BRICK_CELL.len() as u8 + 2;

const MOVE_RATE: u32 = 20; // Maximum movements tetramino per step
const STEP_PERIOD: u32 = 500; // Period
const GAME_OVER_STR: &str = "GAME OVER";

#[derive(Copy, Clone, PartialEq)]
enum TetrisCell {
    EMPTY,
    BRICK(u8, u8, u8),
}

impl TetrisCell {
    fn from_color(rgb_color: (u8, u8, u8)) -> TetrisCell {
        return TetrisCell::BRICK(rgb_color.0, rgb_color.1, rgb_color.2);
    }
}

#[derive(PartialEq)]
enum TetraminoMove {
    None,
    Right,
    Left,
}

struct TetraminoAction {
    tetr_move: TetraminoMove,
    tetr_switch_rot: bool,
    tetr_force_down: bool,
    exit: bool,
}

struct TetrisField {
    // Game field data (exclude moving tetramino)
    game_field: [[TetrisCell; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
    x_pos: i8,
    y_pos: i8,
    rot: u8, // 0..3
    curr_tetr: &'static TetraminoBitmap,
    rng: ThreadRng,
    stdout: Stdout,
    score: i32,
}

impl TetrisField {
    fn new(stdout: Stdout) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            game_field: [[TetrisCell::EMPTY; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
            x_pos: (FIELD_WIDTH / 2 - 2) as i8, // TODO: fixme
            y_pos: 0,
            rot: 1,
            curr_tetr: bitmaps::get_random(&mut rng),
            rng,
            score: 0,
            stdout,
        }
    }

    fn init(&mut self) {
        //going into raw mode
        crossterm::terminal::enable_raw_mode().unwrap();

        execute!(
            self.stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide,
            crossterm::style::SetForegroundColor(crossterm::style::Color::White)
        )
        .unwrap();
        for _ in 0..FIELD_CHAR_WIDTH {
            execute!(self.stdout, Print("=")).unwrap();
        }
        execute!(self.stdout, cursor::MoveTo(0, (FIELD_HEIGHT + 1u8) as u16)).unwrap();
        for _ in 0..FIELD_CHAR_WIDTH {
            execute!(self.stdout, Print("=")).unwrap();
        }
        for x in [0, FIELD_CHAR_WIDTH - 1] {
            for y in 0..FIELD_HEIGHT {
                execute!(
                    self.stdout,
                    cursor::MoveTo(x as u16, (y + 1u8) as u16),
                    Print("|")
                )
                .unwrap();
            }
        }

        execute!(
            self.stdout,
            cursor::MoveTo(0, (FIELD_HEIGHT + 2u8) as u16),
            Print("<right arrow> -- move to the right"),
            cursor::MoveTo(0, (FIELD_HEIGHT + 3u8) as u16),
            Print("<left arrow>  -- move to the left"),
            cursor::MoveTo(0, (FIELD_HEIGHT + 4u8) as u16),
            Print("<up arrow>    -- rotate"),
            cursor::MoveTo(0, (FIELD_HEIGHT + 5u8) as u16),
            Print("<down arrow>  -- drop"),
            cursor::MoveTo(0, (FIELD_HEIGHT + 6u8) as u16),
            Print("ctrl+q        -- exit"),
            cursor::MoveTo(((FIELD_WIDTH * BRICK_CELL.len() as u8) + 4) as u16, 1),
            Print("Score:")
        )
        .unwrap();
    }

    fn get_action(&mut self) -> TetraminoAction {
        let mut act = TetraminoAction {
            tetr_move: TetraminoMove::None,
            tetr_switch_rot: false,
            tetr_force_down: false,
            exit: false,
        };

        while crossterm::event::poll(Duration::from_millis(0)).unwrap() {
            let evt = crossterm::event::read().unwrap();
            if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
                code,
                modifiers,
                kind,
                ..
            }) = evt
            {
                if kind == crossterm::event::KeyEventKind::Press {
                    match code {
                        crossterm::event::KeyCode::Left => {
                            act.tetr_move = TetraminoMove::Left;
                        }
                        crossterm::event::KeyCode::Right => {
                            act.tetr_move = TetraminoMove::Right;
                        }
                        crossterm::event::KeyCode::Up => {
                            act.tetr_switch_rot = true;
                        }
                        crossterm::event::KeyCode::Down => {
                            act.tetr_force_down = true;
                        }
                        _ => {}
                    }
                    if modifiers == crossterm::event::KeyModifiers::CONTROL {
                        match code {
                            crossterm::event::KeyCode::Char('q') => {
                                act.exit = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        return act;
    }

    fn display(&mut self) {
        for (y, line) in self.game_field.iter().enumerate() {
            execute!(self.stdout, cursor::MoveTo(1, 1 + y as u16)).unwrap();
            for cell in line {
                execute!(
                    self.stdout,
                    crossterm::style::SetBackgroundColor(match cell {
                        TetrisCell::EMPTY => crossterm::style::Color::Reset,
                        TetrisCell::BRICK(r, g, b) => crossterm::style::Color::Rgb {
                            r: (*r),
                            g: (*g),
                            b: (*b)
                        },
                    }),
                    Print(match cell {
                        TetrisCell::EMPTY => EMPTY_CELL,
                        _ => BRICK_CELL,
                    })
                )
                .unwrap();
            }
        }
    }

    fn game_over(&mut self) {
        let center_x = ((FIELD_WIDTH * BRICK_CELL.len() as u8 + 2) / 2) as u16
            - (GAME_OVER_STR.len() / 2) as u16;
        let center_y = ((FIELD_HEIGHT + 2) / 2) as u16;
        execute!(
            self.stdout,
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Red),
            cursor::MoveTo(center_x, center_y),
            Print(GAME_OVER_STR),
            cursor::MoveTo(center_x - 1, center_y),
            Print(" "),
            cursor::MoveTo(center_x + GAME_OVER_STR.len() as u16, center_y),
            Print(" "),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset),
            cursor::MoveTo(0, FIELD_HEIGHT as u16 + 10)
        )
        .unwrap();
        println!("Press any key to continue...");
        loop {
            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key { .. } => break,
                _ => continue,
            }
        }
    }

    // Injecting tetramino in game field and return collision resut
    fn inject_tetr(&mut self, fill_value: TetrisCell) {
        let (w, h) = self.curr_tetr.get_dimension(self.rot);
        for y in 0..h {
            for x in 0..w {
                let y_gpos: i8 = y as i8 + self.y_pos;
                let x_gpos: i8 = x as i8 + self.x_pos;
                if (x_gpos < FIELD_WIDTH as i8
                    && y_gpos < FIELD_HEIGHT as i8
                    && x_gpos >= 0
                    && y_gpos >= 0)
                    && !self.curr_tetr.is_empty_cell(x, y, self.rot)
                {
                    self.game_field[y_gpos as usize][x_gpos as usize] = fill_value;
                }
            }
        }
    }

    fn check_collisions(&self) -> bool {
        let (w, h) = self.curr_tetr.get_dimension(self.rot);
        for y in 0..h {
            for x in 0..w {
                let y_gpos: i8 = y as i8 + self.y_pos;
                let x_gpos: i8 = x as i8 + self.x_pos;
                if (x_gpos >= FIELD_WIDTH as i8
                    || y_gpos >= FIELD_HEIGHT as i8
                    || x_gpos < 0
                    || y_gpos < 0
                    || self.game_field[y_gpos as usize][x_gpos as usize] != TetrisCell::EMPTY)
                    && !self.curr_tetr.is_empty_cell(x, y, self.rot)
                {
                    return true;
                }
            }
        }
        return false;
    }

    fn apply_action(&mut self, action: TetraminoAction) {
        let x_delta: i8 = match action.tetr_move {
            TetraminoMove::Right => 1,
            TetraminoMove::Left => -1,
            _ => 0,
        };
        self.x_pos = self.x_pos + x_delta;
        if self.check_collisions() {
            self.x_pos = self.x_pos - x_delta;
        }
        if action.tetr_switch_rot {
            self.rot = (self.rot + 1) % 4;
            if self.check_collisions() {
                self.rot = (self.rot + 3) % 4;
            }
        }
    }

    fn v_step(&mut self, force_down: bool) -> bool {
        let mut is_collide: bool;
        // TODO: Simplify collision checking
        loop {
            self.y_pos += 1;
            is_collide = self.check_collisions();
            if is_collide {
                self.y_pos -= 1;
            }
            if !force_down || is_collide {
                break;
            }
        }
        return is_collide;
    }

    fn clear_line(&mut self, line_ind: u8) {
        for i in (0..line_ind as usize).rev() {
            self.game_field[i + 1] = self.game_field[i];
        }
    }

    fn refresh_field(&mut self) -> u8 {
        let mut cnt: u8 = 0;
        for i in 0..self.game_field.len() {
            let mut clear: bool = true;
            for cell in self.game_field[i] {
                if cell == TetrisCell::EMPTY {
                    clear = false;
                    break;
                }
            }
            if clear {
                self.clear_line(i as u8);
                cnt += 1;
            }
        }
        cnt
    }

    fn display_scrore(&mut self) {
        execute!(
            self.stdout,
            cursor::MoveTo(((FIELD_WIDTH * BRICK_CELL.len() as u8) + 4) as u16, 2),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset),
            Print(format!("{:06}", self.score))
        )
        .unwrap();
    }

    fn step(&mut self) -> bool {
        self.display_scrore();
        let mut changed = true;
        let mut force_down = false;
        let mut exit;
        for _ in 0..MOVE_RATE {
            self.inject_tetr(TetrisCell::from_color(self.curr_tetr.get_color())); // Inject tetramino to buffer
            if changed {
                self.display();
            }
            sleep(Duration::from_millis((STEP_PERIOD / MOVE_RATE) as u64));
            self.inject_tetr(TetrisCell::EMPTY); // Clear tetramino from buffer

            let action: TetraminoAction = self.get_action();
            exit = action.exit;
            if exit {
                self.close();
                return true;
            }
            force_down = action.tetr_force_down;
            changed = (action.tetr_move != TetraminoMove::None) || action.tetr_switch_rot;

            self.apply_action(action);

            if force_down {
                break;
            }
        }
        let is_last_step = self.v_step(force_down);
        self.inject_tetr(TetrisCell::from_color(self.curr_tetr.get_color())); // Inject tetramino to buffer
        self.display();
        if is_last_step {
            self.score += match self.refresh_field() {
                1u8 => 100,
                2u8 => 300,
                3u8 => 700,
                4u8 => 1500,
                _ => 0,
            };
            self.curr_tetr = bitmaps::get_random(&mut self.rng);
            self.x_pos = (FIELD_WIDTH / 2 - 2) as i8; // TODO: fixme
            self.y_pos = 0;
            self.rot = 0;
            if self.check_collisions() {
                self.game_over();
                return true;
            }
        }
        return false;
    }

    fn close(&mut self) {
        execute!(
            self.stdout,
            cursor::MoveTo(0, FIELD_HEIGHT as u16 + 10),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset),
            crossterm::style::SetForegroundColor(crossterm::style::Color::Reset),
            cursor::Show
        )
        .unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

fn main() {
    let stdout = stdout();
    let mut tetris = TetrisField::new(stdout);
    tetris.init();
    loop {
        if tetris.step() {
            tetris.close();
            break;
        }
    }
}
