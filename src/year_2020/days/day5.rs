use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_5.txt");
    let seat_id_iter = lines.lines().map(|seat| {
        id(seat
            .chars()
            .map(|f| if f == 'B' || f == 'R' { '1' } else { '0' })
            .collect::<String>()
            .as_str())
    });
    match part {
        1 => solve01(seat_id_iter),
        2 => solve02(seat_id_iter),
        _ => 1,
    }
}

fn solve01(seat_iter: impl Iterator<Item = usize>) -> usize {
    seat_iter.max().unwrap()
}

fn solve02(seat_id_iter: impl Iterator<Item = usize>) -> usize {
    let mut seat_list = seat_id_iter.collect_vec();
    seat_list.sort();
    seat_list
        .windows(2)
        .find_map(|w| {
            if w[1] - w[0] == 2 {
                Some(w[0] + 1)
            } else {
                None
            }
        })
        .unwrap()
}

fn id(seat: &str) -> usize {
    let row = usize::from_str_radix(seat.split_at(7).0, 2).unwrap();
    let col = usize::from_str_radix(seat.split_at(7).1, 2).unwrap();
    8 * row + col
}
