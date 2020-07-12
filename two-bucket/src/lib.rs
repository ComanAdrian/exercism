#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}
#[derive(Debug, Clone, Copy)]
pub struct BucketState(u8, u8);
/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut first_bucket: BucketState = BucketState(0u8, capacity_1);
    let mut second_bucket: BucketState = BucketState(0u8, capacity_2);
    let mut moves: u8 = 0;

    let _ = match start_bucket {
        Bucket::One => {
            refill(&mut first_bucket);
        }
        Bucket::Two => {
            refill(&mut second_bucket);
        }
    };
    moves += 1;

    while moves != 254 && first_bucket.0 != goal && second_bucket.0 != goal {
        match (first_bucket.1 == goal, second_bucket.1 == goal) {
            (true, _) => {
                refill(&mut first_bucket);
            }
            (false, true) => {
                refill(&mut second_bucket);
            }
            (false, false) => match start_bucket {
                Bucket::One => {
                    match (
                        is_full(&first_bucket),
                        is_full(&second_bucket),
                        is_empty(&first_bucket),
                        is_empty(&second_bucket),
                    ) {
                        (true, _, _, _) => {
                            fill(&mut first_bucket, &mut second_bucket);
                        }
                        (_, true, _, _) => {
                            empty(&mut second_bucket);
                        }
                        (_, _, true, _) => {
                            refill(&mut first_bucket);
                        }
                        (_, _, _, true) => {
                            fill(&mut first_bucket, &mut second_bucket);
                        }
                        (_, _, _, _) => fill(&mut second_bucket, &mut first_bucket),
                    }
                }
                Bucket::Two => {
                    match (
                        is_full(&first_bucket),
                        is_full(&second_bucket),
                        is_empty(&first_bucket),
                        is_empty(&second_bucket),
                    ) {
                        (true, _, _, _) => {
                            empty(&mut first_bucket);
                        }
                        (_, true, _, _) => {
                            fill(&mut second_bucket, &mut first_bucket);
                        }
                        (_, _, true, _) => {
                            fill(&mut second_bucket, &mut first_bucket);
                        }
                        (_, _, _, true) => {
                            refill(&mut second_bucket);
                        }
                        (_, _, _, _) => fill(&mut second_bucket, &mut first_bucket),
                    }
                }
            },
        }
        moves += 1;
    }

    if moves == 254 {
        return None;
    }

    Some(if first_bucket.0 == goal {
        BucketStats {
            moves,
            goal_bucket: Bucket::One,
            other_bucket: second_bucket.0,
        }
    } else {
        BucketStats {
            moves,
            goal_bucket: Bucket::Two,
            other_bucket: first_bucket.0,
        }
    })
}

fn fill(from: &mut BucketState, to: &mut BucketState) {
    let capacity_left_in_to_bucket: u8 = to.1 - to.0;

    if from.0 >= capacity_left_in_to_bucket {
        from.0 = from.0 - capacity_left_in_to_bucket;
        to.0 = to.1;
    } else {
        to.0 = to.0 + from.0;
        from.0 = 0;
    }
}

fn empty(bucket: &mut BucketState) {
    bucket.0 = 0;
}

fn refill(bucket: &mut BucketState) {
    bucket.0 = bucket.1;
}

fn is_full(bucket: &BucketState) -> bool {
    bucket.0 == bucket.1
}

fn is_empty(bucket: &BucketState) -> bool {
    bucket.0 == 0
}
