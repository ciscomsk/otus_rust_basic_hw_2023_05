fn main() {
    let res = array_sum([i32::MAX; 3]);
    println!("{:?}", res);
}

pub fn double_int32(num: u32) -> (u32, bool) {
    num.overflowing_mul(2)
}

pub fn double_int64(num: u32) -> u64 {
    num as u64 * 2
}

pub fn double_float32(num: f32) -> f32 {
    num * 2.0
}

pub fn double_float64(num: f32) -> f64 {
    num as f64 * 2.0
}

pub fn int_plus_float_to_float(int: u32, float: f32) -> f64 {
    int as f64 + float as f64
}

pub fn int_plus_float_to_int(int: u32, float: f32) -> (u64, bool) {
    (int as u64).overflowing_add(float as u64)
}

pub fn tuple_sum(tup: (i32, i32)) -> (i32, bool) {
    (tup.0).overflowing_add(tup.1)
}

pub fn array_sum(arr: [i32; 3]) -> (i32, bool) {
    let part_sum: (i32, bool) = (arr[0]).overflowing_add(arr[1]);

    match part_sum {
        (num, false) => num.overflowing_add(arr[2]),

        _ => part_sum,
    }
}
