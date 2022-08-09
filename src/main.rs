// caesarcipher-rs  --  Rust implementation of the Caesar cipher.
// Copyright (C) 2022  Brynley Llewellyn-Roux
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    println!(
        "{}",
        std::env::args()
            .nth(1)
            .unwrap()
            .chars()
            .map(|ch| {
                if LETTERS.contains(ch) {
                    let mut index = LETTERS.find(ch).unwrap();
                    index += std::env::args().nth(2).unwrap().parse::<usize>().unwrap();
                    LETTERS.chars().nth(index % LETTERS.len()).unwrap()
                } else {
                    ch
                }
            })
            .collect::<String>()
    );
}
