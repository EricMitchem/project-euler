module EricMitchem.ProjectEuler.Problem2

open System.Collections.Generic

let fib =
    let cache = new Dictionary<int64, int64>()

    let rec impl n =
        if cache.ContainsKey(n) then
            cache.[n]
        elif n <= 2L then
            1L
        else
            let res = impl (n - 1L) + impl (n - 2L)
            cache.Add(n, res)
            res

    fun x -> impl x

let problem() =
    let rec impl acc sum =
        match fib acc with
        | num when num > 4000000L -> sum
        | num when num % 2L = 0L -> impl (acc + 1L) (sum + num)
        | _ -> impl (acc + 1L) sum

    impl 1L 0L