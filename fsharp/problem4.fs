module EricMitchem.ProjectEuler.Problem4

open System

let digitCount (n : int) : int =
    let rec impl acc =
        match int (Math.Pow(10.0, float (acc - 1))) with
        | p when n >= p -> acc
        | _ -> impl (acc - 1)

    impl 10

let digit (d : int) (n : int) : int =
    let rec impl acc n =
        match int (Math.Pow(10.0, float (acc - 1))) with
        | p when acc = d ->  n / p
        | p -> impl (acc - 1) (n - (n / p * p))

    impl 10 n

let isPalindromic (n : int) : bool =
    let rec impl acc1 acc2 =
        match () with
        | _ when acc1 >= acc2 -> true
        | _ -> (n |> digit acc1) = (n |> digit acc2) && impl (acc1 + 1) (acc2 - 1)

    impl 1 (digitCount n)

let problem() =
    let rec impl acc1 acc2 max =
        let Max p m = if p |> isPalindromic then Math.Max(p, m) else m

        match acc1 * acc2 with
        | p when acc1 = 100 -> Max p max
        | p when acc2 = 100 -> impl (acc1 - 1) (acc1 - 1) (Max p max)
        | p -> impl acc1 (acc2 - 1) (Max p max)

    impl 999 999 0