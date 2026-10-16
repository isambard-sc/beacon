use anyhow::{bail, Context, Result};
use clap::{CommandFactory as _, Parser, Subcommand};

mod cache;
mod jwt;
mod oidc;

const ISSUER: &str = "https://keycloak.isambard.ac.uk/realms/isambard";
/// Public Keycloak client with the device grant enabled. It issues access tokens only.
const CLIENT_ID: &str = "k8s-gateway";
const SCOPE: &str = "openid profile";

#[derive(Parser)]
#[command(version, about)]
/// Log in to Isambard and get an access token for the Kubernetes gateway
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Log in with the device flow and cache the access token
    Auth,
    /// Print the cached access token if it is still valid
    Token,
    /// Show the claims in the cached access token
    Whoami,
    /// Delete the cached access token
    Logout,
}

/// Shown when beacon is run without a command
fn print_getting_started() {
    println!("Log in to Isambard and get an access token for the Kubernetes gateway.\n");
    println!("To get started, run:\n");
    println!("  beacon auth\n");
    println!("This prints a link to open in your browser so you can log in!\n");
    println!("Then, to print the access token, run:\n");
    println!("  beacon token\n");
    println!("Available commands:");
    for sub in Args::command().get_subcommands() {
        let about = sub.get_about().map(|a| a.to_string()).unwrap_or_default();
        println!("  beacon {:<8} {about}", sub.get_name());
    }
    println!("\nRun `beacon help` for more details.");
}

fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

/// The cached access token, if it has at least 60s left
fn valid_token() -> Result<String> {
    let c = cache::load()?;
    if c.expires_at - now() < 60 {
        bail!("Your access token has expired. Run `beacon auth` to log in again.");
    }
    Ok(c.access_token)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let Some(command) = args.command else {
        print_getting_started();
        return Ok(());
    };
    match command {
        Command::Auth => {
            let ep = oidc::discover(ISSUER)?;
            let access_token = oidc::device_auth(&ep, CLIENT_ID, SCOPE, std::thread::sleep)?;
            let c = cache::Cache {
                expires_at: jwt::exp(&access_token).context("Access token has no expiry.")?,
                access_token,
            };
            cache::save(&c)?;
            eprintln!("Logged in.");
        }
        Command::Token => match valid_token() {
            Ok(token) => println!("{token}"),
            Err(e) => {
                eprintln!("{e:#}");
                std::process::exit(1);
            }
        },
        Command::Whoami => {
            let c = cache::load()?;
            let claims = jwt::payload(&c.access_token)?;
            for name in [
                "iss",
                "aud",
                "azp",
                "preferred_username",
                "short_name",
                "groups",
            ] {
                let value = match &claims[name] {
                    serde_json::Value::Null => "<missing>".to_string(),
                    serde_json::Value::String(s) => s.clone(),
                    v => v.to_string(),
                };
                println!("{name:<19} {value}");
            }
            let exp = chrono::DateTime::from_timestamp(c.expires_at, 0)
                .context("Invalid expiry time.")?
                .with_timezone(&chrono::Local);
            let left = c.expires_at - now();
            let left = if left > 0 {
                format!("{}m {}s left", left / 60, left % 60)
            } else {
                "expired".to_string()
            };
            println!(
                "{:<19} {} ({left})",
                "exp",
                exp.format("%Y-%m-%d %H:%M:%S %:z")
            );
        }
        Command::Logout => {
            cache::delete()?;
            eprintln!("Logged out.");
        }
    }
    Ok(())
}
