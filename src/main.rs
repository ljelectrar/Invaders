use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{frame::{self, new_frame, Drawable}, player::Player, render};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self},
    time::Duration,
    {io, thread},
};


fn main()  -> Result<(), Box<dyn Error>> {
    
    // set up audio
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // RENDER LOOP IN A SEPARATE THREAD
    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
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

    // GAME LOOP
    let mut player = Player::new();

    'gameloop: loop {
        // Per-Frame init
        let mut curr_frame = new_frame();

        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // Player Movement
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
    
                    KeyCode::Esc | KeyCode::Char('s') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ =>{}
                }
            }
        }

        // Draw & Render
        player.draw(&mut curr_frame);

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
