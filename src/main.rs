use octocrate::{APIConfig, GitHubAPI};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rfc-merge-tool", about = "A tool to merge RFCs.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Pr { pr: i64 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args_safe()?;

    match opt.command {
        Command::Pr { pr } => {
            // Create a default GitHub API configuration
            let config = APIConfig::default().shared();

            let api = GitHubAPI::new(&config);

            let pull_request = api.pulls.get("rust-lang", "rfcs", pr).send().await.unwrap();

            eprintln!("{pull_request:?}");
        }
    }

    Ok(())
}
