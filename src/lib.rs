


pub fn npv(mortality_rates: &[f32], lapse_rates: &[f32], interest_rate: f32, sum_assured: f32, premium: f32, init_pols: f32, term: Option<usize>) -> f32 {

    let term = term.unwrap_or_else(|| mortality_rates.len());
    let mut result = 0.0;
    let mut inforce = init_pols;
    let v = 1.0 / (1.0 + interest_rate);

    for (t, (q, w)) in mortality_rates.iter().zip(lapse_rates).enumerate() {
        let no_deaths = if t < term {inforce * q} else {0.0};
        let no_lapses = if t < term {inforce * w} else {0.0};
        let premiums = inforce * premium;
        let claims = no_deaths * sum_assured;
        let net_cashflow = premiums - claims;
        result += net_cashflow * v.powi(t as i32);
        inforce = inforce - no_deaths - no_lapses;
    }

    result
}








#[cfg(test)]
mod tests {
    use crate::npv;

    #[test]
    fn simple_test() {
        let mortality_rates: [f32; 10] = [0.001, 0.002, 0.003, 0.003, 0.004, 0.004, 0.005, 0.007, 0.009, 0.011];
        let lapse_rates: [f32; 10] = [0.05, 0.07, 0.08, 0.10, 0.14, 0.20, 0.20, 0.20, 0.10, 0.04];
        let premium: f32 = 100.0;
        let sum_assured: f32 = 25000.0;
        let interest_rate: f32 = 0.02;
        assert_eq!(npv(&mortality_rates, &lapse_rates, interest_rate, sum_assured, premium, 1.0, None), 51.331318);
    }

}
