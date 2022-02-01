MORTALITY_RATES = {1: 0.001, 2: 0.002, 3: 0.003, 4: 0.003, 5: 0.004, 6: 0.004, 7: 0.005, 8: 0.007, 9: 0.009, 10: 0.011}
LAPSE_RATES = {1:0.05, 2: 0.07, 3: 0.08, 4: 0.10, 5: 0.14, 6: 0.20, 7: 0.20, 8: 0.20, 9: 0.10, 10: 0.04}
PREMIUM = 100.0
SUM_ASSURED = 25000.0
INTEREST_RATE = 0.02


def main():

    npv = PREMIUM
    lx = 1.0
    discount_rate = 1.0 / (1.0 + INTEREST_RATE)

    for t in range(1, 11):
        deaths = lx * MORTALITY_RATES[t]
        lapses = lx * LAPSE_RATES[t]
        lx = lx - deaths - lapses
        premium = lx * PREMIUM
        claims = deaths * SUM_ASSURED
        net_cash_flow = premium - claims
        discounted_net_cash_flow = net_cash_flow * (discount_rate ** t)
        npv += discounted_net_cash_flow

    return npv


print(main())