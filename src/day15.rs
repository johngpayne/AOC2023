use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 15))]
pub fn solve(input: &str) -> String {
    let seq = input.lines().next().unwrap().split(',').collect_vec();

    let part_a = seq.iter().map(|part| hash(part) as u32).sum::<u32>();
    let part_b = part_b(&seq);

    format!("{}/{}", part_a, part_b)
}

fn set_box(hash_boxes: &mut [Vec<(String, u32)>], id: &str, val: u32) {
    let hash_box = hash_boxes.get_mut(hash(id) as usize).unwrap();
    if let Some(index) = hash_box.iter_mut().position(|v| v.0 == id) {
        hash_box[index].1 = val;
    } else {
        hash_box.push((id.into(), val));
    }
}

fn dec_box(hash_boxes: &mut [Vec<(String, u32)>], id: &str) {
    let hash_box = hash_boxes.get_mut(hash(id) as usize).unwrap();
    if let Some(index) = hash_box.iter_mut().position(|val| val.0 == id) {
        hash_box.remove(index);
    }
}

fn part_b(seq: &[&str]) -> usize {
    let mut hash_boxes = (0..256).map(|_| Vec::<(String, u32)>::default()).collect_vec();
    for part in seq {
        if part.ends_with('-') {
            let id = &part[0..part.len() - 1];
            dec_box(&mut hash_boxes, id);
            tracing::debug!("- {}", id);
        } else {
            let mut split = part.split('=');
            let id = split.next().unwrap();
            let num = split.next().unwrap().parse::<u32>().unwrap();
            set_box(&mut hash_boxes, id, num);
            tracing::debug!("= {} {}", id, num);
        }
    }
    hash_boxes.iter().enumerate().map(|(index, hash_box)| {
        (1 + index) * hash_box.iter().enumerate().map(|(entry_index, entry)| {
            (1 + entry_index) * (entry.1 as usize)
        }).sum::<usize>()
    }).sum::<usize>()
}

fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0, |agg, ch| ((agg + (ch as u32)) * 17) % 256) as u8
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        "1320/145".into(),
    )
}
