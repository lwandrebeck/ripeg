// Copyright (C) 2022 Laurent Wandrebeck
//
// This file is part of ripeg.
//
// ripeg is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ripeg is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ripeg.  If not, see <http://www.gnu.org/licenses/>.

use criterion::{criterion_group, criterion_main, Criterion};
use ripeg::charset::*;

fn normalset_add() {
    let mut _s = NormalSet::new(&[67u8, 68, 69]); // C, D, E ASCII decimal value
    let s2 = NormalSet::new(&[0u8, 68, 70]);
    // calling add several times to negate time used by ::new call.
    _s.add(s2);
}

fn normalset_complement() {
    let s = NormalSet::new(&[67u8, 68, 69]);
    // calling complement several times to negate time used by ::new call.
    s.complement();
    s.complement();
    s.complement();
    s.complement();
    s.complement();
    s.complement();
    s.complement();
    s.complement();
}

fn normalset_has() {
    let charset = [67u8, 68, 69]; // C, D, E ASCII decimal value
    let s = NormalSet::new(&charset);
    let mut _result = false;
    // calling has several times to negate time used by ::new call.
    _result = s.has(64);
    _result = s.has(65);
    _result = s.has(66);
    _result = s.has(67);
    _result = s.has(68);
    _result = s.has(69);
    _result = s.has(70);
    _result = s.has(71);
    _result = s.has(72);
    _result = s.has(250);
}

fn normalset_is_small() {
    let charset = [67u8, 68, 69]; // C, D, E ASCII decimal value
    let s = NormalSet::new(&charset);
    let mut _result = false;
    // calling is_small several times to negate time used by ::new call.
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
    _result = s.is_small();
}

fn normalset_new() {
    // worst case, setting every bits
    let _s = NormalSet::new(&[
        0u8, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
        131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
        149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
        167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
        185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
        203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
        221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
        239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ]);
}

fn normalset_range() {
    // worst case, 256 loops.
    let mut _s = NormalSet::range(0, 255);
}

fn normalset_size() {
    let s = NormalSet::new(&[0u8, 1, 2, 3]);
    // calling size several times to negate time used by ::new call.
    let mut _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
}

fn normalset_smallset() {
    let s = NormalSet::new(&[0u8, 1, 2, 3]);
    // calling smallset several times to negate time used by ::new call.
    let mut _s2 = s.smallset();
    let mut _s2 = s.smallset();
    let mut _s2 = s.smallset();
    let mut _s2 = s.smallset();
    let mut _s2 = s.smallset();
    let mut _s2 = s.smallset();
    let mut _s2 = s.smallset();
    let mut _s2 = s.smallset();
}

fn normalset_sub() {
    let s = NormalSet::new(&[0u8, 1, 2, 3]);
    let s2 = NormalSet::range(3, 5);
    // calling sub several times to negate time used by ::new call.
    s.sub(s2);
}

fn smallset_has() {
    let charset = [67u8, 68, 69]; // C, D, E ASCII decimal value
    let s = SmallSet::new(&charset);
    let mut _result = false;
    // calling has several times to negate time used by ::new call.
    _result = s.has(64);
    _result = s.has(65);
    _result = s.has(66);
    _result = s.has(67);
    _result = s.has(68);
    _result = s.has(69);
    _result = s.has(70);
    _result = s.has(71);
    _result = s.has(72);
    _result = s.has(250);
}

fn smallset_new() {
    // worst case, setting every bits
    let _s = SmallSet::new(&[
        0u8, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    ]);
}

fn smallset_size() {
    let s = SmallSet::new(&[0u8, 1, 2, 3]);
    // calling size several times to negate time used by ::new call.
    let mut _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
    _size = s.size();
}

fn normalset_add_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_add", |b| b.iter(normalset_add));
}

fn normalset_complement_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_complement", |b| {
        b.iter(|| normalset_complement())
    });
}

fn normalset_has_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_has", |b| b.iter(|| normalset_has()));
}

fn normalset_is_small_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_is_small", |b| b.iter(|| normalset_is_small()));
}

fn normalset_new_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_new", |b| b.iter(|| normalset_new()));
}

fn normalset_range_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_range", |b| b.iter(|| normalset_range()));
}

fn normalset_size_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_size", |b| b.iter(|| normalset_size()));
}

fn normalset_smallset_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_smallset", |b| b.iter(|| normalset_smallset()));
}

fn normalset_sub_benchmark(c: &mut Criterion) {
    c.bench_function("normalset_sub", |b| b.iter(|| normalset_sub()));
}

fn smallset_has_benchmark(c: &mut Criterion) {
    c.bench_function("smallset_has", |b| b.iter(|| smallset_has()));
}

fn smallset_new_benchmark(c: &mut Criterion) {
    c.bench_function("smallset_new", |b| b.iter(|| smallset_new()));
}

fn smallset_size_benchmark(c: &mut Criterion) {
    c.bench_function("smallset_size", |b| b.iter(|| smallset_size()));
}

criterion_group!(
    benches,
    normalset_add_benchmark,
    normalset_complement_benchmark,
    normalset_has_benchmark,
    normalset_is_small_benchmark,
    normalset_new_benchmark,
    normalset_range_benchmark,
    normalset_size_benchmark,
    normalset_smallset_benchmark,
    normalset_sub_benchmark,
    smallset_has_benchmark,
    smallset_new_benchmark,
    smallset_size_benchmark
);
criterion_main!(benches);
