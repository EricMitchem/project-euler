module EricMitchem.ProjectEuler.Problem5

let lcm (x : int) (y : int) : int =
    let rec impl acc f inc =
        match () with
        | _ when acc % f = 0 -> acc
        | _ -> impl (acc + inc) f inc

    if x > y then
        impl x y x
    else
        impl y x y

let lcmThrough (n : int) : int =
    let rec impl acc res =
        match acc with
        | 1 -> res
        | _ -> impl (acc - 1) (lcm res acc)

    impl n 1

let problem() =
    lcmThrough 20