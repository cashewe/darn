/// clear type enforcement for punishment functions
/// let there be no ambiguity in this file oh please no
pub type PunishmentFn = fn(usize, usize, &mut [usize]);

/// constant punishment, barely a function tbh
pub fn const_punishment(scale: usize, length: usize, out: &mut [usize]) {
    for i in 0..length {
        out[i] = scale;
    }
}

/// punishment gets worse the further in you go
pub fn linear_punishment(scale: usize, length: usize, out: &mut [usize]) {
    for i in 0..length {
        out[i] = i + 1;
    }
}

/// punishment gets better the further in you go
pub fn reverse_linear_punishment(scale: usize, length: usize, out: &mut [usize]) {
    let start = 50usize;

    for i in 0..length {
        out[i] = start.saturating_sub(i);
    }
}

/// inverse triangular punishment is cheapest in the center
pub fn inverse_triangular_punishment(scale: usize, length: usize, out: &mut [usize]) {
    if length == 0 {
        return;
    }
    if length == 1 {
        out[0] = 0;
        return;
    }
    
    let mid = (length - 1) as f64 / 2.0;
    let max_distance = mid;

    for i in 0..length {
        let distance_from_mid = (i as f64 - mid).abs();
        let normalized = distance_from_mid / max_distance;
        let value = (normalized * scale as f64).round() as usize;
        out[i] = value;
    }
}

/// triangular punishment is worst in the center
pub fn triangular_punishment(scale: usize, length: usize, out: &mut [usize]) {
    if length == 0 {
        return;
    }
    if length == 1 {
        out[0] = scale;
        return;
    }

    let mid = (length - 1) as f64 / 2.0;
    let max_distance = mid;
    for i in 0..length {
        let distance_from_mid = (i as f64 - mid).abs();
        let normalized = distance_from_mid / max_distance;
        let value = ((1.0 - normalized) * scale as f64).round() as usize;
        out[i] = value;
    }
}

/// used to access the functions via string match
pub fn lookup_punishment(name: &str) -> Option<PunishmentFn> {
    match name {
        "const" => Some(const_punishment),
        "linear" => Some(linear_punishment),
        "reverse_linear" => Some(linear_punishment),
        "inverse_triangular" => Some(inverse_triangular_punishment),
        "reverse_linear" => Some(reverse_linear_punishment),
        _ => None,
    }
}