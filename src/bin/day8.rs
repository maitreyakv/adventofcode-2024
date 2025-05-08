use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    Map::from_str(input).num_antinodes(Some(vec![1]))
}

fn part_2(input: &str) -> usize {
    Map::from_str(input).num_antinodes(None)
}

#[derive(Debug)]
struct Map {
    antennas: HashMap<char, HashSet<Position>>,
    bounds: Position,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(isize, isize);

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<isize> for Position {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Map {
    fn from_str(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let bounds = Position(
            (lines.len() - 1).try_into().unwrap(),
            (lines[0].len() - 1).try_into().unwrap(),
        );
        let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();
        for (i, line) in lines.into_iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_default()
                        .insert(Position(i.try_into().unwrap(), j.try_into().unwrap()));
                }
            }
        }
        Self { antennas, bounds }
    }

    fn num_antinodes(&self, factors: Option<Vec<isize>>) -> usize {
        let factors = factors.unwrap_or((0..=max(self.bounds.0, self.bounds.1)).collect());
        let mut antinodes = HashSet::new();
        for (p1, p2) in self.antenna_pairs() {
            for factor in &factors {
                let antinode_1 = *p2 + (*p2 - *p1) * *factor;
                if self.is_in_bounds(&antinode_1) {
                    antinodes.insert(antinode_1);
                }
                let antinode_2 = *p1 + (*p1 - *p2) * *factor;
                if self.is_in_bounds(&antinode_2) {
                    antinodes.insert(antinode_2);
                }
            }
        }
        antinodes.len()
    }

    fn antenna_pairs(&self) -> impl Iterator<Item = (&Position, &Position)> {
        self.antennas.keys().flat_map(|c| {
            let antennas = self.antennas.get(c).unwrap().iter().collect::<Vec<_>>();
            (0..antennas.len()).flat_map(move |idx1| {
                ((idx1 + 1)..antennas.len()).map({
                    let antennas = antennas.clone();
                    move |idx2| (antennas[idx1], antennas[idx2])
                })
            })
        })
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        (0..=self.bounds.0).contains(&position.0) && (0..=self.bounds.1).contains(&position.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 381)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1184)
    }
}

#[allow(clippy::items_after_test_module)]
const INPUT: &str = r#"......................D....B...h..................
..............................h...................
.............D...3.....X..................9.......
...........C........X....2.hB......v........b.....
....................................O.............
......u.....3.........p...........................
....u......................v....6.................
......................y..D.....Ov.2..............b
.....u..........X...........o........y............
.........................y...B.f...........s......
.7....................C.2.....Bsyp..........t...q.
.u.7...........X............................Oe..t.
...........V........3......6v.s........o....h....t
..E........L.................6..........o......9..
........E......m.2.P.......O...9...8....b.........
..m..........3.......p..........M8................
..1.....................K.p....................b.e
5...............L...........s.6..........S.M......
....5..1.......E.........k.f.........M............
.E..Y..V......l.......T...D.......9....Q..........
..............................M...................
.....5....P................m...x..q......F......e.
................f...c......................x..F...
..V.C...........7.......a....o....8.........F.....
.......4....L.a..g..P.....8......Q....7d..........
...1......4..a............k......t...d............
..........V..........L....m........K....Q........S
..................1....k.....T....................
..........l......a...............F................
...........P...4.......l......x...................
.............c....g........T......................
.....g............c...Q.......................S...
...............l..................A.d.T.U.........
..........................f...0.............d.....
..........G..................A............e.S...x.
.........Y.......q........g....K..................
.....................q.H4...0.................j...
....................HA..............J.............
..Y..........................0...J.......j........
.......................G.JA...................U...
.......5..........................................
...........c..............G.........K.............
...............................G..................
...........................0.j....................
............................H.......k..........U..
.........................H........................
...................................Y....J.........
..................................j...............
..................................................
.................................................."#;
