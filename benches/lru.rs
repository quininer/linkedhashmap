use criterion::{ criterion_main, criterion_group, Criterion, black_box };


struct Bar([u64; 4]);

const CAP: usize = 1024;

fn bench_linkedhsahmap(c: &mut Criterion) {
    use linkedhashmap::LinkedHashMap;

    c.bench_function("linkedhashmap", |b| {
        let mut map = LinkedHashMap::with_capacity(CAP);
        let mut count = 0;

        b.iter(|| {
            count += 1;
            let bar = black_box(Bar([0x42; 4]));
            map.insert(count, bar);
            if map.len() >= CAP {
                map.pop_front();
            }
        });
    });
}

fn bench_hashlink(c: &mut Criterion) {
    use hashlink::LinkedHashMap;

    c.bench_function("hashlink", |b| {
        let mut map = LinkedHashMap::with_capacity(CAP);
        let mut count = 0;

        b.iter(|| {
            count += 1;
            let bar = black_box(Bar([0x42; 4]));
            map.insert(count, bar);
            if map.len() >= CAP {
                map.pop_front();
            }
        });
    });
}

fn bench_linked_hash_map(c: &mut Criterion) {
    use linked_hash_map::LinkedHashMap;

    c.bench_function("linked-hash-map", |b| {
        let mut map = LinkedHashMap::with_capacity(CAP);
        let mut count = 0;

        b.iter(|| {
            count += 1;
            let bar = black_box(Bar([0x42; 4]));
            map.insert(count, bar);
            if map.len() >= CAP {
                map.pop_front();
            }
        });
    });
}

criterion_group!(lru, bench_linkedhsahmap, bench_hashlink, bench_linked_hash_map);
criterion_main!(lru);
