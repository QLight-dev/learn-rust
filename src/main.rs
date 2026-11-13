struct Date {
    day: i32,
    month: i32,
    year: i32,
}

impl Date {
    fn return_date(&self) -> String {
        format!("{}-{}-{}", self.day, self.month, self.year)
    }
}

impl Date {
    fn advance_year(&mut self) {
        self.year += 1
    }
}

fn main() {
    let mut isaac = Date {
        day: 4,
        month: 1,
        year: 1643,
    };

    println!("Isaac Newton was born in {}", isaac.return_date());

    Date::advance_year(&mut isaac);
    println!("The year after his birth was  {}", isaac.year);
}
