mod covid_api;
use covid_api::{WorldData ,USAData, StateData, CountyData};
mod formatters;
use formatters::*;

use std::time::Duration;
use std::thread::sleep;



fn main() {
    loop {
        let mut display = String::from("\x1B[2J\x1B[1;1H");
        display.push_str(&do_world());
        display.push_str(&do_countries());
        display.push_str(&do_states());
        display.push_str(&do_counties());
        print!("{}",display);
        sleep(Duration::from_secs(15));
    }
}



fn do_world() -> String {
    let world = WorldData::get_world().unwrap();
    let mut display ="Worldwide:\n".to_string();
    display.push_str(&format!("    total:      {:12} {}\n", format_total(world.total_cases), format_new_neutral(world.new_cases)));
    display.push_str(&format!("    active:     {:12}\n", format_total(world.active_cases)));
    display.push_str(&format!("    deaths:     {:12} {}\n", format_total(world.total_deaths), format_new_bad(world.new_deaths)));
    display.push_str(&format!("    recovered:  {:12} {}\n", format_total(world.total_cases), format_new_good(world.new_recovered)));
    display.push('\n');
    display
}



fn do_countries() -> String {
    let country_list = USAData::get_list().unwrap();
    let mut display = String::new();
    for i in 0..2 {
        let country = &country_list[i];
        display.push_str(&format!("{}: {}\n", country.rank,country.name));
        display.push_str(&format!("    total:      {:12} {}\n", format_total(country.total_cases), format_new_neutral(country.new_cases)));
        display.push_str(&format!("    active:     {:12}\n", format_total(country.active_cases)));
        display.push_str(&format!("    deaths:     {:12} {}\n", format_total(country.total_deaths), format_new_bad(country.new_deaths)));
        display.push_str(&format!("    recovered:  {:12} {}\n", format_total(country.total_recovered), format_new_good(country.new_recovered)));
        display.push_str(&format!("    tested:     {:12}\n", format_total(country.total_tests)));
        display.push_str(&format!("    chance of catching:      {}\n", format_chance(country.case_every_x_ppl)));
        display.push_str(&format!("    chance of dying:         {}\n", format_chance(country.death_every_x_ppl)));
        display.push_str(&format!("    chance of being tested:  {}\n", format_chance(country.test_every_x_ppl)));
    }
    display.push('\n');
    display
}



fn do_states() -> String {
    let state_list = StateData::get_list().unwrap();
    let mut display = String::new();
    for state in state_list {
        if state.name == "California" {
            display.push_str(&format!("{}:\n", state.name));
            display.push_str(&format!("    total:      {:12} {}\n", format_total(state.total_cases), format_new_neutral(state.new_cases)));
            display.push_str(&format!("    active:     {:12}\n", format_total(state.active_cases)));
            display.push_str(&format!("    deaths:     {:12} {}\n", format_total(state.total_deaths), format_new_bad(state.new_deaths)));
            display.push_str(&format!("    tested:     {:12}\n", format_total(state.total_tests)));
        }
    }
    display.push('\n');
    display
}



fn do_counties() -> String {
    let county_list = CountyData::get_list("california").unwrap();
    let mut display = String::new();

    let mut l_name   = 0;
    let mut l_total  = 0;
    let mut l_death  = 0;
    let mut l_ncases = 0;
    for i in 0..3 {
        let county = &county_list[i];
        let n_len = county.name.len();
        if n_len > l_name {l_name = n_len}
        let n_total = format_total(county.total_cases).len();
        if n_total > l_total {l_total = n_total}
        let n_death = format_total(county.total_deaths).len();
        if n_death > l_death {l_death = n_death}
        let n_ncases = format_split_neutral(county.new_cases).len();
        if n_ncases > l_ncases {l_ncases = n_ncases}
    }

    for i in 0..3 {
        let county = &county_list[i];
        display.push_str(&format!("{r}: {n:np$} {t:tp$}/{d:dp$} {nt:ntp$}/{nd}\n",
            r=i+1,
            n=&county.name,
            np=l_name,
            t=format_total(county.total_cases),
            tp=l_total,
            d=format_total(county.total_deaths),
            dp=l_death,
            nt=format_split_neutral(county.new_cases),
            ntp=l_ncases,
            nd=format_split_bad(county.new_deaths),
        ))
    }
    display.push('\n');

    for county in county_list {
        if county.name == "Sacramento" {
            display.push_str(&format!("{}:\n", county.name));
            display.push_str(&format!("    total:      {:12} {}\n", format_total(county.total_cases), format_new_neutral(county.new_cases)));
            display.push_str(&format!("    active:     {:12}\n", format_total(county.active_cases)));
            display.push_str(&format!("    deaths:     {:12} {}\n", format_total(county.total_deaths), format_new_bad(county.new_deaths)));
            display.push_str(&format!("    tested:     {:12}", format_total(county.total_tests)));
        }
    }
    display
}