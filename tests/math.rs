use soundchip::math::{quantize_range, remap_range};

#[test]
fn quantization_test() {
    let mut last_value = 0.0;
    let mut value_count = 0;
    let steps = 5;
    for n in -10 ..= 10 {
        let value = n as f32 / 10.0;
        let result = quantize_range(value, steps, -1.0 ..= 1.0);
        if result != last_value {
            last_value = result;
            value_count += 1;
        }
        // println!("{:.3} => {:.3}", value, result);
    }
    assert_eq!(steps, value_count);
}


#[test]
fn remap_test(){
    let a = remap_range(1.0, &(1.0 ..= 2.0), &(5.0 ..= 10.0));
    assert_eq!(a, 5.0);

    let b = remap_range(2.0, &(1.0 ..= 2.0), &(5.0 ..= 10.0));
    assert_eq!(b, 10.0);

    let c = remap_range(1.5, &(1.0 ..= 2.0), &(5.0 ..= 10.0));
    assert_eq!(c, 7.5);

    let d = remap_range(0.0, &(-1.0 ..= 1.0), &(0.0 ..= 1.0));
    assert_eq!(d, 0.5);

    let d = remap_range(0.5, &(0.0 ..= 1.0), &(-1.0 ..= 1.0));
    assert_eq!(d, 0.0);

    let d = remap_range(0.0, &(0.0 ..= 1.0), &(-1.0 ..= 1.0));
    assert_eq!(d, -1.0);

    // Inverted range
    let e = remap_range(0.0, &(0.0 ..= 1.0), &(0.0 ..= -1.0));
    assert_eq!(e, 0.0);

    let f = remap_range(1.0, &(0.0 ..= 1.0), &(0.0 ..= -1.0));
    assert_eq!(f, -1.0);
}
