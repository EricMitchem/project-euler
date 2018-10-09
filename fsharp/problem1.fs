module EricMitchem.ProjectEuler.Problem1

let problem() =
    let rec impl acc sum =
        match acc with
        | 1000 -> sum
        | num when acc % 3 = 0 || acc % 5 = 0 -> impl (acc + 1) (sum + num)
        | _ -> impl (acc + 1) sum

    impl 3 0