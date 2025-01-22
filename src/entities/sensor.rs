use rand::Rng;

pub fn generate_sensor_reading(min: f32, max: f32, error_rate: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let range = max - min;
    let actual_min = min - (range * (error_rate/2.0));
    let actual_max = max + (range * (error_rate/2.0));

    return rng.gen_range(actual_min..actual_max);
}
