//! Aetheris interactive chat TUI.
//!
//! Talks to a local FastAPI mock server (Puter-style) exposing:
//! - `POST /chat/completions` -> OpenAI-style completion (non-streaming)
//! - `POST /stream` -> newline-delimited streaming chunks (SSE-ish)
//! - `GET /models` -> available model ids
//!
//! Rendering uses ratatui + crossterm (plain ANSI, SSH-safe). No `println!`
//! is used after the terminal enters raw mode so the UI is never corrupted.

use std::io::{self, Stdout};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use futures_util::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use serde::Deserialize;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

/// A selectable model: friendly label -> model id.
#[derive(Clone, Copy)]
struct Model {
    label: &'static str,
    id: &'static str,
}

const MODELS: [Model; 5] = [
    Model {
        label: "Fable 5",
        id: "puter/claude-fable-5",
    },
    Model {
        label: "Fable 5.1",
        id: "puter/claude-fable-5.1",
    },
    Model {
        label: "Claude latest",
        id: "puter/claude-fable-5.1",
    },
    Model {
        label: "Sonnet latest",
        id: "puter/sonnet-5",
    },
    Model {
        label: "Opus 5",
        id: "puter/opus-5",
    },
];

const DEFAULT_MODEL_INDEX: usize = 1;

/// Multi-line text editor used for the prompt input box.
#[derive(Default)]
struct TextEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            lines: vec![String::new()],
            ..Self::default()
        }
    }

    /// Full contents of the editor joined by newlines.
    fn text(&self) -> String {
        self.lines.join("\n")
    }

    fn is_empty(&self) -> bool {
        self.lines.iter().all(|l| l.is_empty())
    }

    fn insert_char(&mut self, ch: char) {
        let byte_idx = self.char_to_byte(self.cursor_row, self.cursor_col);
        self.lines[self.cursor_row].insert(byte_idx, ch);
        self.cursor_col += 1;
    }

    #[allow(dead_code)]
    fn insert_newline(&mut self) {
        let byte_idx = self.char_to_byte(self.cursor_row, self.cursor_col);
        let rest = self.lines[self.cursor_row].split_off(byte_idx);
        self.cursor_row += 1;
        self.cursor_col = 0;
        self.lines.insert(self.cursor_row, rest);
    }

    fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let byte_idx = self.char_to_byte(self.cursor_row, self.cursor_col);
            let start = self.char_to_byte(self.cursor_row, self.cursor_col - 1);
            self.lines[self.cursor_row].replace_range(start..byte_idx, "");
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            let prev_len = self.lines[self.cursor_row - 1].chars().count();
            let rest = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = prev_len;
            self.lines[self.cursor_row].push_str(&rest);
        }
    }

    fn delete(&mut self) {
        let len = self.lines[self.cursor_row].chars().count();
        if self.cursor_col < len {
            let byte_idx = self.char_to_byte(self.cursor_row, self.cursor_col);
            let next = self.char_to_byte(self.cursor_row, self.cursor_col + 1);
            self.lines[self.cursor_row].replace_range(byte_idx..next, "");
        } else if self.cursor_row + 1 < self.lines.len() {
            let rest = self.lines.remove(self.cursor_row + 1);
            self.lines[self.cursor_row].push_str(&rest);
        }
    }

    fn cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    fn cursor_right(&mut self) {
        let len = self.lines[self.cursor_row].chars().count();
        if self.cursor_col < len {
            self.cursor_col += 1;
        }
    }

    fn cursor_home(&mut self) {
        self.cursor_col = 0;
    }

    fn cursor_end(&mut self) {
        self.cursor_col = self.lines[self.cursor_row].chars().count();
        self.cursor_row = self.lines.len() - 1;
    }

    fn clear(&mut self) {
        self.lines.clear();
        self.lines.push(String::new());
        self.cursor_row = 0;
        self.cursor_col = 0;
    }

    fn char_to_byte(&self, row: usize, col: usize) -> usize {
        self.lines[row]
            .char_indices()
            .nth(col)
            .map_or(self.lines[row].len(), |(i, _)| i)
    }
}

#[derive(Clone)]
enum Role {
    User,
    Assistant,
    Error,
}

#[derive(Clone)]
struct ChatMessage {
    role: Role,
    model: String,
    text: String,
}

/// An in-flight request whose response is pending / streaming.
struct Inflight {
    id: u64,
    model_label: String,
    buffer: String,
    streaming: bool,
}

/// Events delivered from background async tasks back to the UI loop.
enum AppEvent {
    Response {
        id: u64,
        result: Result<String, String>,
    },
    StreamChunk {
        id: u64,
        text: String,
    },
    StreamDone {
        id: u64,
        result: Result<(), String>,
    },
    ServerStatus {
        ok: bool,
    },
}

struct ChatApp {
    base_url: String,
    http: reqwest::Client,
    selected: usize,
    messages: Vec<ChatMessage>,
    inflight: Option<Inflight>,
    editor: TextEditor,
    streaming: bool,
    follow: bool,
    conv_scroll: usize,
    server_ok: bool,
    next_id: u64,
    ev_rx: UnboundedReceiver<Event>,
    app_rx: UnboundedReceiver<AppEvent>,
    app_tx: UnboundedSender<AppEvent>,
}

#[derive(Parser)]
#[command(name = "aetheris-chat", about = "Aetheris interactive chat TUI")]
struct Args {
    /// Base URL of the local Puter-style model server.
    #[arg(long, default_value = "http://127.0.0.1:8008")]
    base_url: String,
}

impl ChatApp {
    fn new(args: Args) -> Result<Self> {
        let base_url = args.base_url.trim_end_matches('/').to_string();
        let (ev_tx, ev_rx) = mpsc::unbounded_channel::<Event>();
        // Read terminal events on a dedicated thread; the event loop stays async.
        let ev_tx2 = ev_tx.clone();
        std::thread::spawn(move || loop {
            // Poll with a short timeout so the thread can detect a closed channel.
            match event::poll(Duration::from_millis(100)) {
                Ok(true) => match event::read() {
                    Ok(ev) => {
                        if ev_tx2.send(ev).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                },
                Ok(false) => continue,
                Err(_) => break,
            }
        });
        // Keep the original sender alive so the loop receives until UI exit.
        let _leak_sender = ev_tx;

        let (app_tx, app_rx) = mpsc::unbounded_channel::<AppEvent>();
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("build HTTP client")?;

        let app = Self {
            base_url,
            http,
            selected: DEFAULT_MODEL_INDEX,
            messages: Vec::new(),
            inflight: None,
            editor: TextEditor::new(),
            streaming: false,
            follow: true,
            conv_scroll: 0,
            server_ok: true,
            next_id: 0,
            ev_rx,
            app_rx,
            app_tx,
        };
        Ok(app)
    }

    /// Non-blocking probe of GET /models; result arrives as an AppEvent.
    fn probe_server(&mut self) {
        let tx = self.app_tx.clone();
        let url = self.base_url.clone();
        let client = self.http.clone();
        tokio::spawn(async move {
            let ok = client
                .get(format!("{url}/models"))
                .send()
                .await
                .map(|r| r.status().is_success())
                .unwrap_or(false);
            let _ = tx.send(AppEvent::ServerStatus { ok });
        });
    }

    async fn send_pending(&mut self) {
        if self.inflight.is_some() {
            return;
        }
        let prompt = self.editor.text();
        if prompt.trim().is_empty() {
            return;
        }
        let model_idx = self.selected;
        let model_id = MODELS[model_idx].id.to_string();
        let model_label = MODELS[model_idx].label.to_string();
        self.messages.push(ChatMessage {
            role: Role::User,
            model: model_label.clone(),
            text: prompt.clone(),
        });
        self.editor.clear();
        self.follow = true;

        let id = self.next_id;
        self.next_id += 1;
        self.inflight = Some(Inflight {
            id,
            model_label,
            buffer: String::new(),
            streaming: self.streaming,
        });

        let tx = self.app_tx.clone();
        let url = self.base_url.clone();
        let client = self.http.clone();
        if self.streaming {
            tokio::spawn(async move {
                let result = stream_chat(&client, &url, &model_id, &prompt, id, &tx).await;
                let _ = tx.send(AppEvent::StreamDone {
                    id,
                    result: result.map_err(|e| e.to_string()),
                });
            });
        } else {
            tokio::spawn(async move {
                let result = send_chat(&client, &url, &model_id, &prompt).await;
                let _ = tx.send(AppEvent::Response {
                    id,
                    result: result.map_err(|e| e.to_string()),
                });
            });
        }
    }

    fn finalize_inflight(&mut self) {
        if let Some(inf) = self.inflight.take() {
            if !inf.buffer.trim().is_empty() {
                self.messages.push(ChatMessage {
                    role: Role::Assistant,
                    model: inf.model_label,
                    text: inf.buffer,
                });
            }
        }
        self.follow = true;
    }

    fn fail_inflight(&mut self, err: String) {
        if let Some(inf) = self.inflight.take() {
            self.messages.push(ChatMessage {
                role: Role::Error,
                model: inf.model_label,
                text: err,
            });
        }
        self.follow = true;
    }

    fn finalize_inflight_with_error(&mut self, err: String) {
        if let Some(inf) = self.inflight.take() {
            if !inf.buffer.trim().is_empty() {
                self.messages.push(ChatMessage {
                    role: Role::Assistant,
                    model: inf.model_label.clone(),
                    text: inf.buffer,
                });
            }
            self.messages.push(ChatMessage {
                role: Role::Error,
                model: inf.model_label,
                text: err,
            });
        }
        self.follow = true;
    }

    fn handle_app_event(&mut self, ev: AppEvent) {
        match ev {
            AppEvent::ServerStatus { ok } => {
                self.server_ok = ok;
            }
            AppEvent::Response { id, result } => {
                let Some(inf) = self.inflight.as_mut() else {
                    return;
                };
                if inf.id != id {
                    return;
                }
                match result {
                    Ok(text) => {
                        inf.buffer = text;
                        self.finalize_inflight();
                    }
                    Err(e) => self.fail_inflight(e),
                }
            }
            AppEvent::StreamChunk { id, text } => {
                let Some(inf) = self.inflight.as_mut() else {
                    return;
                };
                if inf.id != id {
                    return;
                }
                inf.buffer.push_str(&text);
            }
            AppEvent::StreamDone { id, result } => {
                let Some(inf) = self.inflight.as_mut() else {
                    return;
                };
                if inf.id != id {
                    return;
                }
                match result {
                    Ok(()) => self.finalize_inflight(),
                    Err(e) => {
                        if inf.buffer.trim().is_empty() {
                            self.fail_inflight(e);
                        } else {
                            self.finalize_inflight_with_error(e);
                        }
                    }
                }
            }
        }
    }

    async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q')
                if key.modifiers.is_empty() && self.editor.is_empty() =>
            {
                return Ok(());
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return Ok(());
            }
            KeyCode::Up => {
                self.selected = (self.selected + MODELS.len() - 1) % MODELS.len();
            }
            KeyCode::Down => {
                self.selected = (self.selected + 1) % MODELS.len();
            }
            KeyCode::Enter => {
                self.send_pending().await;
            }
            KeyCode::Char('/') if self.editor.is_empty() => {
                self.streaming = !self.streaming;
            }
            KeyCode::Char('s') if self.editor.is_empty() => {
                self.streaming = !self.streaming;
            }
            KeyCode::Char(ch) => {
                self.editor.insert_char(ch);
            }
            KeyCode::Home => self.editor.cursor_home(),
            KeyCode::End => self.editor.cursor_end(),
            KeyCode::Backspace => self.editor.backspace(),
            KeyCode::Delete => self.editor.delete(),
            KeyCode::Left => self.editor.cursor_left(),
            KeyCode::Right => self.editor.cursor_right(),
            _ => {}
        }
        Ok(())
    }

    fn conversation_lines(&self) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = Vec::new();
        for m in &self.messages {
            let (role_name, color) = match m.role {
                Role::User => ("You", Color::Green),
                Role::Assistant => ("assistant", Color::Cyan),
                Role::Error => ("error", Color::Red),
            };
            let header = Line::from(vec![Span::styled(
                format!("[{role_name} · {}] ", m.model),
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD),
            )]);
            lines.push(header);
            for part in m.text.split('\n') {
                lines.push(Line::from(part.to_string()));
            }
            lines.push(Line::from(String::new()));
        }
        if let Some(inf) = &self.inflight {
            let pending_label = if inf.buffer.is_empty() {
                "thinking…".to_string()
            } else if inf.streaming {
                "streaming…".to_string()
            } else {
                "waiting…".to_string()
            };
            let header = Line::from(vec![Span::styled(
                format!("[assistant · {}] {pending_label}", inf.model_label),
                Style::default().fg(Color::Cyan),
            )]);
            lines.push(header);
            for part in inf.buffer.split('\n') {
                lines.push(Line::from(part.to_string()));
            }
        }
        lines
    }

    fn input_rows(&self, width: usize) -> (Vec<String>, Option<(usize, usize)>) {
        let mut rows: Vec<String> = Vec::new();
        let mut cursor_pos: Option<(usize, usize)> = None;
        for (li, line) in self.editor.lines.iter().enumerate() {
            let wrapped = wrap_to_width(line, width);
            let n = wrapped.len();
            let base = rows.len();
            if li == self.editor.cursor_row {
                let chars_before = prefix_char_count(line, self.editor.cursor_col);
                let (row_off, col) = if width > 0 {
                    (chars_before.checked_div(width).unwrap_or(0), chars_before % width)
                } else {
                    (0, chars_before)
                };
                let row_off = row_off.min(n.saturating_sub(1));
                cursor_pos = Some((base + row_off, col));
            }
            rows.extend(wrapped);
        }
        if rows.is_empty() {
            rows.push(String::new());
            cursor_pos = Some((0, 0));
        }
        (rows, cursor_pos)
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [title_area, model_area, conv_area, input_area, status_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(5),
                Constraint::Length(1),
            ])
            .areas::<5>(frame.size());

        let title = Paragraph::new(Line::from(vec![Span::styled(
            "Aetheris Chat",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Aetheris "));
        frame.render_widget(title, title_area);

        // Model selector.
        let mut spans: Vec<Span<'static>> = Vec::new();
        for (i, m) in MODELS.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("   "));
            }
            let selected = i == self.selected;
            let style = if selected {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            spans.push(Span::styled(format!("{} ({})", m.label, m.id), style));
        }
        let model_par = Paragraph::new(Line::from(spans))
            .block(Block::default().borders(Borders::ALL).title(" Model (↑/↓) "));
        frame.render_widget(model_par, model_area);

        // Conversation.
        let conv_lines = self.conversation_lines();
        let conv_inner_h = conv_area.height.saturating_sub(2) as usize;
        let conv_inner_w = conv_area.width.saturating_sub(2) as usize;
        if self.follow {
            let content_h = approx_height(&conv_lines, conv_inner_w);
            self.conv_scroll = content_h.saturating_sub(conv_inner_h);
        }
        let conv_par = Paragraph::new(conv_lines)
            .block(Block::default().borders(Borders::ALL).title(" Conversation "))
            .wrap(Wrap { trim: false })
            .scroll((self.conv_scroll.min(u16::MAX as usize) as u16, 0));
        frame.render_widget(conv_par, conv_area);

        // Input.
        let input_inner_w = input_area.width.saturating_sub(2) as usize;
        let input_inner_h = input_area.height.saturating_sub(2) as usize;
        let (input_rows, cursor_pos) = self.input_rows(input_inner_w);
        let scroll_top = input_rows.len().saturating_sub(input_inner_h);
        let input_lines: Vec<Line<'static>> =
            input_rows.into_iter().map(Line::from).collect();
        let input_par = Paragraph::new(input_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Prompt (Enter to send) "),
            )
            .wrap(Wrap { trim: false })
            .scroll((scroll_top.min(u16::MAX as usize) as u16, 0));
        frame.render_widget(input_par, input_area);

        if let Some((cy, cx)) = cursor_pos {
            let cy = cy.saturating_sub(scroll_top);
            let x = input_area.x + 1 + cx as u16;
            let y = input_area.y + 1 + cy as u16;
            frame.set_cursor(x, y);
        }

        // Status / hints.
        let mode = if self.streaming { "STREAM" } else { "BATCH" };
        let server = if self.server_ok { "online" } else { "offline" };
        let state = if self.inflight.is_some() {
            "busy"
        } else {
            "ready"
        };
        let status = Line::from(vec![
            Span::styled(
                format!("mode:{mode}  model:{}  server:{server}  state:{state}  ",
                        MODELS[self.selected].id),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                "↑/↓ model · Enter send · / streaming · q quit · ctrl-c quit",
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        frame.render_widget(Paragraph::new(status), status_area);
    }

    async fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;
            tokio::select! {
                biased;
                ev = self.ev_rx.recv() => {
                    match ev {
                        Some(Event::Key(key)) => self.handle_key(key).await?,
                        Some(Event::Resize(_, _)) => {}
                        Some(_) => {}
                        None => return Ok(()),
                    }
                }
                app = self.app_rx.recv() => {
                    if let Some(ev) = app {
                        self.handle_app_event(ev);
                    }
                }
            }
        }
    }
}

/// Wrap a line of text into sub-rows of at most `width` characters.
fn wrap_to_width(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    let mut rows: Vec<String> = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if current.chars().count() == width {
            rows.push(std::mem::take(&mut current));
        }
        current.push(ch);
    }
    if !current.is_empty() || rows.is_empty() {
        rows.push(current);
    }
    rows
}

fn prefix_char_count(line: &str, char_idx: usize) -> usize {
    line.chars().take(char_idx).count()
}

/// Approximate rendered height (with wrapping) of a set of lines.
fn approx_height(lines: &[Line<'_>], width: usize) -> usize {
    if width == 0 {
        return lines.len();
    }
    lines
        .iter()
        .map(|l| {
            let chars: usize = l.spans.iter().map(|s| s.content.chars().count()).sum();
            chars.div_ceil(width).max(1)
        })
        .sum()
}

/// Response shape for the non-streaming endpoint.
#[derive(Deserialize)]
struct ChatCompletion {
    choices: Vec<CompletionChoice>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct CompletionChoice {
    finish_reason: Option<String>,
    text: String,
}

/// OpenAI-style non-streaming completion request.
async fn send_chat(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String> {
    let payload = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
    });
    let resp = client
        .post(format!("{base_url}/chat/completions"))
        .json(&payload)
        .send()
        .await
        .with_context(|| format!("POST {base_url}/chat/completions"))?;
    let status = resp.status();
    let body = resp
        .text()
        .await
        .context("read non-streaming response body")?;
    if !status.is_success() {
        return Err(anyhow!("HTTP {status}: {body}"));
    }
    let parsed: ChatCompletion = serde_json::from_str(&body)
        .map_err(|e| anyhow!("invalid completion payload: {e}: {body}"))?;
    parsed
        .choices
        .into_iter()
        .next()
        .map(|c| c.text)
        .ok_or_else(|| anyhow!("completion returned no choices"))
}

/// Streaming completion against POST /stream (newline-delimited chunks).
async fn stream_chat(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    prompt: &str,
    id: u64,
    tx: &UnboundedSender<AppEvent>,
) -> Result<()> {
    let payload = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
    });
    let resp = client
        .post(format!("{base_url}/stream"))
        .json(&payload)
        .send()
        .await
        .with_context(|| format!("POST {base_url}/stream"))?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".to_string());
        return Err(anyhow!("HTTP {status}: {body}"));
    }

    let mut stream = resp.bytes_stream();
    let mut pending: Vec<u8> = Vec::new();
    // Newline-delimited chunks (SSE-ish). Each line is a piece of the answer.
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("read stream chunk")?;
        pending.extend_from_slice(&chunk);
        while let Some(pos) = pending.iter().position(|&b| b == b'\n') {
            let mut line: Vec<u8> = pending.drain(..=pos).collect();
            if line.last() == Some(&b'\n') {
                line.pop();
            }
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            let raw = String::from_utf8_lossy(&line);
            let text = parse_stream_line(&raw);
            if let Some(text) = text {
                if tx.send(AppEvent::StreamChunk { id, text }).is_err() {
                    return Ok(());
                }
            }
        }
    }
    // Any trailing bytes without a newline.
    if !pending.is_empty() {
        let raw = String::from_utf8_lossy(&pending);
        if let Some(text) = parse_stream_line(&raw) {
            let _ = tx.send(AppEvent::StreamChunk { id, text });
        }
    }
    Ok(())
}

/// Extract the meaningful payload from one stream line, ignoring framing.
fn parse_stream_line(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let payload = trimmed.strip_prefix("data:").unwrap_or(trimmed).trim();
    if payload.is_empty() || payload == "[DONE]" {
        return None;
    }
    Some(payload.to_string())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut app = ChatApp::new(args)?;

    if let Err(e) = run_tui(&mut app) {
        // Best-effort terminal restore so the shell is usable again.
        let _ = disable_raw_mode();
        return Err(e);
    }
    Ok(())
}

fn run_tui(app: &mut ChatApp) -> Result<()> {
    let stdout = io::stdout();
    enable_raw_mode().context("enable raw mode")?;
    let mut out = stdout;
    execute!(out, EnterAlternateScreen).context("enter alternate screen")?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend).context("create terminal")?;
    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime")
        .block_on(async {
            app.probe_server();
            app.run_loop(&mut terminal).await
        });

    // Restore terminal regardless of how the loop ended.
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_is_fable_5_1() {
        assert_eq!(MODELS[DEFAULT_MODEL_INDEX].label, "Fable 5.1");
        assert_eq!(MODELS[DEFAULT_MODEL_INDEX].id, "puter/claude-fable-5.1");
    }

    #[test]
    fn streaming_line_parser_strips_framing() {
        assert_eq!(parse_stream_line("hello world\n"), Some("hello world".to_string()));
        assert_eq!(parse_stream_line("data: chunk"), Some("chunk".to_string()));
        assert_eq!(parse_stream_line("data:[DONE]"), None);
        assert_eq!(parse_stream_line("  "), None);
    }

    #[test]
    fn editor_basic_editing() {
        let mut ed = TextEditor::new();
        for c in "hey".chars() {
            ed.insert_char(c);
        }
        ed.cursor_home();
        ed.insert_char('x');
        assert_eq!(ed.text(), "xhey");
        ed.backspace();
        assert_eq!(ed.text(), "hey");
        ed.cursor_end();
        ed.insert_newline();
        ed.insert_char('z');
        assert_eq!(ed.text(), "hey\nz");
    }

    #[test]
    fn wrapping_honors_width() {
        assert_eq!(wrap_to_width("abcdef", 3), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(wrap_to_width("ab", 2), vec!["ab".to_string()]);
    }
}