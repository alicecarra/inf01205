pub fn get_max_delay(delays: Vec<usize>) -> usize {
    delays.into_iter().fold(0, std::cmp::max)
}
