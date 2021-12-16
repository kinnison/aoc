use std::io::Cursor;

use aoc2021::*;
use bitstream_io::{BigEndian, BitRead, BitReader};

enum Packet {
    Literal(u8, i64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    fn read_from<T>(stream: &mut T) -> Result<Option<(Self, u32)>>
    where
        T: BitRead,
    {
        let ver = if let Ok(v) = stream.read(3) {
            v
        } else {
            return Ok(None);
        };
        let ptype: u8 = if let Ok(v) = stream.read(3) {
            v
        } else {
            return Ok(None);
        };
        if ptype == 4 {
            let mut value = 0;
            let mut nybcount = 0;
            loop {
                let nybble: i64 = stream.read(5)?;
                nybcount += 1;
                value <<= 4;
                value |= nybble & 0xf;
                if nybble >> 4 == 0 {
                    break;
                }
            }
            Ok(Some((Packet::Literal(ver, value), 3 + 3 + (5 * nybcount))))
        } else {
            // Operators have one or two length kinds...
            let mut total_read = 1;
            let packets = if stream.read_bit()? {
                // length is 11 bit packet count
                let mut packets = Vec::new();
                let pcount: u32 = stream.read(11)?;
                total_read += 11;
                for _ in 0..pcount {
                    if let Some((packet, nbits)) = Self::read_from(stream)? {
                        packets.push(packet);
                        total_read += nbits;
                    } else {
                        panic!(
                            "Ran out of bits reading subpackets, I still expected {} more packets",
                            (pcount as usize) - packets.len()
                        );
                    }
                }
                packets
            } else {
                // length is a 15 bit count of bits in subpackets
                let mut packets = Vec::new();
                let mut pbits: u32 = stream.read(15)?;
                total_read += 15;
                loop {
                    if let Some((packet, plen)) = Self::read_from(stream)? {
                        packets.push(packet);
                        total_read += plen;
                        if plen < pbits {
                            pbits -= plen;
                        } else {
                            break;
                        }
                    }
                }
                packets
            };
            Ok(Some((
                Packet::Operator(ver, ptype, packets),
                3 + 3 + total_read,
            )))
        }
    }

    fn read_all(input: &str) -> Result<Self> {
        let input = if input.len() % 2 == 0 {
            input.to_owned()
        } else {
            format!("{}0", input)
        };
        let input = input
            .into_bytes()
            .into_iter()
            .tuples()
            .map(|(h, l)| (hex_byte_to_value(h) << 4) | hex_byte_to_value(l))
            .collect_vec();
        let input = Cursor::new(input);
        let mut input = BitReader::endian(input, BigEndian);
        if let Some(v) = Self::read_from(&mut input)? {
            Ok(v.0)
        } else {
            Err("No idea".into())
        }
    }

    fn version_sum(&self) -> i64 {
        match self {
            Packet::Literal(ver, _) => *ver as i64,
            Packet::Operator(ver, _, pv) => {
                (*ver as i64) + pv.iter().map(|p| p.version_sum()).sum::<i64>()
            }
        }
    }

    fn value(&self) -> i64 {
        match self {
            Packet::Literal(_, v) => *v,
            Packet::Operator(_, op, pv) => match *op {
                0 =>
                /* sum */
                {
                    pv.iter().map(|p| p.value()).sum()
                }
                1 =>
                /* product */
                {
                    pv.iter().map(|p| p.value()).product()
                }
                2 =>
                /* min */
                {
                    pv.iter().map(|p| p.value()).min().unwrap()
                }
                3 =>
                /* max */
                {
                    pv.iter().map(|p| p.value()).max().unwrap()
                }
                4 => unreachable!(),
                5 =>
                /* gt  */
                {
                    if pv[0].value() > pv[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 =>
                /* lt  */
                {
                    if pv[0].value() < pv[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 =>
                /* eq  */
                {
                    if pv[0].value() == pv[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn part1(input: &Packet) -> i64 {
    input.version_sum()
}

fn part2(input: &Packet) -> i64 {
    input.value()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT1: &[(&str, i64)] =
        &[("D2FE28", 6), ("38006F45291200", 9), ("EE00D40C823060", 14)];
    static TEST_INPUT2: &[(&str, i64)] = &[
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];
    #[test]
    fn testcase1() {
        for (input, target) in TEST_INPUT1.iter().copied() {
            let input = Packet::read_all(input).unwrap();
            assert_eq!(part1(&input), target);
        }
    }

    #[test]
    fn testcase2() {
        for (input, target) in TEST_INPUT2.iter().copied() {
            let input = Packet::read_all(input).unwrap();
            assert_eq!(part2(&input), target);
        }
    }
}

fn main() -> Result<()> {
    let input = read_input(16)?;
    let input = Packet::read_all(input.trim())?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
