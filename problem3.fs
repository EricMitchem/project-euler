module EricMitchem.ProjectEuler.Problem3

open System

// Returns a BitArray with all primes set.
let sieveOfEratosthenes (n : int) : Collections.BitArray =
    let arr = new Collections.BitArray(n + 1, true)
    arr.Set(0, false)
    arr.Set(1, false)

    let rec mark p acc n =
        match () with
        | _ when p * p > n -> arr
        | _ ->
            match () with
            | _ when acc > n -> mark (p + 1) ((p + 1) * (p + 1)) n
            | _ -> arr.Set(acc, false)
                   mark p (acc + p) n

    mark 2 4 n

let problem() =
    let Num = 600851475143L
    let maxFactor = int (Math.Sqrt(float Num))
    let sieve = maxFactor |> sieveOfEratosthenes

    let rec lpf acc =
        match sieve.Get(acc) with
        | true ->
            match Num % (int64 acc) = 0L with
            | true -> acc
            | _ -> lpf (acc - 1)
        | _ -> lpf (acc - 1)

    lpf maxFactor