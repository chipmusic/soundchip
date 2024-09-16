use soundchip::math::quantize_range_f32;

#[test]
fn quantization_test() {
    let mut last_value = 0.0;
    let mut value_count = 0;
    let steps = 5;
    for n in -10 ..= 10 {
        let value = n as f32 / 10.0;
        let result = quantize_range_f32(value, steps, -1.0 ..= 1.0);
        if result != last_value {
            last_value = result;
            value_count += 1;
        }
        // println!("{:.3} => {:.3}", value, result);
    }
    assert_eq!(steps, value_count);
}
