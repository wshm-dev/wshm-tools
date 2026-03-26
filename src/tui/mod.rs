pub mod app;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;

use crate::config::Config;
use crate::db::Database;

pub async fn run(config: &Config, db: &Database) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = app::App::new(config, db)?;
    let mut last_log_refresh = std::time::Instant::now();

    // Main loop
    let result = run_loop(&mut terminal, &mut app, db, &mut last_log_refresh);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut app::App,
    db: &Database,
    last_log_refresh: &mut std::time::Instant,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Auto-refresh logs every 5 seconds when on Activity tab
        if app.active_tab == app::Tab::Activity
            && last_log_refresh.elapsed() > std::time::Duration::from_secs(5)
        {
            app.refresh_logs();
            *last_log_refresh = std::time::Instant::now();
        }

        let timeout = std::time::Duration::from_millis(200);
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        return Ok(());
                    }
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('1') => app.active_tab = app::Tab::Issues,
                        KeyCode::Char('2') => app.active_tab = app::Tab::PullRequests,
                        KeyCode::Char('3') => app.active_tab = app::Tab::Queue,
                        KeyCode::Char('4') => app.active_tab = app::Tab::Stats,
                        KeyCode::Char('5') => app.active_tab = app::Tab::Activity,
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                        KeyCode::Char('r') => app.refresh(db)?,
                        KeyCode::Char('s') => app.set_sort(app::SortField::Number),
                        KeyCode::Char('t') => app.set_sort(app::SortField::Title),
                        KeyCode::Char('c') => app.set_sort(app::SortField::Category),
                        KeyCode::Char('p') => app.set_sort(app::SortField::Priority),
                        KeyCode::Char('a') => app.set_sort(app::SortField::Age),
                        KeyCode::Char('o') => app.set_sort(app::SortField::Confidence),
                        KeyCode::Char('m') => app.set_sort(app::SortField::Mergeable),
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
}
