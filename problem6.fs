module EricMitchem.ProjectEuler.Problem6

let problem() =
    let rec impl acc sum sumSq =
        match () with
        | _ when acc > 100 -> (sum * sum) - sumSq
        | _ -> impl (acc + 1) (sum + acc) (sumSq + acc * acc)

    impl 1 0 0