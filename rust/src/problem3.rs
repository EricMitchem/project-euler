/******************************************************************************
* MIT License
*
* Copyright (c) 2018 Eric Mitchem
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
******************************************************************************/

struct PrimeNumbers {
    primes: Vec<u32>,
    curr: u32
}

impl PrimeNumbers {
    fn new() -> PrimeNumbers {
        PrimeNumbers {
            primes: Vec::new(),
            curr: 0
        }
    }

    fn has_prime_factorization(&self, n: u32) -> bool {
        self.primes.iter().any(|p| n % p == 0)
    }
}

impl Iterator for PrimeNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.primes.is_empty() {
            self.primes.push(2);
            self.curr = 3;
            return Some(2)
        }

        let mut num = self.curr;

        loop {
            if self.has_prime_factorization(num) {
                num += 2;
            }

            else {
                self.primes.push(num);
                self.curr = num + 2;
                return Some(num)
            }
        }
    }
}

struct PrimeFactorization {
    primes: PrimeNumbers,
    curr_num: u64,
    curr_prime: u64
}

impl PrimeFactorization {
    fn new(num: u64) -> PrimeFactorization {
        let mut primes = PrimeNumbers::new();
        let curr_num = num;
        let curr_prime = primes.next().unwrap() as u64;

        PrimeFactorization {
            primes,
            curr_num,
            curr_prime
        }
    }
}

impl Iterator for PrimeFactorization {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_num == 1 {
            return None
        }

        loop {
            if self.curr_num % self.curr_prime == 0 {
                self.curr_num /= self.curr_prime;
                return Some(self.curr_prime)
            }

            else {
                self.curr_prime = self.primes.next().unwrap() as u64;
            }
        }
    }
}

#[test]
#[ignore]
fn problem3() {
    let num = 600_851_475_143_u64;

    let res = PrimeFactorization::new(num).max().unwrap();

    assert_eq!(res, 6857);
}