module EricMitchem.ProjectEuler.Problem9

let solvebc (a : int) : int * int =
    let r = 1000 - a
    let r2 = r / 2

    if r % 2 = 0 then
        r2 - 1, r2 + 1
    else
        r2, r2 + 1

let problem() =
    let rec impl a b c =
        match () with
        | _ when a * a + b * b = c * c -> a * b * c
        | _ when c = (1000 - a - (a + 1)) -> let nb, nc = solvebc (a + 1) in impl (a + 1) nb nc
        | _ -> impl a (b - 1) (c + 1)

    impl 1 499 500