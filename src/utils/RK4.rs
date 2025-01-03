pub fn rk42nd_order<FG, FO>(
    a: f32,
    b: f32,
    gamma0: f32,
    omega0: f32,
    fgamma: FG,
    fomega: FO,
    n: u32,
) -> (Vec<f32>, Vec<f32>, Vec<f32>)
where
    FG: Fn(f32, f32, f32) -> f32,
    FO: Fn(f32, f32, f32) -> f32,
{
    let h = (b - a) / n as f32;
    let mut omega = vec![omega0];
    let mut gamma = vec![gamma0];
    let mut t = vec![a];

    for i in 1..=n {
        let ti = a + i as f32 * h;
        let omega_prev = omega[i as usize - 1];
        let gamma_prev = gamma[i as usize - 1];

        // Compute k1 values
        let gamma_k1 = fgamma(ti, gamma_prev, omega_prev);
        let omega_k1 = fomega(ti, gamma_prev, omega_prev);

        // Compute k2 values
        let gamma_k2 = fgamma(
            ti + h / 2.0,
            gamma_prev + h * gamma_k1 / 2.0,
            omega_prev + h * omega_k1 / 2.0,
        );
        let omega_k2 = fomega(
            ti + h / 2.0,
            gamma_prev + h * gamma_k1 / 2.0,
            omega_prev + h * omega_k1 / 2.0,
        );

        // Compute k3 values
        let gamma_k3 = fgamma(
            ti + h / 2.0,
            gamma_prev + h * gamma_k2 / 2.0,
            omega_prev + h * omega_k2 / 2.0,
        );
        let omega_k3 = fomega(
            ti + h / 2.0,
            gamma_prev + h * gamma_k2 / 2.0,
            omega_prev + h * omega_k2 / 2.0,
        );

        // Compute k4 values
        let gamma_k4 = fgamma(ti + h, gamma_prev + h * gamma_k3, omega_prev + h * omega_k3);
        let omega_k4 = fomega(ti + h, gamma_prev + h * gamma_k3, omega_prev + h * omega_k3);

        // Update gamma and omega using weighted average of k1, k2, k3, k4
        let gamma_next =
            gamma_prev + (h / 6.0) * (gamma_k1 + 2.0 * gamma_k2 + 2.0 * gamma_k3 + gamma_k4);
        let omega_next =
            omega_prev + (h / 6.0) * (omega_k1 + 2.0 * omega_k2 + 2.0 * omega_k3 + omega_k4);

        gamma.push(gamma_next);
        omega.push(omega_next);
        t.push(ti);
    }
    (t, gamma, omega)
}

pub fn rk4<F>(a: f32, b: f32, alpha: Vec<f32>, f: F, n: u32) -> (Vec<f32>, Vec<Vec<f32>>)
where
    F: Fn(f32, &[f32]) -> Vec<f32>,
{
    let h = (b - a) / n as f32;
    let mut w = vec![alpha.to_vec()];
    let mut t = vec![a];

    for i in 1..=n {
        let ti = a + i as f32 * h;
        let w_prev = &w[i as usize - 1];
        let w_prev_array: [f32; 3] = [w_prev[0], w_prev[1], w_prev[2]];

        let k1: Vec<f32> = f(ti, &w_prev_array).iter().map(|&val| h * val).collect();
        let k2: Vec<f32> = f(
            ti + h / 2.0,
            &[
                w_prev_array[0] + k1[0] / 2.0,
                w_prev_array[1] + k1[1] / 2.0,
                w_prev_array[2] + k1[2] / 2.0,
            ],
        )
        .iter()
        .map(|&val| h * val)
        .collect();
        let k3: Vec<f32> = f(
            ti + h / 2.0,
            &[
                w_prev_array[0] + k2[0] / 2.0,
                w_prev_array[1] + k2[1] / 2.0,
                w_prev_array[2] + k2[2] / 2.0,
            ],
        )
        .iter()
        .map(|&val| h * val)
        .collect();
        let k4: Vec<f32> = f(
            ti + h,
            &[
                w_prev_array[0] + k3[0],
                w_prev_array[1] + k3[1],
                w_prev_array[2] + k3[2],
            ],
        )
        .iter()
        .map(|&val| h * val)
        .collect();

        let w_next = (0..w_prev.len())
            .map(|i| {
                let w_i = w_prev[i];
                let k1_i = k1[i];
                let k2_i = k2[i];
                let k3_i = k3[i];
                let k4_i = k4[i];
                w_i + (k1_i + 2.0 * k2_i + 2.0 * k3_i + k4_i) / 6.0
            })
            .collect();
        w.push(w_next);
        t.push(ti)
    }

    (t, w)
}
