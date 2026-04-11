use std::collections::HashMap;
use std::io::{self, stdout, IsTerminal, Stdout, Write};
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::queue;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use deckclip_core::{Config, DeckClient};
use owo_colors::OwoColorize;
use serde::Deserialize;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::output::OutputMode;

const LOGO: &str = include_str!("../logo.ans");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProviderKind {
    ChatGpt,
    OpenAI,
    Anthropic,
    Ollama,
}

impl ProviderKind {
    const ALL: [ProviderKind; 4] = [
        ProviderKind::ChatGpt,
        ProviderKind::OpenAI,
        ProviderKind::Anthropic,
        ProviderKind::Ollama,
    ];

    fn from_index(index: usize) -> Self {
        Self::ALL[index]
    }

    fn index(self) -> usize {
        match self {
            ProviderKind::ChatGpt => 0,
            ProviderKind::OpenAI => 1,
            ProviderKind::Anthropic => 2,
            ProviderKind::Ollama => 3,
        }
    }

    fn id(self) -> &'static str {
        match self {
            ProviderKind::ChatGpt => "chatgpt",
            ProviderKind::OpenAI => "openai_api",
            ProviderKind::Anthropic => "anthropic",
            ProviderKind::Ollama => "ollama",
        }
    }

    fn title(self) -> &'static str {
        match self {
            ProviderKind::ChatGpt => "Sign in with ChatGPT (usage included with your ChatGPT Plus)",
            ProviderKind::OpenAI => "Provide your own OpenAI API key",
            ProviderKind::Anthropic => "Provide your own Anthropic API key",
            ProviderKind::Ollama => "Use Ollama",
        }
    }

    fn description(self, status: &ProviderStatus) -> String {
        let mut text = match self {
            ProviderKind::ChatGpt => {
                if let Some(account) = status.account.as_deref() {
                    format!("使用 ChatGPT OAuth 授权。当前账号：{account}")
                } else {
                    "使用 ChatGPT OAuth 授权，在浏览器中完成登录".to_string()
                }
            }
            ProviderKind::OpenAI => "通过 OpenAI API key 进行模型调用".to_string(),
            ProviderKind::Anthropic => "通过 Anthropic API key 进行模型调用".to_string(),
            ProviderKind::Ollama => "使用本地模型".to_string(),
        };

        if status.selected {
            text.push_str("  [当前使用]");
        } else if status.configured {
            text.push_str("  [已配置]");
        }

        text
    }

    fn base_url_placeholder(self) -> Option<&'static str> {
        match self {
            ProviderKind::OpenAI => Some("https://api.openai.com/v1"),
            ProviderKind::Anthropic => Some("https://api.anthropic.com/v1"),
            ProviderKind::Ollama => Some("http://localhost:11434"),
            ProviderKind::ChatGpt => None,
        }
    }

    fn model_placeholder(self) -> Option<&'static str> {
        match self {
            ProviderKind::OpenAI => Some("gpt-5.3"),
            ProviderKind::Anthropic => Some("claude-sonnet-4-6"),
            ProviderKind::Ollama => Some("llama3.3"),
            ProviderKind::ChatGpt => None,
        }
    }

    fn success_title(self) -> &'static str {
        match self {
            ProviderKind::ChatGpt => "ChatGPT 登录成功",
            ProviderKind::OpenAI => "OpenAI API 已配置",
            ProviderKind::Anthropic => "Anthropic API 已配置",
            ProviderKind::Ollama => "Ollama 已配置",
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ProviderStatus {
    #[serde(default)]
    configured: bool,
    #[serde(default)]
    selected: bool,
    #[serde(default)]
    account: Option<String>,
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default)]
    model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct LoginStatusData {
    #[serde(default)]
    providers: HashMap<String, ProviderStatus>,
}

impl LoginStatusData {
    fn provider(&self, provider: ProviderKind) -> ProviderStatus {
        self.providers
            .get(provider.id())
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
struct InputField {
    label: &'static str,
    placeholder: &'static str,
    value: String,
    secret: bool,
}

#[derive(Debug, Clone)]
struct FormState {
    provider: ProviderKind,
    fields: Vec<InputField>,
    focus: usize,
    error: Option<String>,
}

impl FormState {
    fn new(provider: ProviderKind, status: &ProviderStatus) -> Self {
        let base_url = status
            .base_url
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| {
                provider
                    .base_url_placeholder()
                    .unwrap_or_default()
                    .to_string()
            });
        let model = status
            .model
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| provider.model_placeholder().unwrap_or_default().to_string());

        let fields = match provider {
            ProviderKind::OpenAI | ProviderKind::Anthropic => vec![
                InputField {
                    label: "Base URL",
                    placeholder: provider.base_url_placeholder().unwrap_or_default(),
                    value: base_url,
                    secret: false,
                },
                InputField {
                    label: "API Key",
                    placeholder: "在这里输入 API Key",
                    value: String::new(),
                    secret: true,
                },
                InputField {
                    label: "Model",
                    placeholder: provider.model_placeholder().unwrap_or_default(),
                    value: model,
                    secret: false,
                },
            ],
            ProviderKind::Ollama => vec![
                InputField {
                    label: "Base URL",
                    placeholder: provider.base_url_placeholder().unwrap_or_default(),
                    value: base_url,
                    secret: false,
                },
                InputField {
                    label: "Model",
                    placeholder: provider.model_placeholder().unwrap_or_default(),
                    value: model,
                    secret: false,
                },
            ],
            ProviderKind::ChatGpt => Vec::new(),
        };

        Self {
            provider,
            fields,
            focus: 0,
            error: None,
        }
    }

    fn focused_mut(&mut self) -> Option<&mut InputField> {
        self.fields.get_mut(self.focus)
    }

    fn move_focus(&mut self, delta: isize) {
        if self.fields.is_empty() {
            return;
        }
        let len = self.fields.len() as isize;
        self.focus = (self.focus as isize + delta).rem_euclid(len) as usize;
    }

    fn values(&self) -> Vec<String> {
        self.fields
            .iter()
            .map(|field| field.value.trim().to_string())
            .collect()
    }
}

#[derive(Debug, Clone)]
enum Screen {
    Menu {
        selected: usize,
        info: Option<String>,
    },
    ConfirmOverwrite {
        provider: ProviderKind,
        yes_selected: bool,
    },
    Form(FormState),
    ChatGptWaiting,
    Result {
        success: bool,
        title: String,
        detail: Option<String>,
    },
}

#[derive(Debug)]
enum AsyncEvent {
    ChatGptFinished {
        request_id: u64,
        result: Result<String, String>,
    },
}

struct LoginApp {
    status: LoginStatusData,
    screen: Screen,
    events_tx: UnboundedSender<AsyncEvent>,
    events_rx: UnboundedReceiver<AsyncEvent>,
    next_request_id: u64,
    active_chatgpt_request_id: Option<u64>,
}

impl LoginApp {
    fn new(status: LoginStatusData) -> Self {
        let (events_tx, events_rx) = unbounded_channel();
        Self {
            status,
            screen: Screen::Menu {
                selected: 0,
                info: None,
            },
            events_tx,
            events_rx,
            next_request_id: 1,
            active_chatgpt_request_id: None,
        }
    }

    async fn drain_async_events(&mut self) -> Result<()> {
        while let Ok(event) = self.events_rx.try_recv() {
            match event {
                AsyncEvent::ChatGptFinished { request_id, result } => {
                    if self.active_chatgpt_request_id != Some(request_id) {
                        continue;
                    }

                    self.active_chatgpt_request_id = None;
                    match result {
                        Ok(detail) => {
                            self.status = fetch_login_status().await?;
                            self.screen = Screen::Result {
                                success: true,
                                title: ProviderKind::ChatGpt.success_title().to_string(),
                                detail: Some(detail),
                            };
                        }
                        Err(error) => {
                            self.screen = Screen::Result {
                                success: false,
                                title: "ChatGPT 登录失败".to_string(),
                                detail: Some(error),
                            };
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_key_event(&mut self, key: KeyEvent) -> Result<bool> {
        if matches!(key.kind, KeyEventKind::Release) {
            return Ok(false);
        }

        enum PostAction {
            None,
            SelectProvider,
            ClearAndContinue(ProviderKind),
            SubmitForm,
            CancelChatGptAndExit,
        }

        let mut should_exit = false;
        let mut post_action = PostAction::None;

        match &mut self.screen {
            Screen::Menu { selected, info } => match key.code {
                KeyCode::Esc => should_exit = true,
                KeyCode::Up | KeyCode::Char('k') => {
                    *selected = selected.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    *selected = (*selected + 1).min(ProviderKind::ALL.len().saturating_sub(1));
                }
                KeyCode::Char(c) if ('1'..='4').contains(&c) => {
                    *selected = (c as u8 - b'1') as usize;
                    *info = None;
                    post_action = PostAction::SelectProvider;
                }
                KeyCode::Enter => {
                    *info = None;
                    post_action = PostAction::SelectProvider;
                }
                _ => {}
            },
            Screen::ConfirmOverwrite {
                provider,
                yes_selected,
            } => match key.code {
                KeyCode::Esc => should_exit = true,
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Tab
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::Char('h')
                | KeyCode::Char('l') => {
                    *yes_selected = !*yes_selected;
                }
                KeyCode::Enter => {
                    if *yes_selected {
                        post_action = PostAction::ClearAndContinue(*provider);
                    } else {
                        self.screen = Screen::Menu {
                            selected: provider.index(),
                            info: None,
                        };
                    }
                }
                _ => {}
            },
            Screen::Form(form) => match key.code {
                KeyCode::Esc => should_exit = true,
                KeyCode::Tab | KeyCode::Down => {
                    form.move_focus(1);
                }
                KeyCode::BackTab | KeyCode::Up => {
                    form.move_focus(-1);
                }
                KeyCode::Backspace => {
                    if let Some(field) = form.focused_mut() {
                        field.value.pop();
                        form.error = None;
                    }
                }
                KeyCode::Enter => {
                    post_action = PostAction::SubmitForm;
                }
                KeyCode::Char(c)
                    if !key.modifiers.contains(KeyModifiers::CONTROL)
                        && !key.modifiers.contains(KeyModifiers::ALT)
                        && !key.modifiers.contains(KeyModifiers::SUPER) =>
                {
                    if let Some(field) = form.focused_mut() {
                        field.value.push(c);
                        form.error = None;
                    }
                }
                _ => {}
            },
            Screen::ChatGptWaiting => {
                if key.code == KeyCode::Esc {
                    post_action = PostAction::CancelChatGptAndExit;
                }
            }
            Screen::Result { .. } => match key.code {
                KeyCode::Enter | KeyCode::Esc => should_exit = true,
                _ => {}
            },
        }

        match post_action {
            PostAction::None => {}
            PostAction::SelectProvider => self.select_provider().await?,
            PostAction::ClearAndContinue(provider) => {
                self.clear_provider(provider).await?;
                if provider == ProviderKind::ChatGpt {
                    self.start_chatgpt_login();
                } else {
                    let status = self.status.provider(provider);
                    self.screen = Screen::Form(FormState::new(provider, &status));
                }
            }
            PostAction::SubmitForm => self.submit_form().await?,
            PostAction::CancelChatGptAndExit => {
                let _ = cancel_chatgpt_login().await;
                self.active_chatgpt_request_id = None;
                should_exit = true;
            }
        }

        Ok(should_exit)
    }

    fn handle_paste(&mut self, pasted: String) {
        if let Screen::Form(form) = &mut self.screen {
            if let Some(field) = form.focused_mut() {
                field.value.push_str(&pasted);
                form.error = None;
            }
        }
    }

    async fn select_provider(&mut self) -> Result<()> {
        let selected = match &self.screen {
            Screen::Menu { selected, .. } => *selected,
            _ => return Ok(()),
        };
        let provider = ProviderKind::from_index(selected);
        let status = self.status.provider(provider);

        if status.configured {
            self.screen = Screen::ConfirmOverwrite {
                provider,
                yes_selected: false,
            };
            return Ok(());
        }

        if provider == ProviderKind::ChatGpt {
            self.start_chatgpt_login();
        } else {
            self.screen = Screen::Form(FormState::new(provider, &status));
        }

        Ok(())
    }

    async fn clear_provider(&mut self, provider: ProviderKind) -> Result<()> {
        let mut client = DeckClient::new(Config::default());
        client
            .login_clear(provider.id())
            .await
            .with_context(|| format!("无法清空 {} 配置", provider.id()))?;
        self.status = fetch_login_status().await?;
        Ok(())
    }

    fn start_chatgpt_login(&mut self) {
        let request_id = self.next_request_id;
        self.next_request_id += 1;
        self.active_chatgpt_request_id = Some(request_id);

        let tx = self.events_tx.clone();
        tokio::spawn(async move {
            let mut client = DeckClient::new(Config::default());
            let result = match client.login_chatgpt_start().await {
                Ok(response) => Ok(response_detail_message(&response)
                    .unwrap_or_else(|| "浏览器授权已完成，Deck 已切换到 ChatGPT。".to_string())),
                Err(error) => Err(error.to_string()),
            };
            let _ = tx.send(AsyncEvent::ChatGptFinished { request_id, result });
        });

        self.screen = Screen::ChatGptWaiting;
    }

    async fn submit_form(&mut self) -> Result<()> {
        let Screen::Form(form) = &mut self.screen else {
            return Ok(());
        };

        let provider = form.provider;
        let values = form.values();
        let validation_error = match provider {
            ProviderKind::OpenAI | ProviderKind::Anthropic => {
                if values.get(0).is_none_or(|value| value.is_empty()) {
                    Some("Base URL 不能为空".to_string())
                } else if values.get(1).is_none_or(|value| value.is_empty()) {
                    Some("API Key 不能为空".to_string())
                } else if values.get(2).is_none_or(|value| value.is_empty()) {
                    Some("Model 不能为空".to_string())
                } else {
                    None
                }
            }
            ProviderKind::Ollama => {
                if values.get(0).is_none_or(|value| value.is_empty()) {
                    Some("Base URL 不能为空".to_string())
                } else if values.get(1).is_none_or(|value| value.is_empty()) {
                    Some("Model 不能为空".to_string())
                } else {
                    None
                }
            }
            ProviderKind::ChatGpt => None,
        };

        if let Some(error) = validation_error {
            form.error = Some(error);
            return Ok(());
        }

        let mut client = DeckClient::new(Config::default());
        let result = match provider {
            ProviderKind::OpenAI => {
                client
                    .login_openai_configure(&values[0], &values[2], &values[1])
                    .await
            }
            ProviderKind::Anthropic => {
                client
                    .login_anthropic_configure(&values[0], &values[2], &values[1])
                    .await
            }
            ProviderKind::Ollama => client.login_ollama_configure(&values[0], &values[1]).await,
            ProviderKind::ChatGpt => unreachable!("ChatGPT does not use a local form"),
        };

        match result {
            Ok(response) => {
                self.status = fetch_login_status().await?;
                self.screen = Screen::Result {
                    success: true,
                    title: provider.success_title().to_string(),
                    detail: response_detail_message(&response),
                };
            }
            Err(error) => {
                if let Screen::Form(form) = &mut self.screen {
                    form.error = Some(error.to_string());
                }
            }
        }

        Ok(())
    }
}

struct TerminalGuard {
    stdout: Stdout,
}

impl TerminalGuard {
    fn enter() -> Result<Self> {
        enable_raw_mode().context("无法进入终端 raw mode")?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, Hide).context("无法进入终端登录界面")?;
        Ok(Self { stdout })
    }

    fn render(&mut self, app: &LoginApp) -> Result<()> {
        queue!(self.stdout, MoveTo(0, 0), Clear(ClearType::All))?;
        write!(self.stdout, "{LOGO}")?;
        writeln!(self.stdout)?;

        match &app.screen {
            Screen::Menu { selected, info } => {
                render_menu(&mut self.stdout, &app.status, *selected, info.as_deref())?
            }
            Screen::ConfirmOverwrite {
                provider,
                yes_selected,
            } => render_confirm(&mut self.stdout, *provider, *yes_selected)?,
            Screen::Form(form) => render_form(&mut self.stdout, form)?,
            Screen::ChatGptWaiting => render_chatgpt_waiting(&mut self.stdout)?,
            Screen::Result {
                success,
                title,
                detail,
            } => render_result(&mut self.stdout, *success, title, detail.as_deref())?,
        }

        self.stdout.flush()?;
        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(self.stdout, Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

fn render_menu(
    stdout: &mut Stdout,
    status: &LoginStatusData,
    selected: usize,
    info: Option<&str>,
) -> Result<()> {
    writeln!(stdout, "{}", "DeckClip Login".bold())?;
    writeln!(stdout, "为 Deck 选择或重新配置 AI 提供商。")?;
    writeln!(stdout)?;

    for (index, provider) in ProviderKind::ALL.iter().enumerate() {
        let provider_status = status.provider(*provider);
        let prefix = if index == selected { ">" } else { " " };
        let title = format!("{prefix} {}. {}", index + 1, provider.title());
        if index == selected {
            writeln!(stdout, "{}", title.cyan())?;
            writeln!(
                stdout,
                "    {}",
                provider.description(&provider_status).dimmed().cyan()
            )?;
        } else {
            writeln!(stdout, "{title}")?;
            writeln!(
                stdout,
                "    {}",
                provider.description(&provider_status).dimmed()
            )?;
        }
        writeln!(stdout)?;
    }

    writeln!(
        stdout,
        "{}",
        "Press Enter to continue, or ESC to exit".dimmed()
    )?;
    if let Some(info) = info {
        writeln!(stdout)?;
        writeln!(stdout, "{}", info.cyan())?;
    }
    Ok(())
}

fn render_confirm(stdout: &mut Stdout, provider: ProviderKind, yes_selected: bool) -> Result<()> {
    writeln!(stdout, "{}", "检测到已有配置，请问是否要继续？".bold())?;
    writeln!(
        stdout,
        "继续后会清空当前 {} 配置，并重新开始设置。",
        provider.title()
    )?;
    writeln!(stdout)?;

    let no_button = if yes_selected {
        "[ No ]".to_string()
    } else {
        format!("{}", "[ No ]".cyan().bold())
    };
    let yes_button = if yes_selected {
        format!("{}", "[ Yes ]".cyan().bold())
    } else {
        "[ Yes ]".to_string()
    };

    writeln!(stdout, "  {no_button}  {yes_button}")?;
    writeln!(stdout)?;
    writeln!(
        stdout,
        "{}",
        "Press Enter to continue, or ESC to exit".dimmed()
    )?;
    Ok(())
}

fn render_form(stdout: &mut Stdout, form: &FormState) -> Result<()> {
    writeln!(stdout, "{}", form.provider.title().bold())?;
    writeln!(stdout, "填写下列信息后，Deck 会立即切换到对应提供商。")?;
    writeln!(stdout)?;

    for (index, field) in form.fields.iter().enumerate() {
        let selected = index == form.focus;
        let label = if selected {
            format!("> {}", field.label).cyan().to_string()
        } else {
            format!("  {}", field.label)
        };
        let value = if field.value.is_empty() {
            format!("<{}>", field.placeholder).dimmed().to_string()
        } else if field.secret {
            "•".repeat(field.value.chars().count())
        } else {
            field.value.clone()
        };
        if selected {
            writeln!(stdout, "{label}")?;
            writeln!(stdout, "    {}", value.cyan())?;
        } else {
            writeln!(stdout, "{label}")?;
            writeln!(stdout, "    {value}")?;
        }
        writeln!(stdout)?;
    }

    writeln!(
        stdout,
        "{}",
        "Press Tab to switch fields, Enter to save, or ESC to exit".dimmed()
    )?;
    if let Some(error) = &form.error {
        writeln!(stdout)?;
        writeln!(stdout, "{}", error.red())?;
    }
    Ok(())
}

fn render_chatgpt_waiting(stdout: &mut Stdout) -> Result<()> {
    writeln!(stdout, "{}", "Sign in with ChatGPT".bold())?;
    writeln!(stdout, "正在等待浏览器完成授权。")?;
    writeln!(stdout, "完成后 Deck 会自动切换到 ChatGPT。")?;
    writeln!(stdout)?;
    writeln!(stdout, "{}", "Press ESC to cancel and exit".dimmed())?;
    Ok(())
}

fn render_result(
    stdout: &mut Stdout,
    success: bool,
    title: &str,
    detail: Option<&str>,
) -> Result<()> {
    let icon = if success { "✓" } else { "!" };
    let title = if success {
        title.green().bold().to_string()
    } else {
        title.red().bold().to_string()
    };

    writeln!(stdout, "{icon} {title}")?;
    if let Some(detail) = detail.filter(|text| !text.trim().is_empty()) {
        writeln!(stdout)?;
        writeln!(stdout, "{detail}")?;
    }
    writeln!(stdout)?;
    writeln!(
        stdout,
        "{}",
        "Press Enter to finish, or ESC to exit".dimmed()
    )?;
    Ok(())
}

pub async fn run(output: OutputMode) -> Result<()> {
    if matches!(output, OutputMode::Json) {
        bail!("`deckclip login` 暂不支持 --json")
    }

    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        bail!("`deckclip login` 需要交互式终端")
    }

    let status = fetch_login_status().await?;
    let mut app = LoginApp::new(status);
    let mut terminal = TerminalGuard::enter()?;

    loop {
        app.drain_async_events().await?;
        terminal.render(&app)?;

        if event::poll(Duration::from_millis(50)).context("读取按键事件失败")? {
            match event::read().context("读取终端事件失败")? {
                Event::Key(key) => {
                    if app.handle_key_event(key).await? {
                        break;
                    }
                }
                Event::Paste(text) => app.handle_paste(text),
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    Ok(())
}

async fn fetch_login_status() -> Result<LoginStatusData> {
    let mut client = DeckClient::new(Config::default());
    let response = client.login_status().await?;
    let data = response
        .data
        .ok_or_else(|| anyhow!("登录状态响应缺少 data 字段"))?;
    serde_json::from_value(data).context("无法解析登录状态响应")
}

async fn cancel_chatgpt_login() -> Result<()> {
    let mut client = DeckClient::new(Config::default());
    let _ = client.login_chatgpt_cancel().await?;
    Ok(())
}

fn response_detail_message(response: &deckclip_protocol::Response) -> Option<String> {
    let data = response.data.as_ref()?;
    if let Some(message) = data.get("message").and_then(|value| value.as_str()) {
        return Some(message.to_string());
    }
    if let Some(account) = data.get("account").and_then(|value| value.as_str()) {
        return Some(format!("当前账号：{account}"));
    }
    if let Some(provider) = data.get("provider").and_then(|value| value.as_str()) {
        return Some(format!("当前提供商：{provider}"));
    }
    None
}
