//! RSpeek interactive terminal client (ratatui TUI).

use std::io;
use std::time::Duration;

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use rspeek_client_core::ClientState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)
        .context("failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("failed to initialize ratatui terminal")?;

    let state = ClientState::new();

    let res = run_tui(&mut terminal, state).await;

    disable_raw_mode().ok();
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();

    res
}

async fn run_tui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: ClientState,
) -> anyhow::Result<()> {
    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(5),
                        Constraint::Length(3),
                    ])
                    .split(f.area());

                // Header
                let header = Paragraph::new(" RSpeek TUI Client | Press 'q' to quit")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .block(Block::default().borders(Borders::ALL).title("RSpeek"));
                f.render_widget(header, chunks[0]);

                // Body
                let status = match state.connection_status {
                    rspeek_client_core::ConnectionStatus::Disconnected => "Status: Disconnected",
                    rspeek_client_core::ConnectionStatus::Connecting { .. } => {
                        "Status: Connecting..."
                    }
                    rspeek_client_core::ConnectionStatus::Connected { .. } => "Status: Connected",
                    rspeek_client_core::ConnectionStatus::Reconnecting { .. } => {
                        "Status: Reconnecting..."
                    }
                };
                let body = Paragraph::new(format!(
                    "{}\n\nServers: {}\nMessages: {}\n\n(TUI implementation foundation ready)",
                    status,
                    state.servers.len(),
                    state.messages.len()
                ))
                .block(Block::default().borders(Borders::ALL).title("Overview"));
                f.render_widget(body, chunks[1]);

                // Footer
                let footer = Paragraph::new(" [Ready] ")
                    .block(Block::default().borders(Borders::ALL).title("Status"));
                f.render_widget(footer, chunks[2]);
            })
            .map_err(|e| anyhow::anyhow!("terminal draw error: {e}"))?;

        if event::poll(Duration::from_millis(100))?
            && matches!(event::read()?, Event::Key(key) if key.code == KeyCode::Char('q'))
        {
            break;
        }
    }

    Ok(())
}
