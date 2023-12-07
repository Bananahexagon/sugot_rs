pub fn flatten_vec<T>(vv: Vec<Vec<T>>) -> Vec<T> {
    let mut result = Vec::new();
    for v in vv {
        for p in v {
            result.push(p);
        }
    }
    result
}