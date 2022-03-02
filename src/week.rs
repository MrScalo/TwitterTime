pub struct Week {
    pub monday: Vec<String>,
    pub tuesday: Vec<String>,
    pub wednesday: Vec<String>,
    pub thursday: Vec<String>,
    pub friday: Vec<String>,
    pub saturday: Vec<String>,
    pub sunday: Vec<String>,
}

impl Week {
    pub fn new() -> Week {
        Week {
            monday: vec![],
            tuesday: vec![],
            wednesday: vec![],
            thursday: vec![],
            friday: vec![],
            saturday: vec![],
            sunday: vec![]
        }
    }

    pub fn sort(&mut self) {
        self.monday.sort();
        self.tuesday.sort();
        self.wednesday.sort();
        self.thursday.sort();
        self.friday.sort();
        self.saturday.sort();
        self.sunday.sort();
    } 
}

pub struct WeekMeta {
    pub week: Week,
    pub next: String,
    pub last_date: String,
}

impl WeekMeta {
    pub fn new() -> WeekMeta {
        WeekMeta {
            week: Week::new(),
            next: String::new(),
            last_date: String::new(),
        }
    }
}