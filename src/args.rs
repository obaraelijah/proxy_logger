use std::net;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl ValueEnum for LoggingLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Trace,
            Self::Debug,
            Self::Info,
            Self::Warn,
            Self::Error,
            Self::Off,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Trace => PossibleValue::new("trace"),
            Self::Debug => PossibleValue::new("debug"),
            Self::Info => PossibleValue::new("info"),
            Self::Warn => PossibleValue::new("warn"),
            Self::Error => PossibleValue::new("error"),
            Self::Off => PossibleValue::new("off"),
        })
    }
}

impl std::fmt::Display for LoggingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values skipped")
            .get_name()
            .fmt(f)
    }
}


#[derive(Debug, Clone, Copy)]
pub enum TimestampPrecision {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

impl ValueEnum for TimestampPrecision {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Seconds,
            Self::Milliseconds,
            Self::Microseconds,
            Self::Nanoseconds,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Seconds => PossibleValue::new("seconds"),
            Self::Milliseconds => PossibleValue::new("milliseconds"),
            Self::Microseconds => PossibleValue::new("microseconds"),
            Self::Nanoseconds => PossibleValue::new("nanoseconds"),
        })
    }
}

#[command(next_line_help = true)]
#[command(author, version, about, long_about = None)]
#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    /// Application logging level.
    #[arg(short, long, default_value = "debug")]
    pub level: LoggingLevel,
    /// Address on which TCP listener should be binded.
    #[arg(short, long)]
    pub bind_listener_addr: net::SocketAddr,
    /// Address of remote server.
    #[arg(short, long)]
    pub remote_addr: net::SocketAddr,
    /// Incoming connection reading timeout.
    #[arg(short, long, default_value = "60")]
    pub timeout: u64,
    /// Timestamp precision.
    #[arg(short, long, default_value = "seconds")]
    pub precision: TimestampPrecision,
}