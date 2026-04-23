use super::app::{App, Mode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

pub fn draw(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Length(3)])
        .split(f.area());

    // Header
    let title = match &app.mode {
        Mode::Search => format!("/ {}_", app.query),
        _ => if app.query.is_empty() { "ssh-menu".into() } else { format!("filter: {}", app.query) },
    };
    f.render_widget(
        Paragraph::new(title).block(Block::default().borders(Borders::ALL).title("SSH Menu")),
        chunks[0],
    );

    // List
    let items: Vec<ListItem> = app.filtered.iter().filter_map(|i| {
        app.cfg.hosts.get(*i).map(|h| ListItem::new(h.display_line()))
    }).collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(" Hosts ({}) ", app.filtered.len())))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    // Footer
    let footer_text = match &app.mode {
        Mode::Search => "Type to filter  Esc=clear  Up/Down=move  Enter=back to list".to_string(),
        Mode::Form(_) => "Tab/Up/Down=field  Enter or Ctrl-S=save  Esc=cancel".to_string(),
        Mode::Confirm(m, _) => m.clone(),
        Mode::Normal => app.status.clone(),
    };
    f.render_widget(
        Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );

    // Form overlay
    if let Mode::Form(fs) = &app.mode {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);
        let mut lines: Vec<Line> = vec![
            Line::from(Span::styled(
                if fs.editing_index.is_some() { "Edit host" } else { "Add host" },
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];
        for (i, (k, v)) in fs.fields.iter().enumerate() {
            let marker = if i == fs.cursor { "> " } else { "  " };
            let style = if i == fs.cursor {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{}{:<8}: ", marker, k), style),
                Span::raw(v.clone()),
                Span::styled(if i == fs.cursor { "_" } else { "" }, style),
            ]));
        }
        f.render_widget(
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Host editor ")),
            area,
        );
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
