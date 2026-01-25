/// clear type enforcement for punishment functions
/// let there be no ambiguity in this file oh please no
type PunishmentFn = fn(usize, &mut [i32]);

/// no punishment is incurred
fn zero_punishment(length: usize) -> Vec<i32> {
    (0..length).collect()
}

/// 50 punishment is incurred
fn fifty_punishment(length: usize) -> Vec<i32> {
    vec![50; length]
}

/// punishment gets worse the further in you go
fn linear_punishment(length: usize) -> Vec<i32> {
    (0..length).map(|i| i + 1).collect()
}

/// punishment gets better the further you go
fn inverse_punishment(length: usize) -> Vec<i32> {
    (0..length).map(|i| length - i).collect()
}

/// each additional unit of length incurs double the punishment of the previous unit
fn exponential_punishment(length: usize) -> Vec<i32> {
    (0..length).map(|i| 2_i32.pow(i as u32)).collect()
}

/// inverse triangular punishment is cheapest in the center
fn inverse_triangular_punishment(length: usize) -> Vec<i32> {
    if length == 0 {
        return Vec::new();
    }
    if length == 1 {
        return vec![0];
    }
    
    let mut result = Vec::with_capacity(length as usize);
    let max_distance = (length - 1) as f64 / 2.0;
    
    for i in 0..length {
        let mid = (length - 1) as f64 / 2.0;
        let distance_from_mid = (i as f64 - mid).abs();
        let normalized = distance_from_mid / max_distance;
        let value = (normalized * 50.0).round() as i32;
        result.push(value);
    }
    
    result
}