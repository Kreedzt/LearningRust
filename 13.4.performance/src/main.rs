fn main() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    // 编译后遍历 `coefficients` 的值完全用不到循环: 会展开循环
    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>() >> qlp_shift;

        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
