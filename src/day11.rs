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
    let parts = galaxies
        .iter()
        .combinations(2)
        .flat_map(|galaxy_pair| {
            [0, 1].map(|axis| {
                let coord = [0, 1].map(|index| galaxy_pair[index][axis]);
                let min = *coord.iter().min().unwrap();
                let max = *coord.iter().max().unwrap();
                [empties[axis][max] - empties[axis][min], max - min]
            })
        })
        .fold([0, 0], |agg, values| {
            [0, 1].map(|index| agg[index] + values[index])
        });
    mults
        .iter()
        .map(|mult| (mult - 1) * parts[0] + parts[1])
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
