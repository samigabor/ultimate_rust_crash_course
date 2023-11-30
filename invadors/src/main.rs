use std::error::Error;
use rusty_audio::Audio;
use std::io;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crate::terminal::EnterAlternateScreen;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event;
use core::time::Duration;
use invadors::frame;
use invadors::render;
use std::thread;
use invadors::frame::Drawable;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = std::sync::mpsc::channel();
    let render_handle = std::thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = invadors::player::Player::new();
    'gameloop: loop {
        // Per-frame init
        let mut curr_frame = frame::new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => {
                        player.move_left();
                        audio.play("move");
                    }
                    KeyCode::Right => {
                        player.move_right();
                        audio.play("move");
                    }
                    KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Dreaw & render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame).unwrap();
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
