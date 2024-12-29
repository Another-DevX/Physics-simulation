pub fn rk4<F>(a: f32, b: f32, alpha: [f32; 3], f: F, n: u32) -> (Vec<f32>, Vec<Vec<f32>>)
where
    F: Fn(f32, &[f32; 3]) ->  [f32; 3],
{
    let h = (b - a) / n as f32;
    let mut w = vec![alpha.to_vec()];
    let mut t_values = vec![a];

    for i in 1..=n {
        let t = a + i as f32 * h;
        let w_prev = &w[i as usize - 1];
        let w_prev_array: [f32; 3] = [w_prev[0], w_prev[1], w_prev[2]];

        let k1: Vec<f32> = f(t, &w_prev_array).iter().map(|&val| h * val).collect();
        let k2: Vec<f32> = f(
            t + h / 2.0,
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
            t + h / 2.0,
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
            t + h,
            &[
                w_prev_array[0] + k3[0],
                w_prev_array[1] + k3[1],
                w_prev_array[2] + k3[2],
            ],
        )
        .iter()
        .map(|&val| h * val)
        .collect();

        let w_next = w_prev
            .iter()
            .zip(&k1)
            .zip(&k2)
            .zip(&k3)
            .zip(&k4)
            .map(|((((&w_i, &k1_i), &k2_i), &k3_i), &k4_i)| {
                w_i + (k1_i + 2.0 * k2_i + 2.0 * k3_i + k4_i) / 6.0
            })
            .collect();
        w.push(w_next);
        t_values.push(t)
    }

    (t_values, w)
}
