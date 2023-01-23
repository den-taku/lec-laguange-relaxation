fn main() {
    let mut u = [0.0, 1.0, 0.0, 3.0, 1.0, 1.5, 1.5];
    let c = [4.0, 6.0, 7.0, 2.0, 3.0];
    let mut x = [0, 0, 0, 0, 0];
    let a = [
        true, false, false, true, false, //
        false, true, false, false, true, //
        true, false, false, true, false, //
        true, true, true, false, false, //
        false, true, false, false, true, //
        false, false, true, true, false, //
        false, false, true, false, true,
    ];
    for t in 0..3 {
        println!("{}:", t + 1);
        let mut cost = [0.0, 0.0, 0.0, 0.0, 0.0];
        for j in 0..5 {
            cost[j] = c[j];
            for i in 0..7 {
                if a[i * 5 + j] {
                    cost[j] -= u[i]
                }
            }
        }
        println!("  cost: {cost:?}");

        for j in 0..5 {
            x[j] = if cost[j] < 0.0 { 1 } else { 0 };
        }
        println!("  x: {x:?}");

        let mut coeff = [1, 1, 1, 1, 1, 1, 1];
        for i in 0..7 {
            for j in 0..5 {
                if a[i * 5 + j] {
                    coeff[i] -= x[j]
                }
            }
        }
        println!("  coeff: {coeff:?}");

        let mut value = 0.0;
        for j in 0..5 {
            if x[j] == 1 {
                value += c[j]
            }
        }
        for i in 0..7 {
            value += u[i] * (coeff[i] as f64)
        }
        println!(
            "  value: {value} -> true value: {}",
            x.iter()
                .enumerate()
                .map(|(i, a)| if *a == 1 { c[i] } else { 0.0 })
                .sum::<f64>()
        );

        for i in 0..7 {
            let mut ax = 0;
            for j in 0..5 {
                if a[i * 5 + j] {
                    ax += x[j]
                }
            }
            u[i] = u[i] + 0.5 * ((1 - ax) as f64);
            if u[i] < 0.0 {
                u[i] = 0.0;
            }
        }
        println!("  u: {u:?}")
    }
}
