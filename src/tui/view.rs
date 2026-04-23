use super::app::{App, Mode};
use crate::ssh;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

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

// Stable palette — swatches for group coloring (auto-assigned by hash).
const GROUP_PALETTE: &[Color] = &[
    Color::LightMagenta, Color::LightYellow, Color::LightCyan,
    Color::LightGreen, Color::LightBlue, Color::LightRed,
    Color::Magenta, Color::Yellow, Color::Cyan,
];

fn color_for_group(g: &str) -> Color {
    let mut h: u32 = 5381;
    for b in g.as_bytes() { h = h.wrapping_mul(33).wrapping_add(*b as u32); }
    GROUP_PALETTE[(h as usize) % GROUP_PALETTE.len()]
}

fn rounded_block<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}

pub fn draw(f: &mut ratatui::Frame, app: &mut App) {
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
    let accent = Color::Rgb(127, 219, 255); // soft cyan
    let dim = Color::Rgb(110, 110, 110);

    let left = match &app.mode {
        Mode::Search => Line::from(vec![
            Span::styled("🔍  ", Style::default().fg(accent)),
            Span::styled("/ ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::styled(&app.query, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("▎", Style::default().fg(accent).add_modifier(Modifier::SLOW_BLINK)),
        ]),
        _ => if app.query.is_empty() {
            Line::from(vec![
                Span::styled(" 🔌 ", Style::default().fg(accent)),
                Span::styled("ssh-menu", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  v{}", env!("CARGO_PKG_VERSION")), Style::default().fg(dim)),
                Span::styled("   按 ", Style::default().fg(dim)),
                Span::styled("?", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" 查看帮助", Style::default().fg(dim)),
            ])
        } else {
            Line::from(vec![
                Span::styled(" 筛选: ", Style::default().fg(dim)),
                Span::styled(&app.query, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ])
        },
    };

    let right_text = format!(
        "  {} 主机 · 排序 {} · 详情 {} ",
        app.cfg.hosts.len(),
        app.sort_by.label(),
        if app.show_details { "●" } else { "○" },
    );

    let title = Line::from(vec![
        Span::styled("╼ ", Style::default().fg(accent)),
        Span::styled("SSH Menu", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        Span::styled(right_text, Style::default().fg(dim)),
        Span::styled("╾", Style::default().fg(accent)),
    ]);

    f.render_widget(
        Paragraph::new(left).block(
            rounded_block()
                .border_style(Style::default().fg(accent))
                .title(title),
        ),
        area,
    );
}

fn draw_list(f: &mut ratatui::Frame, area: Rect, app: &mut App) {
    if app.filtered.is_empty() {
        let msg = if app.cfg.hosts.is_empty() {
            vec![
                Line::from(""),
                Line::from(Span::styled("  ✨  还没有主机", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  按 ", Style::default()),
                    Span::styled("a", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(" 添加第一台主机", Style::default()),
                ]),
                Line::from(vec![
                    Span::styled("  或运行 ", Style::default()),
                    Span::styled("ssh-menu import", Style::default().fg(Color::Green)),
                    Span::styled(" 从 ~/.ssh/config 导入", Style::default()),
                ]),
                Line::from(""),
                Line::from(Span::styled("  按 ? 查看快捷键", Style::default().fg(Color::DarkGray).italic())),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from(Span::styled("  😶  没有匹配的主机", Style::default().fg(Color::Yellow))),
                Line::from(Span::styled(format!("  筛选词：{:?}", app.query), Style::default().fg(Color::DarkGray))),
                Line::from(""),
                Line::from(Span::styled("  按 Esc 清空筛选", Style::default().fg(Color::DarkGray).italic())),
            ]
        };
        f.render_widget(
            Paragraph::new(msg).block(rounded_block().title(" 主机列表 ")),
            area,
        );
        return;
    }

    let accent = Color::Rgb(127, 219, 255);
    let items: Vec<ListItem> = app.filtered.iter().enumerate().filter_map(|(vis_idx, i)| {
        let h = app.cfg.hosts.get(*i)?;
        let num_str = if vis_idx < 9 { format!("{}", vis_idx + 1) } else { "·".into() };
        let num = format!(" {:>2} ", num_str);
        let group_raw = h.group.as_deref().unwrap_or("-");
        let group_col = if group_raw == "-" { Color::DarkGray } else { color_for_group(group_raw) };
        let group = format!("{:<10}", truncate(group_raw, 10));
        let name = format!("{:<20}", truncate(&h.name, 20));
        let conn = if h.port == 22 {
            format!("{}@{}", h.user, h.host)
        } else {
            format!("{}@{}:{}", h.user, h.host, h.port)
        };
        let jump = h.jump.as_deref().map(|j| format!(" ↪ {}", j)).unwrap_or_default();
        let tags = if h.tags.is_empty() { String::new() } else { format!("  #{}", h.tags.join(" #")) };
        let uses = if h.use_count > 0 { format!("  ×{}", h.use_count) } else { String::new() };

        let mut spans = vec![
            Span::styled(num, Style::default().fg(Color::DarkGray)),
            Span::styled(group, Style::default().fg(group_col)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled(name, Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::styled(conn, Style::default().fg(Color::LightGreen)),
        ];
        if !jump.is_empty() {
            spans.push(Span::styled(jump, Style::default().fg(Color::Yellow)));
        }
        if !tags.is_empty() {
            spans.push(Span::styled(tags, Style::default().fg(Color::Blue).italic()));
        }
        if !uses.is_empty() {
            spans.push(Span::styled(uses, Style::default().fg(Color::DarkGray)));
        }
        Some(ListItem::new(Line::from(spans)))
    }).collect();

    let title = Line::from(vec![
        Span::styled(" 主机列表 ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        Span::styled(format!("({}) ", app.filtered.len()), Style::default().fg(Color::DarkGray)),
    ]);
    let list = List::new(items)
        .block(rounded_block().border_style(Style::default().fg(Color::DarkGray)).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(30, 50, 80))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ▶ ");
    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_details(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let accent = Color::Rgb(127, 219, 255);
    let Some(h) = app.selected_host() else {
        f.render_widget(rounded_block().title(" 详情 "), area);
        return;
    };

    let mut lines: Vec<Line> = vec![];
    let kv = |k: &str, v: String| Line::from(vec![
        Span::styled(format!(" {:<8}", k), Style::default().fg(Color::DarkGray)),
        Span::styled("  ", Style::default()),
        Span::raw(v),
    ]);
    let kv_color = |k: &str, v: String, c: Color, bold: bool| {
        let mut st = Style::default().fg(c);
        if bold { st = st.add_modifier(Modifier::BOLD); }
        Line::from(vec![
            Span::styled(format!(" {:<8}", k), Style::default().fg(Color::DarkGray)),
            Span::styled("  ", Style::default()),
            Span::styled(v, st),
        ])
    };

    lines.push(Line::from(""));
    lines.push(kv_color("name", h.name.clone(), accent, true));
    lines.push(kv("host", h.host.clone()));
    lines.push(kv("user", h.user.clone()));
    lines.push(kv("port", h.port.to_string()));
    if let Some(k) = &h.key { lines.push(kv("key", k.clone())); }
    if let Some(g) = &h.group {
        lines.push(kv_color("group", g.clone(), color_for_group(g), true));
    }
    if !h.tags.is_empty() {
        lines.push(kv_color("tags", format!("#{}", h.tags.join(" #")), Color::Blue, false));
    }
    if let Some(j) = &h.jump { lines.push(kv_color("jump", j.clone(), Color::Yellow, true)); }
    if !h.extra.is_empty() { lines.push(kv("extra", h.extra.join(" "))); }
    if let Some(n) = &h.note { lines.push(kv("note", n.clone())); }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" ─ 使用统计 ──────────", Style::default().fg(Color::DarkGray))));
    lines.push(kv("次数", h.use_count.to_string()));
    lines.push(kv("最近", h.last_used.as_deref().map(ssh::time_ago).unwrap_or_else(|| "从未".into())));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" ─ ssh 命令 ────────────", Style::default().fg(Color::DarkGray))));
    let args = ssh::build_ssh_args(&app.cfg, h);
    lines.push(Line::from(vec![
        Span::styled(" $ ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("ssh {}", args.join(" ")),
            Style::default().fg(Color::LightGreen)),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("   按 Enter 连接 · y 复制到状态栏 · e 编辑",
        Style::default().fg(Color::DarkGray).italic())));

    let title = Line::from(vec![
        Span::styled(" 详情 ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(rounded_block().border_style(Style::default().fg(Color::DarkGray)).title(title)),
        area,
    );
}

fn draw_footer(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let (text, color) = match &app.mode {
        Mode::Search => (
            " 输入过滤 · Ctrl-U 清空 · ↑/↓ 移动 · Enter 若只剩 1 条则连接 · Esc 返回".to_string(),
            Color::Rgb(127, 219, 255),
        ),
        Mode::Form(_) => (
            " Tab/↑/↓ 切字段 · Ctrl-U 清空字段 · Enter 或 Ctrl-S 保存 · Esc 取消".to_string(),
            Color::Rgb(127, 219, 255),
        ),
        Mode::Confirm(m, _) => (format!(" {}", m), Color::LightRed),
        Mode::Help => (" 按任意键关闭帮助".into(), Color::Rgb(127, 219, 255)),
        Mode::Normal => (format!(" {}", app.status), Color::Yellow),
    };
    f.render_widget(
        Paragraph::new(text)
            .style(Style::default().fg(color))
            .block(rounded_block().border_style(Style::default().fg(Color::DarkGray))),
        area,
    );
}

fn draw_form(f: &mut ratatui::Frame, app: &App) {
    let Mode::Form(fs) = &app.mode else { return; };
    let accent = Color::Rgb(127, 219, 255);
    let area = centered_rect(70, 80, f.area());
    f.render_widget(Clear, area);

    let mut lines: Vec<Line> = vec![];
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            if fs.editing_index.is_some() { "✎  编辑主机" } else { "✚  添加主机" },
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(Span::styled(
        "    * 号标记的字段为必填",
        Style::default().fg(Color::DarkGray).italic(),
    )));
    lines.push(Line::from(""));

    for (i, (k, v)) in fs.fields.iter().enumerate() {
        let active = i == fs.cursor;
        let marker = if active { " ▶ " } else { "   " };
        let required = matches!(k.as_str(), "name" | "host");
        let star = if required { "*" } else { " " };

        let label_style = if active {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if required {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Gray)
        };

        let value_span = if v.is_empty() && !active {
            Span::styled("（空）", Style::default().fg(Color::DarkGray).italic())
        } else if active {
            Span::styled(v.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        } else {
            Span::raw(v.clone())
        };

        lines.push(Line::from(vec![
            Span::styled(marker, label_style),
            Span::styled(format!("{}{:<6}", star, k), label_style),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            value_span,
            Span::styled(if active { "▎" } else { "" }, Style::default().fg(Color::Yellow)),
        ]));
        if active {
            lines.push(Line::from(vec![
                Span::styled("              ", Style::default()),
                Span::styled("💡 ", Style::default().fg(Color::DarkGray)),
                Span::styled(field_hint(k), Style::default().fg(Color::DarkGray).italic()),
            ]));
        }
    }

    let title = Line::from(vec![
        Span::styled(" 主机编辑器 ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(rounded_block().border_style(Style::default().fg(accent)).title(title)),
        area,
    );
}

fn draw_help(f: &mut ratatui::Frame) {
    let accent = Color::Rgb(127, 219, 255);
    let area = centered_rect(68, 82, f.area());
    f.render_widget(Clear, area);

    let section = |s: &str| Line::from(Span::styled(
        format!("  ─ {} ──────────────────", s),
        Style::default().fg(accent).add_modifier(Modifier::BOLD),
    ));
    let key = |k: &str, desc: &str| Line::from(vec![
        Span::styled(format!("   {:<12}", k), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
        Span::raw(desc.to_string()),
    ]);

    let lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "   ssh-menu — 快捷键参考",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
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
        key("D (Shift+d)","删除（二次确认）"),
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
        key("Shift-Tab ↑","上一字段"),
        key("Ctrl-U",     "清空当前字段"),
        key("Enter/Ctrl-S","保存并关闭"),
        key("Esc",        "取消"),
        Line::from(""),
        Line::from(Span::styled(
            "   按 ?、Esc、Enter 或 q 关闭帮助",
            Style::default().fg(Color::DarkGray).italic(),
        )),
    ];

    let title = Line::from(vec![
        Span::styled(" ? 帮助 ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(rounded_block().border_style(Style::default().fg(accent)).title(title)),
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
