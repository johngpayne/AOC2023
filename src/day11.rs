use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 11))]
pub fn solve(input: &str) -> String {
    let (galaxies, empties) = read_data(input);
    let dists = calc_dists(&galaxies, &empties, &[2, 1_000_000]);
    format!("{}/{}", dists[0], dists[1])
}

fn read_data(input: &str) -> (Vec<[usize; 2]>, [Vec<usize>; 2]) {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| [x, y])
        })
        .collect_vec();

    let empties = [0, 1].map(|axis| {
        let mut used = vec![];
        galaxies.iter().for_each(|galaxy| {
            if galaxy[axis] >= used.len() {
                used.resize(galaxy[axis] + 1, false);
            }
            used[galaxy[axis]] = true;
        });

        let mut num_empty = vec![0usize; used.len()];
        (0..used.len()).for_each(|index| {
            num_empty[index] = if index == 0 { 0 } else { num_empty[index - 1] }
                + if !used[index] { 1 } else { 0 };
        });
        num_empty
    });

    (galaxies, empties)
}

fn calc_dists(galaxies: &[[usize; 2]], empties: &[Vec<usize>; 2], mults: &[usize]) -> Vec<usize> {
    let min_max = |a: usize, b: usize| (a.min(b), a.max(b));
    let mut total_empties = 0;
    let mut total_dist = 0;
    galaxies.iter().enumerate().for_each(|(index, g0)| {
        galaxies.iter().skip(index + 1).for_each(|g1| {
            (0..2).for_each(|axis| {
                let (min, max) = min_max(g0[axis], g1[axis]);
                total_empties += empties[axis][max] - empties[axis][min];
                total_dist += max - min;
            })
        });
    });
    mults
        .iter()
        .map(|mult| (mult - 1) * total_empties + total_dist)
        .collect()
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    let (galaxies, empties) = read_data(
        "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....",
    );
    let dists = calc_dists(&galaxies, &empties, &[2, 100]);

    (format!("{}/{}", dists[0], dists[1]), "374/8410".into())
}
