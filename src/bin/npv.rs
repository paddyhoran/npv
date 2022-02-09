use npv::npv;
// discounted_net_cash_flow = net_cash_flow * (discount_rate ** t)
// # discounted_net_cash_flow = net_cash_flow  ** discount_rate
// # discounted_net_cash_flow = math.pow(net_cash_flow, discount_rate)

fn main() {
    // stable-x86_64-pc-windows-msvc
    // cargo 1.49.0 (d00d64df9 2020-12-05)
    // rustc 1.49.0 (e1884a8e3 2020-12-29)
    let mortality_rates: [f64; 10] = [0.001, 0.002, 0.003, 0.003, 0.004, 0.004, 0.005, 0.007, 0.009, 0.011];
    let lapse_rates: [f64; 10] = [0.05, 0.07, 0.08, 0.10, 0.14, 0.20, 0.20, 0.20, 0.10, 0.04];
    let premium: f64 = 100.0;
    let sum_assured: f64 = 25000.0;
    let interest_rate: f64 = 0.02;
    // loop {
        npv(&mortality_rates, &lapse_rates, interest_rate, sum_assured, premium, 1.0, None);
    // }
}
