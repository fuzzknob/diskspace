use crate::disks::DiskInfo;
use crossterm;
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Result};

pub fn print_result(drives: Vec<DiskInfo>) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let view_height = (drives.len() * 2) as u16;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(view_height),
        },
    )?;
    terminal.draw(|frame| {
        let area = frame.size();
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                (0..view_height)
                    .map(|_| Constraint::Length(2))
                    .collect::<Vec<_>>(),
            )
            .split(area);
        for (index, drive) in drives.iter().enumerate() {
            let [title_area, indicator_area] = *(Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(1, 2); 2])
                .split(rows[index]))
            else {
                return ();
            };
            let [gauge_area, info_area] = *(Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(indicator_area))
            else {
                return ();
            };
            frame.render_widget(Paragraph::new(drive.drive_name.clone()).bold(), title_area);
            frame.render_widget(
                Gauge::default()
                    .bg(Color::DarkGray)
                    .gauge_style(Style::new().light_cyan())
                    .percent(drive.used_percent)
                    .label(""),
                gauge_area,
            );
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::from(" "),
                    Span::from(format!("{} / {}", drive.used, drive.total))
                        .style(Style::default().fg(Color::LightCyan)),
                    Span::from(" "),
                    Span::from(format!("({})", drive.available))
                        .style(Style::default().fg(Color::Green)),
                ])),
                info_area,
            );
        }
    })?;
    crossterm::terminal::disable_raw_mode()?;
    println!("");
    Ok(())
}
