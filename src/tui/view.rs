use super::app::{App, Mode};
use crate::ssh;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

// === Design tokens: dark, neon-tech palette ===
const BG:        Color = Color::Rgb(15, 17, 26);       // near-black blue
const SURFACE:   Color = Color::Rgb(22, 26, 40);       // panel surface
const ACCENT:    Color = Color::Rgb(127, 219, 255);    // neon cyan
const ACCENT2:   Color = Color::Rgb(170, 130, 255);    // purple accent
const MUTED:     Color = Color::Rgb(100, 110, 130);    // secondary text
const TEXT:      Color = Color::Rgb(220, 228, 245);    // primary text
const SUCCESS:   Color = Color::Rgb(110, 231, 183);    // mint green
const WARNING:   Color = Color::Rgb(250, 204, 21);     // amber
const DANGER:    Color = Color::Rgb(248, 113, 113);    // soft red
const INFO:      Color = Color::Rgb(96, 165, 250);     // bright blue

const FIELD_HINTS: &[(&str, &str)] = &[
    ("name",  "必填 · 显示别名（支持中文）"),
    ("host",  "必填 · IP 或主机名"),
    ("user",  "登录用户，默认 root"),
    ("port",  "SSH 端口，默认 22"),
    ("key",   "私钥路径，支持 ~/，留空使用默认/Agent"),
    ("group", "可选分组，用于视觉聚合"),
    ("tags",  "可选标签，逗号分隔，可用于搜索"),
    ("jump",  "可选跳板：填本列表里的另一台 name"),
    ("note",  "自由备注，显示在详情面板"),
];

fn field_hint(name: &str) -> &'static str {
    FIELD_HINTS.iter().find(|(k, _)| *k == name).map(|(_, v)| *v).unwrap_or("")
}

const GROUP_PALETTE: &[Color] = &[
    Color::Rgb(170, 130, 255),  // purple
    Color::Rgb(250, 204, 21),   // amber
    Color::Rgb(127, 219, 255),  // cyan
    Color::Rgb(110, 231, 183),  // mint
    Color::Rgb(96, 165, 250),   // blue
    Color::Rgb(248, 113, 113),  // coral
    Color::Rgb(244, 114, 182),  // pink
    Color::Rgb(251, 191, 36),   // orange
    Color::Rgb(52, 211, 153),   // emerald
];

fn color_for_group(g: &str) -> Color {
    let mut h: u32 = 5381;
    for b in g.as_bytes() { h = h.wrapping_mul(33).wrapping_add(*b as u32); }
    GROUP_PALETTE[(h as usize) % GROUP_PALETTE.len()]
}

fn panel<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(BG))
        .border_style(Style::default().fg(Color::Rgb(40, 48, 72)))
}

fn panel_accent<'a>(c: Color) -> Block<'a> {
    panel().border_style(Style::default().fg(c))
}

pub fn draw(f: &mut ratatui::Frame, app: &mut App) {
    // Paint background
    f.render_widget(
        Block::default().style(Style::default().bg(BG)),
        f.area(),
    );

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Length(3)])
        .split(f.area());

    draw_header(f, outer[0], app);

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

    match &app.mode {
        Mode::Form(_) => draw_form(f, app),
        Mode::Help => draw_help(f),
        _ => {}
    }
}

fn draw_header(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let left = match &app.mode {
        Mode::Search => Line::from(vec![
            Span::styled(" ⌕ ", Style::default().fg(ACCENT).bg(SURFACE).add_modifier(Modifier::BOLD)),
            Span::styled(" / ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(&app.query, Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
            Span::styled("▎", Style::default().fg(ACCENT).add_modifier(Modifier::SLOW_BLINK)),
        ]),
        _ => if app.query.is_empty() {
            Line::from(vec![
                Span::styled(" ◆ ", Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)),
                Span::styled("SSH", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
                Span::styled("·", Style::default().fg(MUTED)),
                Span::styled("MENU", Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  v{}", env!("CARGO_PKG_VERSION")), Style::default().fg(MUTED)),
                Span::styled("   ", Style::default()),
                Span::styled(" ? ", Style::default().fg(BG).bg(WARNING).add_modifier(Modifier::BOLD)),
                Span::styled(" 帮助 ", Style::default().fg(MUTED)),
                Span::styled(" │ ", Style::default().fg(Color::Rgb(40, 48, 72))),
                Span::styled("© 2026 ", Style::default().fg(MUTED)),
                Span::styled("Aidan-996", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
                Span::styled("  MIT", Style::default().fg(MUTED)),
            ])
        } else {
            Line::from(vec![
                Span::styled(" ⌕ ", Style::default().fg(BG).bg(ACCENT).add_modifier(Modifier::BOLD)),
                Span::styled(" 筛选: ", Style::default().fg(MUTED)),
                Span::styled(&app.query, Style::default().fg(WARNING).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" ({})", app.filtered.len()), Style::default().fg(MUTED)),
            ])
        },
    };

    let right_stats = Line::from(vec![
        Span::styled(" ⚡ ", Style::default().fg(SUCCESS)),
        Span::styled(format!("{}", app.cfg.hosts.len()), Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        Span::styled(" 主机 ", Style::default().fg(MUTED)),
        Span::styled("│ ", Style::default().fg(Color::Rgb(40, 48, 72))),
        Span::styled("⇅ ", Style::default().fg(ACCENT2)),
        Span::styled(app.sort_by.label(), Style::default().fg(TEXT)),
        Span::styled(" │ ", Style::default().fg(Color::Rgb(40, 48, 72))),
        Span::styled(
            if app.show_details { "◉ 详情" } else { "○ 详情" },
            Style::default().fg(if app.show_details { SUCCESS } else { MUTED }),
        ),
        Span::styled(" ", Style::default()),
    ]);

    let hstack = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    f.render_widget(
        Paragraph::new(left).block(
            panel_accent(ACCENT)
                .title(Line::from(vec![
                    Span::styled(" ╼ ", Style::default().fg(ACCENT2)),
                    Span::styled("CONTROL PANEL", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
                    Span::styled(" ╾", Style::default().fg(ACCENT2)),
                ])),
        ),
        hstack[0],
    );
    f.render_widget(
        Paragraph::new(right_stats).alignment(ratatui::layout::Alignment::Right).block(
            panel()
                .border_style(Style::default().fg(ACCENT2)),
        ),
        hstack[1],
    );
}

fn draw_list(f: &mut ratatui::Frame, area: Rect, app: &mut App) {
    if app.filtered.is_empty() {
        let msg = if app.cfg.hosts.is_empty() {
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(" ✨ ", Style::default().fg(BG).bg(WARNING).add_modifier(Modifier::BOLD)),
                    Span::styled(" 还没有主机", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  按 ", Style::default().fg(MUTED)),
                    Span::styled(" a ", Style::default().fg(BG).bg(ACCENT).add_modifier(Modifier::BOLD)),
                    Span::styled(" 添加第一台主机", Style::default().fg(TEXT)),
                ]),
                Line::from(vec![
                    Span::styled("  或运行 ", Style::default().fg(MUTED)),
                    Span::styled(" ssh-menu import ", Style::default().fg(BG).bg(SUCCESS).add_modifier(Modifier::BOLD)),
                    Span::styled(" 从 ~/.ssh/config 导入", Style::default().fg(TEXT)),
                ]),
                Line::from(""),
                Line::from(Span::styled("  按 ? 查看快捷键", Style::default().fg(MUTED).italic())),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from(Span::styled("  ∅  没有匹配的主机", Style::default().fg(WARNING))),
                Line::from(Span::styled(format!("  筛选词：{:?}", app.query), Style::default().fg(MUTED))),
                Line::from(""),
                Line::from(Span::styled("  按 Esc 清空筛选", Style::default().fg(MUTED).italic())),
            ]
        };
        f.render_widget(
            Paragraph::new(msg).block(panel().title(list_title(app))),
            area,
        );
        return;
    }

    let items: Vec<ListItem> = app.filtered.iter().enumerate().filter_map(|(vis_idx, i)| {
        let h = app.cfg.hosts.get(*i)?;
        Some(render_host_row(vis_idx, h))
    }).collect();

    let list = List::new(items)
        .block(panel().title(list_title(app)))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 50, 95))
                .fg(TEXT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ▎ ");
    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn list_title(app: &App) -> Line<'static> {
    Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled("▣", Style::default().fg(ACCENT)),
        Span::styled(" 主机列表 ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(format!("[{}] ", app.filtered.len()), Style::default().fg(MUTED)),
    ])
}

fn render_host_row(vis_idx: usize, h: &crate::config::Host) -> ListItem<'static> {
    // Status dot color: recent use → success; has jump → warning; else info.
    let dot_color = if let Some(lu) = &h.last_used {
        use time::{format_description::well_known::Rfc3339, OffsetDateTime};
        if let Ok(past) = OffsetDateTime::parse(lu, &Rfc3339) {
            let now = OffsetDateTime::now_utc();
            let hours = (now - past).whole_hours();
            if hours < 24 { SUCCESS } else if hours < 24 * 7 { INFO } else { MUTED }
        } else { MUTED }
    } else if h.jump.is_some() { WARNING } else { MUTED };

    let num_str = if vis_idx < 9 { format!("{}", vis_idx + 1) } else { "·".into() };
    let group_raw = h.group.as_deref().unwrap_or("-");
    let group_col = if group_raw == "-" { MUTED } else { color_for_group(group_raw) };
    let name = format!("{:<18}", truncate(&h.name, 18));

    // Connection string with distinct styling per part
    let port_str = if h.port == 22 { String::new() } else { format!(":{}", h.port) };
    let jump = h.jump.as_deref().map(|j| format!(" ↪ {}", j)).unwrap_or_default();
    let tags = if h.tags.is_empty() { String::new() } else { format!("  #{}", h.tags.join(" #")) };
    let uses = if h.use_count > 0 { format!("  ×{}", h.use_count) } else { String::new() };

    let mut spans = vec![
        Span::styled(format!(" {:>2} ", num_str), Style::default().fg(MUTED)),
        Span::styled("●", Style::default().fg(dot_color).add_modifier(Modifier::BOLD)),
        Span::styled(" ", Style::default()),
        Span::styled(format!("{:<10}", truncate(group_raw, 10)), Style::default().fg(group_col).add_modifier(Modifier::BOLD)),
        Span::styled("│ ", Style::default().fg(Color::Rgb(40, 48, 72))),
        Span::styled(name, Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(h.user.clone(), Style::default().fg(ACCENT2)),
        Span::styled("@", Style::default().fg(MUTED)),
        Span::styled(h.host.clone(), Style::default().fg(SUCCESS)),
    ];
    if !port_str.is_empty() {
        spans.push(Span::styled(port_str, Style::default().fg(INFO)));
    }
    if !jump.is_empty() {
        spans.push(Span::styled(jump, Style::default().fg(WARNING)));
    }
    if !tags.is_empty() {
        spans.push(Span::styled(tags, Style::default().fg(INFO).italic()));
    }
    if !uses.is_empty() {
        spans.push(Span::styled(uses, Style::default().fg(MUTED)));
    }
    ListItem::new(Line::from(spans))
}

fn draw_details(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let Some(h) = app.selected_host() else {
        draw_about(f, area);
        return;
    };

    let mut lines: Vec<Line> = vec![];

    // ── Card header: big name + status pill ──────────────────────
    let status_text = if let Some(lu) = &h.last_used {
        format!("● 活跃 · {}", ssh::time_ago(lu))
    } else {
        "○ 未连接".into()
    };
    let status_color = if h.last_used.is_some() { SUCCESS } else { MUTED };

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(&h.name, Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(status_text, Style::default().fg(status_color)),
    ]));
    lines.push(Line::from(""));

    // ── Connection card ─────────────────────────────────────────
    lines.push(section_title("CONNECTION"));
    lines.push(kv_large("🌐", "host", &h.host, SUCCESS));
    lines.push(kv_large("👤", "user", &h.user, ACCENT2));
    lines.push(kv_large("🔌", "port", &h.port.to_string(), INFO));
    if let Some(k) = &h.key {
        lines.push(kv_large("🔑", "key ", k, WARNING));
    }
    if let Some(j) = &h.jump {
        lines.push(kv_large("↪ ", "jump", j, WARNING));
    }
    lines.push(Line::from(""));

    // ── Meta card ──────────────────────────────────────────────
    if h.group.is_some() || !h.tags.is_empty() || h.note.is_some() || !h.extra.is_empty() {
        lines.push(section_title("META"));
        if let Some(g) = &h.group {
            let c = color_for_group(g);
            lines.push(Line::from(vec![
                Span::styled("   ", Style::default()),
                Span::styled(format!(" {} ", g), Style::default().bg(c).fg(BG).add_modifier(Modifier::BOLD)),
            ]));
        }
        if !h.tags.is_empty() {
            let mut spans = vec![Span::styled("   ", Style::default())];
            for t in &h.tags {
                spans.push(Span::styled(format!(" #{} ", t), Style::default().fg(INFO).add_modifier(Modifier::BOLD)));
                spans.push(Span::raw(" "));
            }
            lines.push(Line::from(spans));
        }
        if let Some(n) = &h.note {
            lines.push(Line::from(vec![
                Span::styled("  📝 ", Style::default().fg(MUTED)),
                Span::styled(n.clone(), Style::default().fg(TEXT)),
            ]));
        }
        if !h.extra.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  ⚙  ", Style::default().fg(MUTED)),
                Span::styled(h.extra.join(" "), Style::default().fg(MUTED)),
            ]));
        }
        lines.push(Line::from(""));
    }

    // ── Usage card ─────────────────────────────────────────────
    lines.push(section_title("USAGE"));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(format!("{:>4}", h.use_count), Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("  连接次数", Style::default().fg(MUTED)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            h.last_used.as_deref().map(ssh::time_ago).unwrap_or_else(|| "从未".into()),
            Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  最近使用", Style::default().fg(MUTED)),
    ]));
    lines.push(Line::from(""));

    // ── SSH command card ───────────────────────────────────────
    lines.push(section_title("SSH COMMAND"));
    let args = ssh::build_ssh_args(&app.cfg, h);
    lines.push(Line::from(vec![
        Span::styled(" $ ", Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)),
        Span::styled("ssh ", Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD)),
        Span::styled(args.join(" "), Style::default().fg(TEXT)),
    ]));
    lines.push(Line::from(Span::styled(
        "   按 y 在状态栏显示完整命令",
        Style::default().fg(MUTED).italic(),
    )));
    lines.push(Line::from(""));

    // ── Shortcuts card ─────────────────────────────────────────
    lines.push(section_title("SHORTCUTS"));
    for (k, desc) in SHORTCUTS_SHORT {
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(format!(" {} ", k), Style::default().fg(BG).bg(WARNING).add_modifier(Modifier::BOLD)),
            Span::styled("  ", Style::default()),
            Span::styled(*desc, Style::default().fg(TEXT)),
        ]));
    }
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(" ? ", Style::default().fg(BG).bg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
        Span::styled("查看全部快捷键", Style::default().fg(MUTED).italic()),
    ]));
    lines.push(Line::from(""));

    // ── About card ─────────────────────────────────────────────
    lines.push(section_title("ABOUT"));
    for l in about_lines() { lines.push(l); }

    let title = Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled("◈", Style::default().fg(ACCENT2)),
        Span::styled(" 主机详情 ", Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(panel_accent(ACCENT2).title(title)),
        area,
    );
}

fn kv_large(icon: &'static str, key: &'static str, value: &str, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("  {} ", icon), Style::default().fg(MUTED)),
        Span::styled(format!("{:<5}", key), Style::default().fg(MUTED)),
        Span::styled(value.to_string(), Style::default().fg(color).add_modifier(Modifier::BOLD)),
    ])
}

fn section_title(name: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(" ▸ ", Style::default().fg(ACCENT2)),
        Span::styled(name.to_string(), Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)),
        Span::styled(" ", Style::default()),
        Span::styled(
            "─".repeat(24),
            Style::default().fg(Color::Rgb(40, 48, 72)),
        ),
    ])
}

const SHORTCUTS_SHORT: &[(&str, &str)] = &[
    (" ⏎ ",      "连接选中主机"),
    (" a ",      "添加主机"),
    (" e ",      "编辑主机"),
    (" D ",      "删除主机（二次确认）"),
    (" / ",      "搜索过滤"),
    (" s ",      "切换排序"),
    (" i ",      "切换详情面板"),
    (" y ",      "显示等效 ssh 命令"),
    ("1-9",      "跳到第 N 个"),
    (" q ",      "退出"),
];

fn about_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("  🔌 ", Style::default().fg(ACCENT)),
            Span::styled("ssh-menu ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(format!("v{}", env!("CARGO_PKG_VERSION")),
                Style::default().fg(MUTED)),
        ]),
        Line::from(Span::styled("     交互式 TUI SSH 连接管理器",
            Style::default().fg(TEXT))),
        Line::from(vec![
            Span::styled("  👤 ", Style::default().fg(MUTED)),
            Span::styled("作者  ", Style::default().fg(MUTED)),
            Span::styled("Aidan-996", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  🔗 ", Style::default().fg(MUTED)),
            Span::styled("仓库  ", Style::default().fg(MUTED)),
            Span::styled("github.com/Aidan-996/ssh-menu",
                Style::default().fg(INFO).add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from(vec![
            Span::styled("  📄 ", Style::default().fg(MUTED)),
            Span::styled("许可  ", Style::default().fg(MUTED)),
            Span::styled("MIT © 2026",
                Style::default().fg(TEXT)),
        ]),
    ]
}

fn draw_about(f: &mut ratatui::Frame, area: Rect) {
    let mut lines: Vec<Line> = vec![Line::from("")];

    let banner = [
        " ╔═══════════════════════════╗",
        " ║   S S H   ·   M E N U     ║",
        " ╚═══════════════════════════╝",
    ];
    for l in banner {
        lines.push(Line::from(Span::styled(
            format!("  {}", l),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::from(""));
    for l in about_lines() { lines.push(l); }

    lines.push(Line::from(""));
    lines.push(section_title("SHORTCUTS"));
    for (k, desc) in SHORTCUTS_SHORT {
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(format!(" {} ", k), Style::default().fg(BG).bg(WARNING).add_modifier(Modifier::BOLD)),
            Span::styled("  ", Style::default()),
            Span::styled(*desc, Style::default().fg(TEXT)),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  按 ? 查看完整帮助 · 按 a 添加主机",
        Style::default().fg(MUTED).italic(),
    )));

    let title = Line::from(vec![
        Span::styled(" ◈ 主机详情 ", Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(panel_accent(ACCENT2).title(title)),
        area,
    );
}

fn draw_footer(f: &mut ratatui::Frame, area: Rect, app: &App) {
    // Build shortcut pills for the footer in normal mode.
    if matches!(app.mode, Mode::Normal) {
        let pills: Vec<Span> = [
            (" ⏎ ",   "连接",    SUCCESS),
            (" a ",   "添加",    ACCENT),
            (" e ",   "编辑",    ACCENT),
            (" D ",   "删除",    DANGER),
            (" / ",   "搜索",    ACCENT2),
            (" s ",   "排序",    ACCENT2),
            (" i ",   "详情",    ACCENT2),
            (" ? ",   "帮助",    WARNING),
            (" q ",   "退出",    MUTED),
        ].iter().flat_map(|(key, label, color)| {
            vec![
                Span::styled(" ", Style::default()),
                Span::styled(*key, Style::default().fg(BG).bg(*color).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" {} ", label), Style::default().fg(TEXT)),
            ]
        }).collect();

        f.render_widget(
            Paragraph::new(Line::from(pills)).block(panel()),
            area,
        );
        return;
    }

    let (text, color) = match &app.mode {
        Mode::Search => (
            " 输入过滤 · Ctrl-U 清空 · ↑/↓ 移动 · Enter 若只剩 1 条则连接 · Esc 返回".to_string(),
            ACCENT,
        ),
        Mode::Form(_) => (
            " Tab/↑/↓ 切字段 · Ctrl-U 清空字段 · Enter 或 Ctrl-S 保存 · Esc 取消".to_string(),
            ACCENT,
        ),
        Mode::Confirm(m, _) => (format!(" ⚠  {}", m), DANGER),
        Mode::Help => (" 按任意键关闭帮助".into(), ACCENT),
        Mode::Normal => (String::new(), TEXT), // unreachable
    };
    f.render_widget(
        Paragraph::new(text)
            .style(Style::default().fg(color))
            .block(panel_accent(color)),
        area,
    );
}

fn draw_form(f: &mut ratatui::Frame, app: &App) {
    let Mode::Form(fs) = &app.mode else { return; };
    let area = centered_rect(70, 80, f.area());
    f.render_widget(Clear, area);

    let mut lines: Vec<Line> = vec![];
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            if fs.editing_index.is_some() { " ✎  编辑主机 " } else { " ✚  添加主机 " },
            Style::default().fg(BG).bg(ACCENT).add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(Span::styled(
        "    * 号标记的字段为必填",
        Style::default().fg(MUTED).italic(),
    )));
    lines.push(Line::from(""));

    for (i, (k, v)) in fs.fields.iter().enumerate() {
        let active = i == fs.cursor;
        let marker = if active { " ▶ " } else { "   " };
        let required = matches!(k.as_str(), "name" | "host");
        let star = if required { "*" } else { " " };

        let label_style = if active {
            Style::default().fg(WARNING).add_modifier(Modifier::BOLD)
        } else if required {
            Style::default().fg(TEXT)
        } else {
            Style::default().fg(MUTED)
        };

        let value_span = if v.is_empty() && !active {
            Span::styled("（空）", Style::default().fg(MUTED).italic())
        } else if active {
            Span::styled(v.clone(), Style::default().fg(TEXT).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(v.clone(), Style::default().fg(TEXT))
        };

        lines.push(Line::from(vec![
            Span::styled(marker, label_style),
            Span::styled(format!("{}{:<6}", star, k), label_style),
            Span::styled(" │ ", Style::default().fg(Color::Rgb(40, 48, 72))),
            value_span,
            Span::styled(if active { "▎" } else { "" }, Style::default().fg(WARNING)),
        ]));
        if active {
            lines.push(Line::from(vec![
                Span::styled("              ", Style::default()),
                Span::styled("💡 ", Style::default().fg(MUTED)),
                Span::styled(field_hint(k), Style::default().fg(MUTED).italic()),
            ]));
        }
    }

    let title = Line::from(vec![
        Span::styled(" ✎ 主机编辑器 ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(panel_accent(ACCENT).title(title)),
        area,
    );
}

fn draw_help(f: &mut ratatui::Frame) {
    let area = centered_rect(68, 82, f.area());
    f.render_widget(Clear, area);

    let section = |s: &str| Line::from(vec![
        Span::styled("  ▸ ", Style::default().fg(ACCENT2)),
        Span::styled(s.to_string(), Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" {}", "─".repeat(20)), Style::default().fg(Color::Rgb(40, 48, 72))),
    ]);
    let key = |k: &str, desc: &str| Line::from(vec![
        Span::styled("    ", Style::default()),
        Span::styled(format!(" {} ", k), Style::default().fg(BG).bg(WARNING).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
        Span::styled(desc.to_string(), Style::default().fg(TEXT)),
    ]);

    let lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("   ", Style::default()),
            Span::styled(" ssh-menu ", Style::default().fg(BG).bg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("  快捷键参考", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        section("移动"),
        key("↑/↓ j/k",    "上下移动"),
        key("PgUp/PgDn",  "翻 10 行"),
        key("g / G",      "首项 / 末项"),
        key("Home/End",   "首项 / 末项"),
        key("1-9",        "跳到第 N 个可见项"),
        key("a-z",        "跳到下一个首字母匹配的主机"),
        Line::from(""),
        section("动作"),
        key("Enter",      "连接当前选中"),
        key("a",          "添加主机"),
        key("e",          "编辑当前选中"),
        key("D",          "删除（Shift+d，二次确认）"),
        key("y",          "状态栏显示等效 ssh 命令"),
        Line::from(""),
        section("视图"),
        key("/",          "进入搜索模式"),
        key("s",          "切换排序：名称→分组→最近→最多"),
        key("i",          "切换详情面板"),
        key("r",          "刷新 / 重新过滤"),
        key("?",          "切换本帮助"),
        Line::from(""),
        section("退出"),
        key("q / Esc",    "退出（或关闭浮层）"),
        key("Ctrl-C",     "强制退出"),
        Line::from(""),
        section("搜索模式"),
        key("任意字符",   "加入过滤条件"),
        key("Backspace",  "删除最后一个字符"),
        key("Ctrl-U",     "清空过滤条件"),
        key("Enter",      "仅剩 1 条时直接连接"),
        key("Esc",        "清空并返回普通模式"),
        Line::from(""),
        section("表单模式"),
        key("Tab / ↓",    "下一字段"),
        key("Shift-Tab",  "上一字段"),
        key("Ctrl-U",     "清空当前字段"),
        key("Enter",      "保存并关闭"),
        key("Esc",        "取消"),
        Line::from(""),
        Line::from(Span::styled(
            "   按 ?、Esc、Enter 或 q 关闭帮助",
            Style::default().fg(MUTED).italic(),
        )),
    ];

    let title = Line::from(vec![
        Span::styled(" ? 帮助 ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(panel_accent(ACCENT).title(title)),
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
