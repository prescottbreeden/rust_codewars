
fn main() {
    let input = vec![ (0,1) , (7,8) , (1,2) , (6,7) , (0,3) , (5,8) , (3,4) , (1,4) , (4,5) , (2,5) , (4,7) , (3,6) ];
    let result = dots_and_boxes(input);
    println!("{:?}", result);
}

pub fn dots_and_boxes(r: Vec<(usize,usize)>) -> (usize,usize) {
    let mut player1 = (1, 0);
    let mut player2 = (2, 0);
    let mut current_player = player1;

    let boxy = |n: usize, i: usize| -> Vec<(usize, usize)> { 
        let x = i + (i + 1)/n % n;
        vec![
            (x, x + 1),         // (0, 1)
            (x + 1, x + 1 + n), // (1, 4)
            (x + 1 + n, x + n), // (4, 3)
            (x + n, x),         // (3, 0)
        ]
    };

    let n = ((r.len() + 1) as f64).sqrt() as usize;
    let boxes = usize::pow(n - 1, 2);

    let mut results: Vec<Vec<(usize, usize)>> = vec![0; boxes].into_iter().enumerate().map(|(index, _)| {
        boxy(n, index)
    }).collect();

    for (x, y) in r {
        for i in 0..results.len() {
            results[i] = results[i].clone().into_iter().filter(|(a, b)| {
                !((a == &x && b == &y) || (a == &y && b == &x))
            }).collect();
        }
        println!("{:?}", results);
        let change_player = results.into_iter().fold(true, |acc, curr| {
            if curr.len() == 0 {
                current_player.1 += 1;
                return false
            }
            acc
        });
        if change_player {
            if current_player.0 == 1 {
                current_player = player2;
            } else {
                current_player = player1;
            }
        }
    }

    (0, 1)

    /*
        currentPlayer = player1

        // player tuple = (pos, score)
        player1 = (1, 0)
        player2 = (2, 0)

        boxes = [
                [(4, 3), (0, 3)]
                [(1, 2)]
            ]

            1: [(0, 1), (1, 4), (4, 3), (0, 3)]
            1: [(Box - 1, Box), (Box, Box + len), (Box + len, len), (Box - 1, len)]

            2: [(1, 2), (2, 5), (5, 4), (4, 1)]
            2: [(Box - 1, Box), (Box - 1 , Box + len), (Box + len, Box - 1 + len), (Box - 1 + len, Box - 1)]

            3: [(3, 4), (4, 7), (7, 6), (6, 3)]
            3: i => mult 1 -> add 1 -> add len -> mult 1 -> sub 1 -> mult 1 -> sub len

            3: i => (mult 1 -> add 1) -> (mult 1 -> add len) -> (mult 1 -> sub 1) -> (mult 1 -> sub len)


            let 3a: i => (i + floor(((i + 1)/len) % len)) -> (                                , add 1)
            let 3b: i => (i + floor(((i + 1)/len) % len)) -> (-> add 1 ->                     , add len)
            let 3c: i => (i + floor(((i + 1)/len) % len)) -> (-> add 1 -> -> add len          , sub 1)
            let 3d: i => (i + floor(((i + 1)/len) % len)) -> (-> add 1 -> -> add len -> sub 1 , sub len)




// --[ game logic ]--

    filter all boxes for (0, 1) && (1, 0)

    let changePlayer = fold(true, (acc, curr) -> 
        if curr.len == 0 
            then currentPlayer.1 += 1
            acc = false
        acc
    )

    filter len > 0

    if changePlayer then currentPlayer = the other player


// --[ sample input ]--

        [ (0,1)
        , (7,8)
        , (1,2)
        , (6,7)
        , (0,3)
        , (5,8)
        , (3,4)
        , (1,4)
        , (4,5)
        , (2,5)
        , (4,7)
        , (3,6)
        ] 

    */

}

