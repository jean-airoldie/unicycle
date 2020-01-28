use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use std::slice::SliceIndex;

static INDEXES: [usize; 1000] = [
    77975, 92324, 20136, 15648, 52494, 60691, 80040, 40303, 49247, 29647, 99498, 34012, 42435,
    45566, 59426, 78446, 19651, 45430, 87048, 83403, 15900, 82085, 63020, 48584, 40625, 58799,
    88683, 63218, 10869, 76588, 74480, 32141, 61242, 7364, 20195, 96056, 4726, 56939, 71157, 34914,
    88484, 70246, 39182, 35076, 92731, 50074, 11594, 35764, 24791, 17864, 5295, 3789, 22812, 60481,
    12370, 92098, 97350, 65061, 25272, 19569, 96159, 45058, 28864, 33538, 67, 40343, 12470, 21964,
    950, 98386, 68325, 62955, 1069, 33449, 12221, 31260, 43833, 69251, 64025, 43984, 99556, 68840,
    79661, 2275, 66640, 1912, 12992, 40660, 54925, 68334, 83997, 45672, 52417, 34844, 68173, 96451,
    21551, 74891, 44264, 27988, 42520, 75406, 89084, 43632, 93275, 3843, 61656, 13319, 65433,
    73073, 69726, 89001, 58631, 65723, 61288, 45692, 63143, 88019, 47929, 62721, 4098, 55915,
    81801, 69181, 47107, 43663, 73752, 27447, 15535, 12763, 10042, 60851, 47199, 55611, 99097,
    41666, 62217, 32785, 64375, 29647, 43971, 62042, 64854, 23657, 41379, 70447, 5573, 35174,
    97803, 14436, 56930, 6080, 82320, 58489, 70950, 77212, 31301, 23026, 28173, 70861, 85305,
    70200, 51170, 34624, 63596, 13738, 75137, 67243, 21616, 20514, 42060, 65376, 95267, 68461,
    58403, 24997, 43798, 91622, 16767, 83243, 80604, 44388, 51008, 9986, 30997, 64123, 24437,
    86727, 47380, 46955, 65763, 87766, 95388, 64299, 27800, 51655, 94095, 37052, 91676, 63776,
    38640, 66078, 18864, 60788, 75136, 39788, 91788, 15358, 45258, 82861, 80487, 11141, 40362,
    83646, 45836, 29198, 44570, 73800, 38070, 7584, 79149, 48386, 76740, 72286, 17236, 85501, 7014,
    48217, 73276, 23900, 18280, 75300, 32319, 58851, 21795, 81166, 97101, 56285, 27526, 50037,
    23199, 30812, 71982, 53679, 49876, 59896, 67255, 23337, 71075, 76937, 22619, 86495, 93246,
    11060, 25696, 17370, 35030, 60619, 40545, 31223, 99364, 40919, 25859, 45257, 64365, 57027,
    43051, 2460, 22738, 76323, 24948, 99514, 23516, 22653, 73038, 76524, 43968, 81216, 28541,
    53591, 7185, 4844, 45790, 44633, 8729, 14253, 46306, 33364, 20701, 18893, 26543, 47184, 19509,
    87399, 97059, 66200, 94421, 69689, 61803, 8166, 11198, 42969, 41155, 3772, 14449, 37224, 34524,
    61355, 57726, 78652, 11611, 20426, 77549, 20835, 77492, 50691, 11777, 99913, 22006, 4615,
    60282, 40181, 70031, 47341, 15656, 90338, 95250, 98801, 26330, 78152, 54277, 93240, 46191,
    60782, 82943, 50540, 76152, 6657, 71179, 11179, 34690, 89099, 29588, 96536, 85086, 70859,
    84691, 61115, 58476, 81596, 83493, 95148, 12924, 71549, 70049, 39775, 51838, 27126, 76095,
    65491, 88105, 41218, 61012, 78115, 42957, 83869, 24216, 21974, 30549, 76410, 34277, 60197,
    81546, 66319, 14016, 88877, 34426, 97833, 3261, 1084, 78248, 65131, 43199, 83943, 24716, 13100,
    87413, 47913, 58537, 25563, 99638, 614, 30016, 5799, 43607, 96894, 51678, 2085, 26823, 87090,
    1021, 18921, 93763, 95919, 43928, 74724, 8262, 60451, 41100, 45272, 42773, 41899, 20757, 99068,
    2420, 44868, 93657, 1543, 85712, 69216, 38642, 87118, 88900, 42224, 44331, 41553, 9841, 81119,
    78029, 45716, 51485, 18892, 57136, 27213, 92631, 3439, 51495, 24684, 1650, 50406, 53777, 74213,
    34905, 50198, 27317, 32189, 84012, 849, 51930, 45733, 70621, 67870, 39525, 42862, 14574, 56848,
    25084, 54995, 67073, 81957, 53051, 11302, 4496, 45141, 83267, 31035, 38693, 61497, 32075,
    78496, 48371, 56664, 75386, 50843, 45322, 35194, 98707, 80202, 32322, 92325, 70583, 28939,
    30609, 63572, 13714, 15523, 74352, 25546, 73833, 8043, 70765, 86953, 97601, 99489, 21980,
    20504, 89712, 97576, 17710, 89982, 17291, 65229, 65817, 84187, 15081, 15895, 10603, 98392,
    19449, 81194, 62355, 21383, 75700, 72282, 25052, 46514, 54874, 26826, 38300, 69427, 55833,
    29833, 67266, 92561, 41694, 50969, 92998, 85567, 29769, 78211, 58734, 38034, 76410, 46031,
    77380, 99216, 48349, 84924, 37191, 4331, 73203, 12348, 7326, 16489, 46644, 96591, 29614, 81036,
    71323, 24482, 76162, 42577, 18215, 70609, 58211, 14030, 66350, 9135, 94416, 18335, 22873,
    23470, 88314, 90047, 44388, 94211, 25005, 50418, 69736, 92364, 95765, 42099, 17636, 15103,
    36612, 78542, 25150, 18579, 54990, 46973, 69681, 98010, 237, 18183, 57997, 62716, 52498, 7928,
    17782, 86209, 3864, 2581, 19037, 69538, 88352, 68431, 99743, 56897, 29684, 67075, 28245, 92203,
    50126, 66856, 13193, 39258, 50035, 90126, 53143, 8037, 27090, 80341, 14653, 34920, 66366,
    87319, 86942, 17113, 70294, 14572, 52179, 27803, 88332, 72342, 67491, 8373, 22310, 5818, 87950,
    27763, 77552, 68372, 47739, 34020, 726, 7652, 15701, 32062, 56247, 58369, 56013, 66785, 41615,
    53431, 85575, 29844, 10043, 8874, 59471, 73365, 24257, 16291, 24685, 86089, 20321, 54661, 7099,
    61399, 22344, 64914, 56654, 56324, 84636, 97518, 50882, 23465, 51717, 77875, 349, 63730, 57631,
    12703, 13150, 264, 19360, 34639, 16432, 67019, 82366, 37752, 2761, 38320, 77160, 77587, 61242,
    40751, 77733, 22476, 23636, 50186, 81248, 19162, 80468, 33948, 20390, 47320, 35913, 36186,
    18356, 26626, 30382, 58921, 8182, 98799, 95858, 33381, 60989, 94130, 54004, 24725, 59495,
    51909, 24693, 71321, 53230, 76691, 41108, 16383, 64042, 75942, 20775, 30152, 84487, 96879,
    80313, 80750, 57057, 64867, 80245, 76669, 56117, 75765, 20428, 98722, 56968, 2223, 43553,
    57997, 72948, 42221, 9705, 70366, 11262, 87049, 56664, 86459, 34283, 4212, 46167, 40201, 33118,
    38305, 12834, 78629, 45993, 295, 40989, 83113, 58291, 87492, 60850, 47517, 61622, 39562, 86370,
    83006, 52830, 73046, 64970, 17436, 60947, 32525, 30283, 32796, 60860, 98837, 37116, 15835,
    39308, 58691, 41152, 58666, 32969, 34428, 96497, 83880, 56901, 69403, 69834, 54311, 94642,
    22033, 33355, 79556, 84547, 5390, 47202, 2298, 36087, 42446, 85884, 66097, 91176, 37654, 59480,
    57414, 40226, 97649, 5917, 95539, 1143, 55877, 19563, 16384, 68775, 24175, 72247, 82147, 80655,
    21897, 29163, 18976, 34941, 11599, 75810, 68742, 43892, 98103, 22352, 97738, 17241, 20876,
    2490, 82871, 92572, 15405, 5028, 15333, 8272, 18168, 2141, 66282, 6618, 7254, 69241, 5830,
    23099, 4629, 31885, 57222, 4050, 22782, 61098, 52981, 98443, 24570, 63090, 45970, 36174, 77662,
    23747, 78109, 88798, 91527, 54896, 8192, 75359, 56560, 9512, 42707, 68837, 61748, 83251, 44199,
    76568, 64105, 60177, 83434, 87226, 16016, 47537, 68005, 63090, 83946, 57270, 84568, 19283,
    44967, 24776, 72163, 13401, 69108, 47210, 29816, 77269, 96662, 71437, 63010, 92798, 76189,
    17096, 89558, 24669, 50163, 3686, 48179, 79090, 96713, 87974, 76876, 96420, 5989, 34500, 32263,
    66083, 19135, 22463, 70064, 8600, 3875, 83775, 58596, 27940, 41760, 47532, 11528, 52544, 75513,
    52771, 86247, 6013, 10968, 44391, 88093, 65891, 75128, 48060, 61469, 76495, 98403, 29837,
    68880, 69285, 57011, 81212, 25673, 59621, 63068, 9818, 33772, 94129, 61353, 23087, 42777,
    20712, 60915, 13427, 8191, 22149, 28442, 50076, 55472, 55286, 83036, 76325, 43797, 19450,
    43357, 38040, 1655, 79686, 52417, 85941, 12931, 7768, 28973, 70638, 59073, 30705, 17255, 85731,
    98797, 8477, 48827, 84903, 31137, 80735, 55882, 35021, 97786, 94253, 39091, 94497, 50434,
    27625, 13628, 35616, 44849, 44396, 90444,
];

pub fn bitset_benchmark(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("bitset iter");

        for n in [1, 10, 50, 100, 250, 500, 750, 1000].iter() {
            group.bench_with_input(BenchmarkId::new("unicycle", n), n, |b, n| {
                let set = unicycle(..*n);
                b.iter(|| set.iter().collect::<Vec<_>>());
            });

            group.bench_with_input(BenchmarkId::new("hibitset", n), n, |b, n| {
                let set = hibitset(..*n);
                b.iter(|| (&set).into_iter().collect::<Vec<_>>());
            });
        }
    }

    {
        let mut group = c.benchmark_group("bitset drain");

        for n in [1, 10, 50, 100, 250, 500, 750, 1000].iter() {
            group.bench_with_input(BenchmarkId::new("unicycle", n), n, |b, n| {
                let set = unicycle(..*n);
                b.iter(|| set.clone().drain().collect::<Vec<_>>());
            });

            group.bench_with_input(BenchmarkId::new("hibitset", n), n, |b, n| {
                use hibitset::DrainableBitSet as _;
                let set = hibitset(..*n);
                b.iter(|| (&mut set.clone()).drain().collect::<Vec<_>>());
            });
        }
    }

    {
        let mut group = c.benchmark_group("bitset clone all");

        group.bench_function("unicycle", |b| {
            let set = unicycle(..);
            b.iter(|| set.clone());
        });

        group.bench_function("hibitset", |b| {
            let set = hibitset(..);
            b.iter(|| set.clone());
        });
    }

    fn hibitset<I: SliceIndex<[usize], Output = [usize]>>(index: I) -> hibitset::BitSet {
        let mut set = hibitset::BitSet::with_capacity(1_000_000);

        for i in &INDEXES[index] {
            set.add(*i as u32);
        }

        set
    }

    fn unicycle<I: SliceIndex<[usize], Output = [usize]>>(index: I) -> unicycle::BitSet {
        let mut set = unicycle::BitSet::with_capacity(1_000_000);

        for i in &INDEXES[index] {
            set.set(*i);
        }

        set
    }
}

criterion_group!(bitset, bitset_benchmark);
criterion_main!(bitset);
