
fn main() {
    let input = vec![(0,1),(5,9),(9,8),(13,9),(7,11),(0,4),(9,10),(10,6),(4,5),(2,3),(13,14),(4,8),(8,12),(1,5),(5,6),(14,15),(10,11),(2,6),(12,13),(1,2),(10,14),(15,11),(6,7),(3,7)];
    //let input = vec![(0,1),(7,8),(1,2),(6,7),(0,3),(5,8),(3,4),(1,4),(4,5),(2,5),(4,7),(3,6)];
    //let input = vec![(0,1),(1,2),(2,5),(5,4),(4,7),(7,8),(8,5),(1,4),(6,7),(3,6),(3,0),(3,4)];
    //let input = vec![(0,1),(7,8),(1,2),(6,7),(0,3),(8,5),(3,4),(4,1),(4,5),(2,5),(3,6),(7,4)];
    //let input = vec![(5,9),(6,10),(9,13),(7,11),(4,5),(5,6),(6,7),(8,12),(1,5),(2,6),(8,9),(10,14),(10,11),(4,8),(12,13),(9,10),(1,2),(11,15),(13,14),(14,15),(0,4),(0,1),(3,7),(2,3)];
    let result = dots_and_boxes(input);
    println!("{:?}", result);
}

pub fn dots_and_boxes(r: Vec<(usize,usize)>) -> (usize,usize) {
    let mut scores = [0, 0];

    let boxy = |n: usize, i: usize| -> Vec<(usize, usize)> { 
        let x = i + i/(n - 1);
        vec![
            (x, x + 1),         // (0, 1)
            (x + 1, x + 1 + n), // (1, 4)
            (x + 1 + n, x + n), // (4, 3)
            (x + n, x),         // (3, 0)
        ]
    };

    let nodes = r.clone().into_iter().fold(0, |acc, f| {
        if f.0 > acc && f.0 > f.1 {
            return f.0
        } else if f.1 > acc && f.1 > f.0 {
            return f.1
        }
        acc
    }) + 1;
    let length_of_grid = (nodes as f64).sqrt() as usize;

    let boxes = usize::pow(length_of_grid - 1, 2);

    let mut results: Vec<Vec<(usize, usize)>> = vec![0; boxes].into_iter().enumerate().map(|(index, _)| {
        boxy(length_of_grid, index)
    }).collect();


    let mut turn = 0;
    for (x, y) in r {
        for i in 0..results.len() {
            results[i] = results[i].clone().into_iter().filter(|(a, b)| {
                !((a == &x && b == &y) || (a == &y && b == &x))
            }).collect();
        }

        let stay = results.clone().into_iter().fold(false, |acc, b| {
            if b.len() == 0 {
                scores[turn] += 1;
                return true;
            }
            acc
        });
        if !stay {
            turn = (turn + 1) % 2;
        }

        results = results.into_iter().filter(|b| {
            b.len() > 0
        }).collect();
    }
    (scores[0], scores[1])
}

