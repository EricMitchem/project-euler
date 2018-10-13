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

struct FibNumbers {
    curr: u32,
    prev: u32
}

impl FibNumbers {
    fn new() -> FibNumbers {
        FibNumbers {
            curr: 1,
            prev: 0
        }
    }
}

impl Iterator for FibNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let succ = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = succ;
        Some(succ)
    }
}

#[test]
#[ignore]
fn problem2() {
    let res: u32 = FibNumbers::new()
        .take_while(|&n| n <= 4_000_000)
        .filter(|&n| n % 2 == 0)
        .sum();

    assert_eq!(res, 4_613_732);
}