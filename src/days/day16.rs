use std::fs;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    literal: bool,
    literal_value: i128,
    length_type_id: bool,
    packets: Vec<Packet>,
    start: usize,
    end: usize,
    version_sum: usize,
}

impl Packet {
    fn new(bit_stream: &Vec<u8>, mut index: usize) -> Self {
        let start = index;
        let mut version_sum: usize = 0;
        let version = {
            let mut sum = 0;
            for i in (0..3).rev() {
                sum += bit_stream[index] << i;
                index += 1;
            }
            sum
        };

        version_sum += version as usize;

        let type_id = {
            let mut sum = 0;
            for i in (0..3).rev() {
                sum += bit_stream[index] << i;
                index += 1;
            }
            sum
        };

        let literal;
        let mut literal_value = 0_i128;
        let mut length_type_id = false;
        let mut packets: Vec<Packet> = vec![];

        if type_id == 4 {
            // Literal
            literal = true;
            loop {
                let start = bit_stream[index];
                index += 1;

                for i in (0..4).rev() {
                    literal_value += (bit_stream[index] as i128) << i;
                    index += 1;
                }

                if start == 0 {
                    break;
                } else {
                    literal_value = literal_value << 4;
                }
            }
        } else {
            length_type_id = if bit_stream[index] == 1 { true } else { false };
            index += 1;

            if length_type_id {
                let mut num_sub_packets: u16 = 0;
                for i in (0..11).rev() {
                    num_sub_packets += (bit_stream[index] as u16) << i;
                    index += 1;
                }

                for _i in 0..num_sub_packets {
                    let sub_packet = Packet::new(bit_stream, index);
                    index = sub_packet.end;
                    version_sum += sub_packet.version_sum;
                    packets.push(sub_packet);
                }
            } else {
                let mut length_packets: u16 = 0;
                for i in (0..15).rev() {
                    length_packets += (bit_stream[index] as u16) << i;
                    index += 1;
                }
                let mut length_reamaining = length_packets;
                while length_reamaining > 0 {
                    let sub_packet = Packet::new(bit_stream, index);
                    length_reamaining -= sub_packet.end as u16 - index as u16;
                    index = sub_packet.end;
                    version_sum += sub_packet.version_sum;
                    packets.push(sub_packet);
                }
            }

            literal_value = match type_id {
                0 => {
                    let mut sum = 0;
                    for p in &packets {
                        sum += p.literal_value;
                    }
                    sum
                }
                1 => {
                    let mut prod = 1;
                    for p in &packets {
                        prod *= p.literal_value;
                    }
                    prod
                }
                2 => {
                    let mut min = i128::MAX;
                    for p in &packets {
                        if p.literal_value < min {
                            min = p.literal_value;
                        }
                    }
                    min
                }
                3 => {
                    let mut max = i128::MIN;
                    for p in &packets {
                        if p.literal_value > max {
                            max = p.literal_value;
                        }
                    }
                    max
                }
                5 => {
                    if packets[0].literal_value > packets[1].literal_value {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].literal_value < packets[1].literal_value {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].literal_value == packets[1].literal_value {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            };

            literal = false;
        }

        let end = index;

        Self {
            version,
            type_id,
            literal,
            literal_value,
            length_type_id,
            packets,
            start,
            end,
            version_sum,
        }
    }
}

fn string_to_bit_stream(s: String) -> Vec<u8> {
    s.trim()
        .chars()
        .map(|c| match c {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => unreachable!(),
        })
        .flatten()
        .collect::<Vec<u8>>()
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day16.txt").unwrap();

    let bit_stream: Vec<u8> = string_to_bit_stream(stream);

    let packet = Packet::new(&bit_stream, 0);

    let part1_sol = part1(&packet);
    let part2_sol = part2(&packet);

    (part1_sol, part2_sol)
}

fn part1(packet: &Packet) -> String {
    return packet.version_sum.to_string();
}

fn part2(packet: &Packet) -> String {
    return packet.literal_value.to_string();
}
