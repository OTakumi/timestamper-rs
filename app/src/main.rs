use clap::Parser;
mod time_stamp;
mod calc;

// TODO: 開始打刻できる
// TODO: 終了打刻できる
// TODO: 当日の稼働時間を表示する
// TODO: 当月の合計稼働時間を表示する
// TODO: 当月の稼働日数を表示する
// TODO: 打刻一覧をCSVファイルで出力する

#[derive(Parser)]
#[clap(
    name = "Time stamper",
    version = "v0.1.0",
    about = "Worktime punch application"
)]

struct Cli {
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
}

fn main() {
    let cli: Cli = Cli::parse();

    if cli.start {
        let timestamp = time_stamp::TimeStamp::new();
        println!("Start Time: {:?}", timestamp.get_start_work_time());
    }

    if cli.end {
        let timestamp = time_stamp::TimeStamp::new();
        println!("End Time: {:?}", timestamp.get_end_work_time());
    }

    if cli.month {
        println!("This month worktime: ");
    }
}
