module EricMitchem.ProjectEuler.Problem10

open System

// Returns a BitArray with all primes set.
let sieveOfEratosthenes (n : int) : Collections.BitArray =
    let arr = new Collections.BitArray(n + 1, true)
    arr.Set(0, false)
    arr.Set(1, false)

    let succ p =
        let rec impl acc =
            match arr.Get(acc) with
            | true -> acc
            | _ -> impl (acc + 1)

        impl (p + 1)

    let rec mark p acc n =
        match () with
        | _ when p * p > n -> arr
        | _ ->
            match () with
            | _ when acc > n -> let sp = succ p in mark sp (sp * sp) n
            | _ -> arr.Set(acc, false)
                   mark p (acc + p) n

    mark 2 4 n

let problem() =
    let sieve = sieveOfEratosthenes 2000000

    let rec impl idx sum =
        match () with
        | _ when idx > 2000000 -> sum
        | _ ->
            match sieve.Get(idx) with
            | true -> impl (idx + 2) (sum + int64 idx)
            | _ -> impl (idx + 2) sum

    impl 3 2L