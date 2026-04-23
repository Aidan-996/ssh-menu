use super::app::{App, Mode};
use crate::ssh;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

const FIELD_HINTS: &[(&str, &str)] = &[
    ("name",  "Required. Display alias, any text (e.g. 'prod-db', '我的机房')."),
    ("host",  "Required. IP or hostname (e.g. '10.0.0.5', 'db.example.com')."),
    ("user",  "Login user. Default 'root'."),
    ("port",  "SSH port. Default 22."),
    ("key",   "Private key path. Supports '~/' (e.g. '~/.ssh/id_rsa'). Empty = default agent/keys."),
    ("group", "Optional group for visual organization (e.g. 'prod', 'cloud')."),
    ("tags",  "Comma-separated tags for search (e.g. 'mysql,linux'). Empty OK."),
    ("jump",  "Optional ProxyJump: name of another host in this list. Empty for direct."),
    ("note",  "Free-text note, shown in the details panel."),
];

fn field_hint(name: &str) -> &'static str {
    FIELD_HINTS.iter().find(|(k, _)| *k == name).map(|(_, v)| *v).unwrap_or("")
}

pub fn draw(f: &mut ratatui::Frame, app: &mut App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Length(3)])
        .split(f.area());

    draw_header(f, outer[0], app);

    // Body: optional split into list + details
    if app.show_details && !app.filtered.is_empty() {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(outer[1]);
        draw_list(f, body[0], app);
        draw_details(f, body[1], app);
    } else {
        draw_list(f, outer[1], app);
    }

    draw_footer(f, outer[2], app);

    // Overlays
    match &app.mode {
        Mode::Form(_) => draw_form(f, app),
        Mode::Help => draw_help(f),
        _ => {}
    }
}

fn draw_header(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let left = match &app.mode {
        Mode::Search => Line::from(vec![
            Span::styled("/ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&app.query),
            Span::styled("_", Style::default().fg(Color::Cyan).add_modifier(Modifier::SLOW_BLINK)),
        ]),
        _ => if app.query.is_empty() {
            Line::from(vec![
                Span::styled("🔌 ssh-menu", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" v{}", env!("CARGO_PKG_VERSION")), Style::default().fg(Color::DarkGray)),
            ])
        } else {
            Line::from(vec![
                Span::styled("filter: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&app.query, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ])
        },
    };

    let right = format!(
        " {} hosts • sort:{} • details:{} ",
        app.cfg.hosts.len(),
        app.sort_by.label(),
        if app.show_details { "on" } else { "off" },
    );

    let title = Line::from(vec![
        Span::raw(" SSH Menu "),
        Span::styled(right, Style::default().fg(Color::DarkGray)),
    ]);

    f.render_widget(
        Paragraph::new(left).block(Block::default().borders(Borders::ALL).title(title)),
        area,
    );
}

fn draw_list(f: &mut ratatui::Frame, area: Rect, app: &mut App) {
    if app.filtered.is_empty() {
        let msg = if app.cfg.hosts.is_empty() {
            vec![
                Line::from(""),
                Line::from(Span::styled("  No hosts yet.", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
                Line::from(""),
                Line::from("  Press  a  to add your first host"),
                Line::from("  Or run  ssh-menu import  to pull from ~/.ssh/config"),
                Line::from(""),
                Line::from(Span::styled("  Press ? for help", Style::default().fg(Color::DarkGray))),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from(Span::styled("  No matches.", Style::default().fg(Color::Yellow))),
                Line::from(format!("  query: {:?}", app.query)),
                Line::from(""),
                Line::from(Span::styled("  Esc to clear filter", Style::default().fg(Color::DarkGray))),
            ]
        };
        f.render_widget(
            Paragraph::new(msg).block(Block::default().borders(Borders::ALL).title(" Hosts ")),
            area,
        );
        return;
    }

    let items: Vec<ListItem> = app.filtered.iter().enumerate().filter_map(|(vis_idx, i)| {
        let h = app.cfg.hosts.get(*i)?;
        let num = format!("{:>2} ", if vis_idx < 9 { format!("{}", vis_idx + 1) } else { "·".into() });
        let group = format!("{:<10}", h.group.as_deref().unwrap_or(""));
        let name = format!("{:<20}", truncate(&h.name, 20));
        let conn = if h.port == 22 {
            format!("{}@{}", h.user, h.host)
        } else {
            format!("{}@{}:{}", h.user, h.host, h.port)
        };
        let jump = h.jump.as_deref().map(|j| format!(" ↪{}", j)).unwrap_or_default();
        let tags = if h.tags.is_empty() { String::new() } else { format!(" [{}]", h.tags.join(",")) };

        let mut spans = vec![
            Span::styled(num, Style::default().fg(Color::DarkGray)),
            Span::styled(group, Style::default().fg(Color::Magenta)),
            Span::raw(" "),
            Span::styled(name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(conn, Style::default().fg(Color::Green)),
        ];
        if !jump.is_empty() {
            spans.push(Span::styled(jump, Style::default().fg(Color::Yellow)));
        }
        if !tags.is_empty() {
            spans.push(Span::styled(tags, Style::default().fg(Color::Blue)));
        }
        Some(ListItem::new(Line::from(spans)))
    }).collect();

    let title = format!(" Hosts ({}) ", app.filtered.len());
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_details(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let Some(h) = app.selected_host() else {
        f.render_widget(Block::default().borders(Borders::ALL).title(" Details "), area);
        return;
    };

    let mut lines: Vec<Line> = vec![];
    let kv = |k: &str, v: String| Line::from(vec![
        Span::styled(format!("{:<10}", k), Style::default().fg(Color::DarkGray)),
        Span::raw(v),
    ]);
    let kv_color = |k: &str, v: String, c: Color| Line::from(vec![
        Span::styled(format!("{:<10}", k), Style::default().fg(Color::DarkGray)),
        Span::styled(v, Style::default().fg(c).add_modifier(Modifier::BOLD)),
    ]);

    lines.push(kv_color("name", h.name.clone(), Color::Cyan));
    lines.push(kv("host", h.host.clone()));
    lines.push(kv("user", h.user.clone()));
    lines.push(kv("port", h.port.to_string()));
    if let Some(k) = &h.key { lines.push(kv("key", k.clone())); }
    if let Some(g) = &h.group { lines.push(kv_color("group", g.clone(), Color::Magenta)); }
    if !h.tags.is_empty() {
        lines.push(kv_color("tags", h.tags.join(", "), Color::Blue));
    }
    if let Some(j) = &h.jump { lines.push(kv_color("jump", j.clone(), Color::Yellow)); }
    if !h.extra.is_empty() { lines.push(kv("extra", h.extra.join(" "))); }
    if let Some(n) = &h.note { lines.push(kv("note", n.clone())); }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("── usage ──", Style::default().fg(Color::DarkGray))));
    lines.push(kv("connects", h.use_count.to_string()));
    lines.push(kv("last", h.last_used.as_deref().map(ssh::time_ago).unwrap_or_else(|| "never".into())));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("── ssh command ──", Style::default().fg(Color::DarkGray))));
    let args = ssh::build_ssh_args(&app.cfg, h);
    lines.push(Line::from(Span::styled(
        format!("ssh {}", args.join(" ")),
        Style::default().fg(Color::Green),
    )));

    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title(" Details ")),
        area,
    );
}

fn draw_footer(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let (text, color) = match &app.mode {
        Mode::Search => (
            "type to filter  •  Ctrl-U clear  •  ↑/↓ move  •  Enter connect (if 1 match)  •  Esc back".to_string(),
            Color::Cyan,
        ),
        Mode::Form(_) => (
            "Tab/↑/↓ field  •  Ctrl-U clear field  •  Enter or Ctrl-S save  •  Esc cancel".to_string(),
            Color::Cyan,
        ),
        Mode::Confirm(m, _) => (m.clone(), Color::Red),
        Mode::Help => ("press any key to close help".into(), Color::Cyan),
        Mode::Normal => (app.status.clone(), Color::Yellow),
    };
    f.render_widget(
        Paragraph::new(text)
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL)),
        area,
    );
}

fn draw_form(f: &mut ratatui::Frame, app: &App) {
    let Mode::Form(fs) = &app.mode else { return; };
    let area = centered_rect(75, 80, f.area());
    f.render_widget(Clear, area);

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            if fs.editing_index.is_some() { "✎ Edit host" } else { "✚ Add new host" },
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Required fields: name, host",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
    ];
    for (i, (k, v)) in fs.fields.iter().enumerate() {
        let active = i == fs.cursor;
        let marker = if active { "▶ " } else { "  " };
        let required = matches!(k.as_str(), "name" | "host");
        let label_style = if active {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if required {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Gray)
        };
        let star = if required { "*" } else { " " };
        let value_style = if active {
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let placeholder = if v.is_empty() && !active {
            Span::styled("(empty)", Style::default().fg(Color::DarkGray))
        } else {
            Span::styled(v.clone(), value_style)
        };
        lines.push(Line::from(vec![
            Span::styled(marker, label_style),
            Span::styled(format!("{}{:<7}", star, k), label_style),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            placeholder,
            Span::styled(if active { "_" } else { "" }, Style::default().fg(Color::Yellow)),
        ]));
        if active {
            lines.push(Line::from(vec![
                Span::styled("            ", Style::default()),
                Span::styled(field_hint(k), Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
            ]));
        }
    }

    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Host editor ")),
        area,
    );
}

fn draw_help(f: &mut ratatui::Frame) {
    let area = centered_rect(70, 80, f.area());
    f.render_widget(Clear, area);

    let header = |s: &str| Line::from(Span::styled(
        format!("── {} ──", s),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ));
    let key = |k: &str, desc: &str| Line::from(vec![
        Span::styled(format!("  {:<12}", k), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(desc.to_string()),
    ]);

    let lines: Vec<Line> = vec![
        Line::from(Span::styled("ssh-menu — keyboard reference",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD))),
        Line::from(""),
        header("Navigation"),
        key("↑/↓ j/k",    "move selection"),
        key("PgUp/PgDn",  "jump 10 items"),
        key("g / G",      "first / last"),
        key("Home/End",   "first / last"),
        key("1-9",        "jump to Nth visible"),
        key("a-z",        "jump to next host starting with letter"),
        Line::from(""),
        header("Actions"),
        key("Enter",      "connect to selected host"),
        key("a",          "add a new host"),
        key("e",          "edit selected host"),
        key("D",          "delete selected host (Shift+d, asks y/N)"),
        key("y",          "show the equivalent ssh command in status bar"),
        Line::from(""),
        header("View"),
        key("/",          "enter search mode (live filter)"),
        key("s",          "cycle sort: name → group → recent → most-used"),
        key("i",          "toggle details panel"),
        key("r",          "refresh / re-filter"),
        key("?",          "toggle this help"),
        Line::from(""),
        header("Exit"),
        key("q / Esc",    "quit (or close overlay)"),
        key("Ctrl-C",     "quit"),
        Line::from(""),
        header("Search mode"),
        key("any char",   "add to filter, live update"),
        key("Backspace",  "delete last char"),
        key("Ctrl-U",     "clear filter"),
        key("Enter",      "connect if exactly 1 match"),
        key("Esc",        "clear and return to normal"),
        Line::from(""),
        header("Form mode (add / edit)"),
        key("Tab / ↓",    "next field"),
        key("Shift-Tab ↑","prev field"),
        key("Ctrl-U",     "clear current field"),
        key("Enter/Ctrl-S","save and close"),
        key("Esc",        "cancel without saving"),
        Line::from(""),
        Line::from(Span::styled("Press ?, Esc, Enter or q to close this help",
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))),
    ];

    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" ? Help ")),
        area,
    );
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_string() }
    else {
        let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
        out.push('…');
        out
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
