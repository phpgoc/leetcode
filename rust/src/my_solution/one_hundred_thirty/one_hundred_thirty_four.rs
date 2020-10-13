pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let len = gas.len();
    if len != cost.len() {
        return -1;
    }
    let mut costs = 0;
    let mut vec = vec![];
    let mut min = std::i32::MAX;
    let mut min_index = 0;
    for i in 0..len {
        costs = costs + gas[i] - cost[i];
        if costs < min {
            min = costs;
            min_index = i;
        }
        vec.push(costs);
    }
    //    println!("{:?}", vec);
    if vec.last().unwrap() < &0_i32 {
        return -1;
    }

    ((min_index + 1) % len) as i32
}
