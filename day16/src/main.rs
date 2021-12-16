#[derive(Debug)]
struct Packet {
    version: u64,
    packet_type: PacketType,
}

#[derive(Debug)]
enum PacketType {
    Literal(u64),
    Operator { op: Op, packets: Vec<Packet> },
}

#[derive(Debug)]
enum Op {
    Sum,
    Prod,
    Min,
    Max,
    GT,
    LT,
    EQ,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version,
            PacketType::Operator { op: _, packets } => {
                self.version + packets.iter().map(|p| p.version_sum()).sum::<u64>()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(value) => *value,
            PacketType::Operator { op, packets } => {
                let mut values = packets.iter().map(|p| p.evaluate());

                match op {
                    Op::Sum => values.sum(),
                    Op::Prod => values.product(),
                    Op::Min => values.min().unwrap(),
                    Op::Max => values.max().unwrap(),
                    Op::GT => (values.next().unwrap() > values.next().unwrap()) as u64,
                    Op::LT => (values.next().unwrap() < values.next().unwrap()) as u64,
                    Op::EQ => (values.next().unwrap() == values.next().unwrap()) as u64,
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input").trim();

    let input: String = input
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!(),
        })
        .collect();

    let packet = parse(&mut input.as_str());

    println!(
        "Part 1: {}\nPart 2: {}",
        packet.version_sum(),
        packet.evaluate()
    );
}

fn parse(input: &mut &str) -> Packet {
    let version = consume(input, 3);

    let packet_type = match consume(input, 3) {
        4 => {
            let mut value = 0;
            loop {
                let group = consume(input, 5);
                value = (value << 4) + (group & 0b1111);
                if group >> 4 == 0 {
                    break PacketType::Literal(value);
                }
            }
        }
        type_id => {
            let mut packets = Vec::new();

            match consume(input, 1) {
                0 => {
                    let length = consume(input, 15);
                    let (mut left, right) = input.split_at(length as usize);
                    *input = right;

                    while !left.is_empty() {
                        packets.push(parse(&mut left));
                    }
                }
                1 => {
                    let count = consume(input, 11);
                    for _ in 0..count {
                        packets.push(parse(input))
                    }
                }
                invalid => panic!("Invalid length type ID: {}", invalid),
            };

            let op = match type_id {
                0 => Op::Sum,
                1 => Op::Prod,
                2 => Op::Min,
                3 => Op::Max,
                5 => Op::GT,
                6 => Op::LT,
                7 => Op::EQ,
                invalid => panic!("Invalid type ID: {}", invalid),
            };

            PacketType::Operator { op, packets }
        }
    };

    Packet {
        version,
        packet_type,
    }
}

fn consume(input: &mut &str, bits: usize) -> u64 {
    let (left, right) = input.split_at(bits);
    *input = right;
    u64::from_str_radix(left, 2).unwrap()
}
