use std::{
    os::unix::process::ExitStatusExt,
    path::Path,
    process::{Command, ExitCode},
};

use clap::{Parser, builder::NonEmptyStringValueParser};

#[derive(Debug)]
enum Error {
    NoTerminal,
    InvalidTerminal(String),
    NoClassOption,
    Io(std::io::Error),
}

#[derive(Debug, Default)]
struct Spec {
    subcommand: Option<&'static str>,
    class: Option<&'static str>,
    wait: Option<&'static str>,
    exec: Option<&'static str>,
}

impl Spec {
    fn for_terminal(name: &str) -> Self {
        match name {
            "alacritty" => Spec {
                subcommand: None,
                class: Some("--class"),
                wait: None,
                exec: Some("-e"),
            },
            "foot" => Spec {
                subcommand: None,
                class: Some("--app-id"),
                wait: None,
                exec: Some("--"),
            },
            "gnome-terminal" => Spec {
                subcommand: None,
                class: Some("--class"),
                wait: Some("--wait"),
                exec: Some("--"),
            },
            "kitty" => Spec {
                subcommand: None,
                class: Some("--class"),
                wait: None,
                exec: None,
            },
            "konsole" => Spec {
                subcommand: None,
                class: None,
                wait: None,
                exec: Some("-e"),
            },
            "st" => Spec {
                subcommand: None,
                class: Some("-c"),
                wait: None,
                exec: Some("-e"),
            },
            "terminator" => Spec {
                subcommand: None,
                class: None,
                wait: None,
                exec: Some("-x"),
            },
            "urxvt" => Spec {
                subcommand: None,
                class: Some("-name"),
                wait: None,
                exec: Some("-e"),
            },
            "wezterm" => Spec {
                subcommand: Some("start"),
                class: Some("--class"),
                wait: None,
                exec: Some("--"),
            },
            "xfce4-terminal" => Spec {
                subcommand: None,
                class: Some("--class"),
                wait: None,
                exec: Some("-x"),
            },
            "xterm" => Spec {
                subcommand: None,
                class: Some("-class"),
                wait: None,
                exec: Some("-e"),
            },
            _ => Spec::default(),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[arg(long = "terminal", help = "Terminal command", value_parser = NonEmptyStringValueParser::new())]
    terminal_command: Option<String>,

    #[arg(long, help = "Terminal exec subcommand", value_parser = NonEmptyStringValueParser::new())]
    exec_subcommand: Option<String>,

    #[arg(long, help = "Terminal exec option", value_parser = NonEmptyStringValueParser::new())]
    exec_option: Option<String>,

    #[arg(long, help = "Terminal wait option", value_parser = NonEmptyStringValueParser::new())]
    wait_option: Option<String>,

    #[arg(long, help = "Terminal class option", value_parser = NonEmptyStringValueParser::new())]
    class_option: Option<String>,

    #[arg(long, help = "Terminal class", value_parser = NonEmptyStringValueParser::new())]
    class: Option<String>,

    #[cfg(target_os = "linux")]
    #[arg(short, long, help = "Redirect stdio to parent shell")]
    redirect: bool,

    #[arg(required = true, trailing_var_arg = true, help = "Command to run")]
    command: Vec<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            match err {
                Error::NoTerminal => {
                    eprintln!(
                        "error: no terminal was provided\nhint: set $TERMINAL or pass --terminal <TERMINAL>"
                    )
                }
                Error::InvalidTerminal(terminal) => {
                    eprintln!("error: invalid terminal '{terminal}'")
                }
                Error::NoClassOption => eprintln!(
                    "error: terminal has no default class option\nhint: pass --class-option <CLASS_OPTION>"
                ),
                Error::Io(err) => eprintln!("error: {err}"),
            }
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<u8, Error> {
    let cli = Cli::parse();

    let parts = cli
        .terminal_command
        .or_else(|| std::env::var("TERMINAL").ok())
        .filter(|c| !c.trim().is_empty())
        .ok_or(Error::NoTerminal)
        .and_then(|s| shlex::split(&s).ok_or_else(|| Error::InvalidTerminal(s)))?;
    let term_cmd = &parts[0];
    let term_args = &parts[1..];
    let term_name = Path::new(term_cmd)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(term_cmd);

    let spec = Spec::for_terminal(term_name);

    let subcommand = cli.exec_subcommand.as_deref().or(spec.subcommand);
    let exec_option = cli.exec_option.as_deref().or(spec.exec);
    let wait_option = cli.wait_option.as_deref().or(spec.wait);
    let class_option = cli.class_option.as_deref().or(spec.class);
    let class = cli
        .class
        .map(|c| class_option.map(|_| c).ok_or(Error::NoClassOption))
        .transpose()?;

    let mut cmd = Command::new(term_cmd);
    cmd.args(term_args);

    if let Some(subcommand) = subcommand {
        cmd.arg(subcommand);
    }

    if let Some(class_option) = class_option
        && let Some(class) = class
    {
        cmd.arg(class_option).arg(class);
    }

    if let Some(wait_option) = wait_option {
        cmd.arg(wait_option);
    }

    if let Some(exec_option) = exec_option {
        cmd.arg(exec_option);
    }

    #[cfg(target_os = "linux")]
    if cli.redirect {
        let pid = std::process::id();
        cmd.arg("sh").arg("-c").arg(format!(
            "exec \"$0\" \"$@\" 9<>/dev/tty </proc/{pid}/fd/0 >/proc/{pid}/fd/1 2>/proc/{pid}/fd/2"
        ));
    }

    cmd.args(cli.command);

    cmd.status()
        .map(|s| match (s.code(), s.signal()) {
            (Some(c), _) => Ok((c & 0xff) as u8),
            (None, Some(s)) => Ok((128 + s) as u8),
            (None, None) => Ok(1),
        })
        .map_err(Error::Io)?
}
