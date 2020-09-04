
use select::document::Document;
use select::predicate::*;
use reqwest::blocking;

use std::fmt;
use std::error;

const WORLD_DATA_URL: &str  = "https://www.worldometers.info/coronavirus/";
const COUNTRY_DATA_URL: &str = "https://www.worldometers.info/coronavirus/country/us/";
const STATE_DATA_URL: &str = "https://www.worldometers.info/coronavirus/usa/";



#[derive(Debug)]
pub enum CustomError {
    UrlError(String, u16),
    TableError,
}
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::UrlError(url, code)    => write!(f, "UrlError: {} returned status code {}",url,code),
            CustomError::TableError             => write!(f, "TableError: Couldn't parse table value (region may have no data)"),
        }
    }
}
impl error::Error for CustomError {}



pub struct WorldData {
    pub name:               String,
    pub total_cases:        Option<u64>,
    pub new_cases:          Option<u64>,
    pub total_deaths:       Option<u64>,
    pub new_deaths:         Option<u64>,
    pub total_recovered:    Option<u64>,
    pub new_recovered:      Option<u64>,
    pub active_cases:       Option<u64>,
    pub serious_critical:   Option<u64>,
    pub tot_cases_1m_pop:   Option<u64>,
    pub deaths_1m_pop:      Option<u64>,
    pub total_tests:        Option<u64>,
    pub tests_1m_pop:       Option<u64>,
    pub population:         Option<u64>,
    pub case_every_x_ppl:   Option<u64>,
    pub death_every_x_ppl:  Option<u64>,
    pub test_every_x_ppl:   Option<u64>,
}
impl WorldData {
    pub fn new(name: String, data: Vec<Option<u64>>) -> Self {
        Self {
            name,
            total_cases:        data[0],
            new_cases:          data[1],
            total_deaths:       data[2],
            new_deaths:         data[3],
            total_recovered:    data[4],
            new_recovered:      data[5],
            active_cases:       data[6],
            serious_critical:   data[7],
            tot_cases_1m_pop:   data[8],
            deaths_1m_pop:      data[9],
            total_tests:        data[10],
            tests_1m_pop:       data[11],
            population:         data[12],
            case_every_x_ppl:   data[13],
            death_every_x_ppl:  data[14],
            test_every_x_ppl:   data[15],
        }
    }

    pub fn get_world() -> Result<WorldData, Box<dyn error::Error>> {
        let document = get_html(WORLD_DATA_URL)?;
        let table = document.find(Attr("id","main_table_countries_today")).nth(0).ok_or(CustomError::TableError)?;
        let tbody = table.find(Name("tbody")).nth(0).ok_or(CustomError::TableError)?;
        let mut tr = tbody.find(Name("tr"));
        let mut td = tr.nth(7).ok_or(CustomError::TableError)?.find(Name("td"));

        let name = td.nth(1).ok_or(CustomError::TableError)?.text();
        let data = td.map(|d|
            if let Ok(n) = d.text().replace(",","").parse::<u64>() {Some(n)} else {None}
        ).collect::<Vec<Option<u64>>>();

        Ok(WorldData::new(name,data))
    }
}



pub struct USAData {
    pub rank:               u64,
    pub name:               String,
    pub total_cases:        Option<u64>,
    pub new_cases:          Option<u64>,
    pub total_deaths:       Option<u64>,
    pub new_deaths:         Option<u64>,
    pub total_recovered:    Option<u64>,
    pub new_recovered:      Option<u64>,
    pub active_cases:       Option<u64>,
    pub serious_critical:   Option<u64>,
    pub tot_cases_1m_pop:   Option<u64>,
    pub deaths_1m_pop:      Option<u64>,
    pub total_tests:        Option<u64>,
    pub tests_1m_pop:       Option<u64>,
    pub population:         Option<u64>,
    pub case_every_x_ppl:   Option<u64>,
    pub death_every_x_ppl:  Option<u64>,
    pub test_every_x_ppl:   Option<u64>,
}
impl USAData {
    pub fn new(name: String, rank: u64, data: Vec<Option<u64>>) -> Self {
        USAData {
            rank,
            name,
            total_cases:        data[0],
            new_cases:          data[1],
            total_deaths:       data[2],
            new_deaths:         data[3],
            total_recovered:    data[4],
            new_recovered:      data[5],
            active_cases:       data[6],
            serious_critical:   data[7],
            tot_cases_1m_pop:   data[8],
            deaths_1m_pop:      data[9],
            total_tests:        data[10],
            tests_1m_pop:       data[11],
            population:         data[12],
            case_every_x_ppl:   data[14],
            death_every_x_ppl:  data[15],
            test_every_x_ppl:   data[16],
        }
    }

    pub fn get_list() -> Result<Vec<USAData>, Box<dyn error::Error>> {
        let document = get_html(WORLD_DATA_URL)?;
        let table = document.find(Attr("id","main_table_countries_today")).nth(0).ok_or(CustomError::TableError)?;
        let tbody = table.find(Name("tbody")).nth(0).ok_or(CustomError::TableError)?;
        let mut tr = tbody.find(Name("tr"));
        for _ in 0..8 {tr.next();}

        let mut country_list: Vec<USAData> = Vec::new();
        for tr_node in tr {
            let mut td = tr_node.find(Name("td"));
            let rank = td.next().ok_or(CustomError::TableError)?.text().parse::<u64>().unwrap();
            let name = td.next().ok_or(CustomError::TableError)?.text();
            let data = td.map(|d|
                    if let Ok(n) = d.text().replace(",","").trim().parse::<u64>() {Some(n)} else {None}
            ).collect::<Vec<Option<u64>>>();
            country_list.push(USAData::new(name,rank,data));
        }

        Ok(country_list)
    }
}



pub struct StateData {
    pub rank:               u64,
    pub name:               String,
    pub total_cases:        Option<u64>,
    pub new_cases:          Option<u64>,
    pub total_deaths:       Option<u64>,
    pub new_deaths:         Option<u64>,
    pub total_recovered:    Option<u64>,
    pub active_cases:       Option<u64>,
    pub tot_cases_1m_pop:   Option<u64>,
    pub deaths_1m_pop:      Option<u64>,
    pub total_tests:        Option<u64>,
    pub tests_1m_pop:       Option<u64>,
    pub population:         Option<u64>,
}
impl StateData {
    pub fn new(name: String, rank: u64, data: Vec<Option<u64>>) -> Self {
        Self {
            rank,
            name,
            total_cases:        data[0],
            new_cases:          data[1],
            total_deaths:       data[2],
            new_deaths:         data[3],
            total_recovered:    data[4],
            active_cases:       data[5],
            tot_cases_1m_pop:   data[6],
            deaths_1m_pop:      data[7],
            total_tests:        data[8],
            tests_1m_pop:       data[9],
            population:         data[10],
        }
    }

    pub fn get_list() -> Result<Vec<StateData>, Box<dyn error::Error>> {
        let document = get_html(COUNTRY_DATA_URL)?;
        let table = document.find(Attr("id","usa_table_countries_today")).nth(0).ok_or(CustomError::TableError)?;
        let tbody = table.find(Name("tbody")).nth(0).ok_or(CustomError::TableError)?;
        let mut tr = tbody.find(Name("tr"));
        tr.next();

        let mut state_list: Vec<StateData> = Vec::new();
        for tr_node in tr {
            let mut td = tr_node.find(Name("td"));
            let rank = td.next().ok_or(CustomError::TableError)?.text().parse::<u64>().unwrap();
            let name = td.next().ok_or(CustomError::TableError)?.text().replace("\n","");
            let data = td.map(|d|
                if let Ok(n) = d.text().replace(",","").replace("\n","").replace(" ","").parse::<u64>() {Some(n)} else {None}
            ).collect::<Vec<Option<u64>>>();
            state_list.push(StateData::new(name.trim().to_string(),rank,data));
        }

        Ok(state_list)
    }
}



pub struct CountyData {
    pub name:               String,
    pub total_cases:        Option<u64>,
    pub new_cases:          Option<u64>,
    pub total_deaths:       Option<u64>,
    pub new_deaths:         Option<u64>,
    pub active_cases:       Option<u64>,
    pub total_tests:        Option<u64>,
}
impl CountyData {
    pub fn new(name: String, data: Vec<Option<u64>>) -> Self {
        Self {
            name,
            total_cases:        data[0],
            new_cases:          data[1],
            total_deaths:       data[2],
            new_deaths:         data[3],
            active_cases:       data[4],
            total_tests:        data[5],
        }
    }

    pub fn get_list(state: &str) -> Result<Vec<CountyData>, Box<dyn error::Error>> {
        let document = get_html(&format!("{}{}",STATE_DATA_URL,state))?;
        let table = document.find(Attr("id","usa_table_countries_today")).nth(0).ok_or(CustomError::TableError)?;
        let tbody = table.find(Name("tbody")).nth(0).ok_or(CustomError::TableError)?;
        let mut tr = tbody.find(Name("tr"));
        tr.next();

        let mut county_list: Vec<CountyData> = Vec::new();
        for tr_node in tr {
            let mut td = tr_node.find(Name("td"));
            let name = td.next().ok_or(CustomError::TableError)?.text().replace("\n","");
            let data = td.map(|d|
                if let Ok(n) = d.text().replace(",","").replace("\n","").replace(" ","").parse::<u64>() {Some(n)} else {None}
            ).collect::<Vec<Option<u64>>>();
            county_list.push(CountyData::new(name.trim().to_string(),data));
        }

        Ok(county_list)
    }
}

pub fn get_html(url: &str) -> Result <Document, Box<dyn error::Error>>{
    let resp = blocking::get(url)?;
    if !resp.status().is_success() {
        return Err(Box::new(CustomError::UrlError(
            url.to_string(),
            resp.status().as_u16()
        )))
    };
    Ok(Document::from_read(resp)?)
}