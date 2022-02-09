// impl From<AnnualInterestRate> for MonthlyInterestRate {
//     fn from(annual_rate: AnnualInterestRate) -> Self {
//         MonthlyInterestRate {rate: (1.0 + annual_rate.rate).powf(1.0 / 12.0) - 1.0}
//     }
// }
//
// fn print_monthly_rate<T: Into<MonthlyInterestRate>>(interest_rate: T) {
//     println!("{}", interest_rate.into().rate)
// }



struct MonthlyInterestRate {
    rate: f64
}

struct AnnualInterestRate {
    rate: f64
}

fn print_monthly_rate(interest_rate: MonthlyInterestRate) {
    println!("{}", interest_rate.rate)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let rate = MonthlyInterestRate { rate: 0.02};
        print_monthly_rate(rate);
    }









    // #[test]
    // fn run() {
    //     let rate = AnnualInterestRate { rate: 0.05};
    //     print_monthly_rate(rate);
    //     let rate = MonthlyInterestRate { rate: 0.02};
    //     print_monthly_rate(rate);
    // }
}
