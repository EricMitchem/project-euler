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

struct Numbers {
    curr: u32
}

impl Numbers {
    fn new() -> Numbers {
        Numbers {curr: 3}
    }
}

impl Iterator for Numbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        for num in self.curr .. 1000 {
            if num % 3 == 0 || num % 5 == 0 {
                self.curr = num + 1;
                return Some(num)
            }
        }

        None
    }
}

#[test]
#[ignore]
fn problem1() {
    let numbers = Numbers::new();

    let sum: u32 = numbers.sum();

    assert_eq!(sum, 233_168);
}