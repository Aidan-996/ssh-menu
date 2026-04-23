use crate::config::Host;

pub struct FormState {
    pub editing_index: Option<usize>,
    pub fields: Vec<(String, String)>,
    pub cursor: usize,
}

impl FormState {
    pub fn new(h: Option<&Host>) -> Self {
        let def = |v: &str| v.to_string();
        let h_def = Host {
            name: String::new(), host: String::new(),
            user: "root".into(), port: 22,
            key: None, group: None, tags: vec![],
            jump: None, note: None, extra: vec![],
            last_used: None, use_count: 0,
        };
        let src = h.unwrap_or(&h_def);
        Self {
            editing_index: None,
            fields: vec![
                ("name".into(), def(&src.name)),
                ("host".into(), def(&src.host)),
                ("user".into(), def(&src.user)),
                ("port".into(), src.port.to_string()),
                ("key".into(), src.key.clone().unwrap_or_default()),
                ("group".into(), src.group.clone().unwrap_or_default()),
                ("tags".into(), src.tags.join(",")),
                ("jump".into(), src.jump.clone().unwrap_or_default()),
                ("note".into(), src.note.clone().unwrap_or_default()),
            ],
            cursor: 0,
        }
    }

    pub fn to_host(&self) -> Option<Host> {
        let get = |i: usize| self.fields[i].1.clone();
        let name = get(0);
        let host = get(1);
        if name.is_empty() || host.is_empty() { return None; }
        let port: u16 = get(3).parse().unwrap_or(22);
        let key = { let v = get(4); if v.is_empty() { None } else { Some(v) } };
        let group = { let v = get(5); if v.is_empty() { None } else { Some(v) } };
        let tags: Vec<String> = get(6).split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        let jump = { let v = get(7); if v.is_empty() { None } else { Some(v) } };
        let note = { let v = get(8); if v.is_empty() { None } else { Some(v) } };
        Some(Host {
            name, host,
            user: { let v = get(2); if v.is_empty() { "root".into() } else { v } },
            port, key, group, tags, jump, note, extra: vec![],
            last_used: None, use_count: 0,
        })
    }
}
