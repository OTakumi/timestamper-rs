// read csv file and return a vector of the rows
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use csv::Reader;
use chrono::prelude::*;

struct CsvReader {
    file_path: String,
}

struct TotalWorkTime {
    total: DateTime<Utc>,
    unit: String,
}

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut rdr = Reader::from_reader(contents.as_bytes());
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row: Vec<String> = Vec::new();
        for field in record.iter() {
            row.push(field.to_string());
        }
        rows.push(row);
    }
    Ok(rows)
}

// get today total work time
// If the start time is empty, returns 0
// If the end time is empty, returns the elapsed time from start to current time
pub fn get_today_total_work_time(rows: Vec<Vec<String>>) -> Result<TotalWorkTime, Box<dyn Error>> {
    let mut total = 0;
    let mut unit = "";
    let today = Utc::now().date();
    for row in rows.iter() {
        let date = row[0].parse::<DateTime<Utc>>()?;
        if date.date() == today {
            let start = row[1].parse::<DateTime<Utc>>()?;
            let end = row[2].parse::<DateTime<Utc>>()?;
            let elapsed = end - start;
            total += elapsed.num_seconds();
            unit = "seconds";
        }
    }
    Ok(TotalWorkTime {
        total: Utc.timestamp(total, 0),
        unit: unit.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
}
