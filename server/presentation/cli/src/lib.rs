use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "Time stamper",
    version = "v0.1.0",
    about = "Worktime punch application"
)]
pub struct Cli {
    /// Punch start work time
    #[arg(short, long)]
    start: bool,

    /// Punch end work time
    #[arg(short, long)]
    end: bool,

    /// Show worktime of today
    #[arg(short, long)]
    today: bool,

    /// Show worktime of this month
    #[arg(short, long)]
    month: bool,

    /// Show work status
    #[arg(long)]
    status: bool,
}

impl Cli {
    pub fn run() {
        let cli: Cli = Cli::parse();

        if cli.start {
            println!("Start work");
        }

        if cli.end {
            println!("End work");
        }

        if cli.month {
            println!("This month worktime: ");
        }

        if cli.status {
            println!("Work status");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
