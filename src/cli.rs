use clap::{Arg, ArgMatches, Command};
use shell_words;
pub fn parse_command(input: &str) -> Result<ArgMatches, clap::Error> {
    let args = shell_words::split(input).unwrap_or_else(|_| vec![]); 

        Command::new("myshell")
            .subcommand(
                Command::new("exit")
                    .about("Exit the shell")
                    .arg(Arg::new("CODE").required(false)),
            )
            .subcommand(
                Command::new("echo")
                    .about("Echoes input")
                    .arg(Arg::new("TEXT").required(true).num_args(1..).allow_hyphen_values(true)),
            )
            .subcommand(
                Command::new("cat")
                    .about("Concatenates and prints file contents")
                    .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe  with  space")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe with \"quotes\"")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe with \\'single quotes\\'")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe with \\n newline")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("type")
                    .about("Finds command type")
                    .arg(Arg::new("CMD").required(true)),
            )
            .subcommand(
                Command::new("cd")
                .about("move the the directory")
                .arg(Arg::new("CD").required(true)),
            )
            .subcommand(Command::new("pwd").about("Tells the current directory"))
            .try_get_matches_from(std::iter::once("myshell").chain(args.iter().map(String::as_str)))
}