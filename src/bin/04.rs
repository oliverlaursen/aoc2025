advent_of_code::solution!(4);

use grid::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Node {
    ToiletPaper = 1,
    Nothing = 0
}

impl TryFrom<char> for Node {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Node::ToiletPaper),
            '.' => Ok(Node::Nothing),
            _ => Err("Invalid node")
        }
    }
}

fn get_adjacent_toilet_papers(grid:&Grid<Node>, x: isize, y: isize) -> usize {
    grid.get(x+1, y).map(|node| *node as usize).unwrap_or(0) +
    grid.get(x-1, y).map(|node| *node as usize).unwrap_or(0) +
    grid.get(x, y+1).map(|node| *node as usize).unwrap_or(0) +
    grid.get(x, y-1).map(|node| *node as usize).unwrap_or(0) +
    grid.get(x+1, y+1).map(|node| *node as usize).unwrap_or(0) +
    grid.get(x+1, y-1).map(|node| *node as usize).unwrap_or(0) +
    grid.get(x-1, y-1).map(|node| *node as usize).unwrap_or(0) +
    grid.get(x-1, y+1).map(|node| *node as usize).unwrap_or(0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let cols = input.lines().next().unwrap().len();
    let grid:Vec<Node> = input.lines()
        .flat_map(|line| line
            .chars()
            .map(|c|c.try_into().unwrap())
        ).collect();
    let grid: Grid<Node> = Grid::from_vec(grid, cols);
    Some(grid.indexed_iter()
        .filter(|(_, e)| **e == Node::ToiletPaper)
        .filter(|((x,y), _)| get_adjacent_toilet_papers(&grid, *x as isize, *y as isize) < 4)
        .count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let cols = input.lines().next().unwrap().len();
    let grid:Vec<Node> = input.lines()
        .flat_map(|line| line
            .chars()
            .map(|c|c.try_into().unwrap())
        ).collect();
    let mut grid: Grid<Node> = Grid::from_vec(grid, cols);
    let mut removed = 0;
    let mut to_remove:Vec<(usize, usize)> = grid
        .indexed_iter()
        .filter(|(_, e)| **e == Node::ToiletPaper)
        .map(|((x,y),_)| (x,y))
        .filter(|(x,y)| get_adjacent_toilet_papers(&grid, *x as isize, *y as isize) < 4)
        .collect();
    while to_remove.len() > 0 {
        removed += to_remove.len();
        to_remove
            .iter()
            .for_each(|(x,y)| *grid.get_mut(*x,*y).unwrap() = Node::Nothing);
        to_remove = grid
            .indexed_iter()
            .filter(|(_, e)| **e == Node::ToiletPaper)
            .map(|((x,y),_)| (x,y))
            .filter(|(x,y)| get_adjacent_toilet_papers(&grid, *x as isize, *y as isize) < 4)
            .collect();

    }
    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
