//could definitely condense all of this down to a few functions easily

use colored::*;
use num_format::{Locale, ToFormattedString};

pub fn format_new_good(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("{:8} today", format!("+{}",i.to_formatted_string(&Locale::en)).green()),
        None => String::new(),
    }
}

pub fn format_new_neutral(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("{:8} today", format!("+{}",i.to_formatted_string(&Locale::en)).yellow()),
        None => String::new(),
    }
}

pub fn format_new_bad(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("{:8} today", format!("+{}",i.to_formatted_string(&Locale::en)).red()),
        None => String::new(),
    }
}

#[allow(dead_code)]
pub fn format_split_good(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("+{}",i.to_formatted_string(&Locale::en)).green().to_string(),
        None => "+0".green().to_string(),
    }
}

pub fn format_split_neutral(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("+{}",i.to_formatted_string(&Locale::en)).yellow().to_string(),
        None => "+0".yellow().to_string(),
    }
}

pub fn format_split_bad(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("+{}",i.to_formatted_string(&Locale::en)).red().to_string(),
        None => "+0".red().to_string(),
    }
}

pub fn format_total(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("{}",i.to_formatted_string(&Locale::en)),
        None => "N/A".to_string(),
    }
}

pub fn format_chance(n: Option<u64>) -> String {
    match n {
        Some(i) => format!("1/{}",i.to_formatted_string(&Locale::en)),
        None => "N/A".to_string(),
    }
}
