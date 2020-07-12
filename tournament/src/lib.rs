use std::cmp::Ordering;
use std::collections::HashMap;

const TABLE_HEAD: &str = "Team                           | MP |  W |  D |  L |  P";
const TABLE_LINE_SEPARATOR: &str = "; ;";
const TABLE_TEAM_NAME_COLUMN_LENGTH: usize = 31;

enum Result {
    Win,
    Draw,
    Loss,
}

impl Result {
    fn from(result: &str) -> Self {
        match result {
            "win" => Result::Win,
            "draw" => Result::Draw,
            "loss" => Result::Loss,
            _ => Result::Loss,
        }
    }

    fn from_oposite(result: &str) -> Self {
        match result {
            "loss" => Result::Win,
            "draw" => Result::Draw,
            "win" => Result::Loss,
            _ => Result::Loss,
        }
    }
}

#[derive(Hash, Eq, Ord, PartialEq, Debug)]
struct Record {
    pub team_name: String,
    pub wins: u16,
    pub draws: u16,
    pub losses: u16,
}

impl Record {
    const WIN_POINTS: u16 = 3;

    pub fn new(team_name: &str, result: Result) -> Record {
        let mut record = Record {
            team_name: team_name.to_string(),
            wins: 0,
            draws: 0,
            losses: 0,
        };

        record.add_result(result);

        record
    }

    pub fn matches_played(&self) -> u16 {
        self.wins + self.draws + self.losses
    }

    fn add_result(&mut self, result: Result) {
        match result {
            Result::Win => self.wins += 1,
            Result::Draw => self.draws += 1,
            Result::Loss => self.losses += 1,
        }
    }

    pub fn points_count(&self) -> u16 {
        self.wins * Record::WIN_POINTS + self.draws
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order_by_points = other.points_count().cmp(&self.points_count());
        let order_alphabetically = other.team_name.cmp(&self.team_name);

        Some(match (order_by_points, order_alphabetically) {
            (Ordering::Equal, Ordering::Less) => Ordering::Greater,
            (Ordering::Equal, Ordering::Greater) => Ordering::Less,
            _ => order_by_points,
        })
    }
}

pub fn tally(match_results: &str) -> String {
    if match_results.is_empty() {
        return TABLE_HEAD.to_string();
    }
    let mut records: HashMap<String, Record> = HashMap::new();
    let match_results_without_newline = match_results.replace("\n", TABLE_LINE_SEPARATOR);
    let splitted = match_results_without_newline
        .split(';')
        .collect::<Vec<&str>>();

    let records_vector: Vec<_> = splitted.split(|&val| val == " ").collect();

    for record in records_vector.into_iter() {
        let first_team: (&str, Result) = (record[0], Result::from(record[2]));
        let second_team: (&str, Result) = (record[1], Result::from_oposite(record[2]));

        insert(&mut records, first_team);
        insert(&mut records, second_team);
    }

    let mut new_vec_records = records.into_iter().map(|it| it.1).collect::<Vec<_>>();
    new_vec_records.sort();
    create_table(new_vec_records)
}

fn insert(records: &mut HashMap<String, Record>, team: (&str, Result)) -> () {
    if records.get(team.0).is_none() {
        let team_record = Record::new(team.0, team.1);
        records.insert(team_record.team_name.clone(), team_record);
    } else {
        records.get_mut(team.0).unwrap().add_result(team.1);
    }
}

fn create_table(records: Vec<Record>) -> String {
    let mut head = TABLE_HEAD.to_string();
    head.push_str("\n");

    for record in records.into_iter() {
        let whitespaces = TABLE_TEAM_NAME_COLUMN_LENGTH - record.team_name.len();
        head.push_str(record.team_name.as_str());
        head.push_str(" ".repeat(whitespaces).as_str());

        head.push_str(format_cell(record.matches_played()).as_str());
        head.push_str(format_cell(record.wins).as_str());
        head.push_str(format_cell(record.draws).as_str());
        head.push_str(format_cell(record.losses).as_str());
        head.push_str(format_cell(record.points_count()).trim_end());

        head.push_str("\n");
    }

    head.truncate(head.len() - 1);

    head
}

fn format_cell(count: u16) -> String {
    format!("|  {} ", count)
}
