pub fn extract_common_len<T>(vec_of_vecs: &[Vec<T>]) -> u8 {
    let all_same_len = vec_of_vecs.iter().all(|v| v.len() == vec_of_vecs[0].len());
    match all_same_len {
        false => panic!("Some values have nonequal length"),
        true => vec_of_vecs.get(0).unwrap().len() as u8,
    }
}
