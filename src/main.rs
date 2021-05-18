use bit_reverse::ParallelReverse;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

mod keys;

fn main() {
    // println!("Hello, world!");

    // println!("{}", format!("{:016b}", 0b101_u64));
    // println!("{}", format!("{:016b}", 0b101_u64.swap_bits()));
    // let hiu: Vec<_> = (0..10).into_par_iter().map(|x| x + 1).collect();
    // println!("{:?}", hiu);
    // return;
    let full = [
        37589154_u64,
        22319859,
        49601601,
        17211352,
        21322215,
        14970886,
        1649821,
        45204256,
        18110763,
        11247215,
        48752797,
        22991738,
        50403000,
        4416778,
        23378941,
        63942721,
        36627768,
        54604762,
        25150413,
        60094010,
        22829520,
        57720301,
        716646,
    ];

    // let raw_decode_dict = "{'00000': ' ', '00001': 'a', '00010': 'b', '00011': 'c', '00100': 'd', '00101': 'e', '00110': 'f', '00111': 'g', '01000': 'h', '01001': 'i', '01010': 'j', '01011': 'k', '01100': 'l', '01101': 'm', '01110': 'n', '01111': 'o', '10000': 'p', '10001': 'q', '10010': 'r', '10011': 's', '10100': 't', '10101': 'u', '10110': 'v', '10111': 'w', '11000': 'x', '11001': 'y', '11010': 'z', '11011': '0', '11100': '1', '11101': '2', '11110': '3', '11111': '4'}";
    let mut decode = HashMap::new();

    let raw_decode = std::fs::read_to_string("test2.txt").unwrap();
    raw_decode
        .split('\n')
        .map(|p| p.split(' ').collect::<Vec<_>>())
        .for_each(|p| {
            decode.insert(
                p[0],
                match p[1].len() {
                    0 => " ",
                    _ => p[1],
                },
            );
        });

    dbg!(&decode);

    let file = std::fs::read_to_string("vocab.csv").unwrap();

    let vocab: HashSet<_> = file.split('\n').collect();
    // dbg!(russian_nouns.len());
    search(&vocab, &decode);
    let n: u64 = 66732557;

    // decode_message(&keys::get_keys(), &full, &decode, n);
}

fn search(vocab: &HashSet<&str>, decode: &HashMap<&str, &str>) {
    //    e,n = (6983, 36917767)
    let e: u64 = 10711;
    let n: u64 = 66732557;
    // 847111
    (800_000..100_000_000_u64)
        // (847100..847120_u64)
        // ((32022215 - 20)..(32022215 + 20_u64))
        // .into_par_iter()
        .for_each(|i| {
            // let dec = pows(37589154, i, n);
            // let s = format!("{:0>width$b}", dec, width = 25);
            // let mut word = "".to_string();

            // for i in (0..25).step_by(5) {
            //     let c = &s[i..(i + 5)];
            //     word.push_str(decode.get(c).unwrap());
            // }
            // assert_eq!(word, decode_block(37589154, i, decode, n));

            let maybe = decode_message(
                &[i],
                // &[3282295, 28313038, 3319914, 33560101, 10977960],
                &[
                    37589154, 22319859, 49601601, 17211352, 21322215, 14970886, 1649821, 45204256,
                    18110763, 11247215, 48752797,
                    22991738,
                    //  50403000, 4416778, 23378941, 63942721,
                    // 36627768, 54604762, 25150413, 60094010, 22829520, 57720301,
                    // 716646,
                    // 63941997, 7696790, 33885967, 42308121,
                ],
                &decode,
                n,
            )[0]
            .join("");

            // if maybe.starts_with("death") {
            //     println!("{} {:?}", i, maybe);
            // } else {
            //     return;
            // }

            // dbg!(i, &maybe);
            // .iter()
            // .map(|x| &x[..])

            if i % 10_000 == 0 {
                println!("{}", i);
            }
            let mmaybe: HashSet<&str> = maybe.split(' ').filter(|&x| x != "").collect();

            if vocab.intersection(&mmaybe).count() >= 2 {
                println!("{} {:?}", i, maybe);
            }
            // println!("{:?}", maybe);
        })
}

fn mul(a: u64, b: u64, m: u64) -> u64 {
    if b == 1 {
        return a;
    }
    if b & 1 == 0 {
        let t = mul(a, b / 2, m);
        return (2 * t) % m;
    }
    (mul(a, b - 1, m) + a) % m
}

fn pows(a: u64, b: u64, m: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    if b & 1 == 0 {
        let t = pows(a, b / 2, m);
        return mul(t, t, m) % m;
    }
    mul(pows(a, b - 1, m), a, m) % m
}

fn decode_block(block: u64, d: u64, de: &HashMap<&str, &str>, n: u64) -> String {
    let dec = pows(block, d, n);
    let mut s = format!("{:0>width$b}", dec, width = 25);
    // dbg!("Was", &s);
    s = s.chars().rev().collect();
    // dbg!("Became", &s);
    let mut word = "".to_string();
    (0..25).step_by(5).for_each(|i| {
        word.push_str(de.get(&s[i..(i + 5)]).unwrap());
    });

    // dbg!(&word);
    // word
    // print!("{}", &word);
    word
}

fn decode_message(
    keys: &[u64],
    cipher: &[u64],
    de: &HashMap<&str, &str>,
    n: u64,
) -> Vec<Vec<String>> {
    keys.iter()
        .map(|&k| {
            // println!("Decoding 2 blocks");
            cipher
                .par_iter()
                .map(|&c| decode_block(c, k, de, n))
                .collect()
            // println!("");
        })
        .collect()
}
