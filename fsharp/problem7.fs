module EricMitchem.ProjectEuler.Problem7

let notFactorOf (n : int64) (f : int64) : bool =
    n % f <> 0L

let isPrime (n : int64) : bool =
    let max = int64 (System.Math.Sqrt(float n))

    let rec prime acc =
        match () with
        | _ when acc = max -> acc |> notFactorOf n
        | _ -> acc |> notFactorOf n && prime (acc + 1L)

    match () with
    | _ when n = 2L || n = 3L -> true
    | _ -> prime 2L

let problem() =
    let rec impl acc count =
        match () with
        | _ when count = 10001L -> acc - 2L
        | _ -> impl (acc + 2L) (if acc |> isPrime then count + 1L else count)

    impl 3L 1L