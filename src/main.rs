use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{self, stdout};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect(); // skip binary name

    let use_email_picker = args.iter().any(|arg| arg == "-s" || arg == "--signoff");

    if use_email_picker {
        let emails = parse_emails_from_gitconfig();
        if emails.is_empty() {
            eprintln!("❌ No email addresses found in ~/.gitconfig.");
            return Ok(());
        }

        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let selected_email = run_ui(&mut terminal, &emails)?;

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        // Set Git author/committer env vars
        unsafe {
            env::set_var("GIT_AUTHOR_EMAIL", &selected_email);
            env::set_var("GIT_COMMITTER_EMAIL", &selected_email);
        }


        println!("✅ Using email: {}", selected_email);
    }

    let mut command = Command::new("git");
    
    // Only add "commit" if the user did not already specify it
    if args.first().map(String::as_str) != Some("commit") {
        command.arg("commit");
    }
    
    let mut child = command
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;


    child.wait()?;
    Ok(())
}

fn run_ui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    emails: &[String],
) -> io::Result<String> {
    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Min(1)].as_ref())
                .split(size);

            let items: Vec<ListItem> = emails
                .iter()
                .enumerate()
                .map(|(i, email)| {
                    let style = if i == selected {
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    };
                    ListItem::new(email.clone()).style(style)
                })
                .collect();

            let email_list = List::new(items)
                .block(Block::default().title("Select Git Email").borders(Borders::ALL));
            f.render_widget(email_list, chunks[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        selected = (selected + 1) % emails.len();
                    }
                    KeyCode::Up => {
                        if selected == 0 {
                            selected = emails.len() - 1;
                        } else {
                            selected -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        return Ok(emails[selected].clone());
                    }
                    KeyCode::Char('q') => {
                        return Err(io::Error::new(io::ErrorKind::Other, "Cancelled by user"));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn parse_emails_from_gitconfig() -> Vec<String> {
    let mut emails = HashSet::new();

    let gitconfig_path = dirs::home_dir()
        .map(|mut path| {
            path.push(".gitconfig");
            path
        })
        .unwrap_or_else(|| PathBuf::from("~/.gitconfig"));

    let content = match fs::read_to_string(&gitconfig_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let email_re = Regex::new(r"(?i)^[#\s]*email\s*=\s*(\S+@\S+)").unwrap();

    for line in content.lines() {
        if let Some(caps) = email_re.captures(line) {
            if let Some(email) = caps.get(1) {
                emails.insert(email.as_str().to_string());
            }
        }
    }

    let mut email_vec: Vec<_> = emails.into_iter().collect();
    email_vec.sort();
    email_vec
}
