use super::*;

#[test]
fn test_push() {
    let mut tester = RingBuffer::with_capacity(15);
    let cap = tester.cap();

    for i in 0..cap - 1 {
        tester.push(i)
    }

    assert!(Iterator::eq(tester.into_iter(), (0..cap - 1).into_iter()));
}

#[test]
fn test_pop() {
    let mut tester = RingBuffer::<usize>::with_capacity(15);
    let cap = tester.cap();

    for _ in 0..cap - 1 {
        assert_eq!(tester.pop(), None)
    }

    assert!(Iterator::eq(tester.into_iter(), vec![].into_iter()));
}

#[test]
fn test_push_pop() {
    let mut tester = RingBuffer::with_capacity(15);
    let cap = tester.cap();

    for i in 0..cap - 1 {
        if i % 3 <= 1 {
            tester.push(i)
        } else {
            assert_ne!(tester.pop(), None)
        }
    }

    assert!(Iterator::eq(
        tester.into_iter(),
        vec![6, 7, 9, 10, 12, 13].into_iter()
    ));
}

#[test]
fn test_resize() {
    let mut tester = RingBuffer::with_capacity(15);

    for i in 0..50 {
        tester.push(i)
    }

    assert!(Iterator::eq(tester.into_iter(), (0..50).into_iter()));
}

#[test]
fn test_resize_2() {
    let mut tester = RingBuffer::with_capacity(15);

    for i in 0..10 {
        tester.push(i)
    }
    for _ in 0..5 {
        tester.pop();
    }
    for i in 10..30 {
        tester.push(i)
    }

    assert!(Iterator::eq(tester.into_iter(), (5..30).into_iter()));
}

#[test]
fn test_resize_3() {
    let mut tester = RingBuffer::with_capacity(15);

    for i in 0..10 {
        tester.push(i)
    }
    for _ in 0..8 {
        tester.pop();
    }
    for i in 10..30 {
        tester.push(i)
    }

    assert!(Iterator::eq(tester.into_iter(), (8..30).into_iter()));
}

#[test]
fn test_zero_capacity() {
    let mut tester = RingBuffer::with_capacity(0);

    for i in 0..20 {
        tester.push(i)
    }

    assert!(Iterator::eq(tester.into_iter(), (0..20).into_iter()));
}

// extern crate test;
// use std::collections::VecDeque;
// use test::Bencher;
//
// #[bench]
// fn bench_push_pop(b: &mut Bencher) {
//     b.iter(|| {
//         let mut tester = RingBuffer::with_capacity(0);
//
//         for i in 0..100 {
//             tester.push(i)
//         }
//         for _ in 0..50 {
//             tester.pop();
//         }
//         for i in 100..1000 {
//             tester.push(i)
//         }
//
//         assert!(Iterator::eq(tester.into_iter(), (50..1000).into_iter()));
//     })
// }
//
// #[bench]
// fn bench_push_pop_vec_deque(b: &mut Bencher) {
//     b.iter(|| {
//         let mut tester = VecDeque::with_capacity(0);
//
//         for i in 0..100 {
//             tester.push_back(i)
//         }
//         for _ in 0..50 {
//             tester.pop_front();
//         }
//         for i in 100..1000 {
//             tester.push_back(i)
//         }
//
//         assert!(Iterator::eq(tester.into_iter(), (50..1000).into_iter()));
//     })
// }

// test tests::bench_push_pop           ... bench:      17,112 ns/iter (+/- 304)
// test tests::bench_push_pop_vec_deque ... bench:       3,143 ns/iter (+/- 118)
// Разница на порядок, не так уж плохо, но определенно есть места для оптимизации
