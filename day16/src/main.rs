#[derive(Debug)]
struct Packet {
    version: u64,
    packet_type: PacketType,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version,
            PacketType::Operator { op: _, subpackets } => {
                self.version + subpackets.iter().map(|p| p.version_sum()).sum::<u64>()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(value) => *value,
            PacketType::Operator { op, subpackets } => {
                let mut subvalues = subpackets.iter().map(|p| p.evaluate());

                match op {
                    Op::Sum => subvalues.sum(),
                    Op::Prod => subvalues.product(),
                    Op::Min => subvalues.min().unwrap(),
                    Op::Max => subvalues.max().unwrap(),
                    Op::GT => (subvalues.next().unwrap() > subvalues.next().unwrap()) as u64,
                    Op::LT => (subvalues.next().unwrap() < subvalues.next().unwrap()) as u64,
                    Op::EQ => (subvalues.next().unwrap() == subvalues.next().unwrap()) as u64,
                }
            }
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(u64),
    Operator { op: Op, subpackets: Vec<Packet> },
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
    let type_id = consume(input, 3);

    if type_id == 4 {
        let mut value = 0;
        loop {
            value <<= 4;
            let group = consume(input, 5);
            value += group & 0xF;
            if group & 0x10 == 0 {
                break;
            }
        }
        return Packet {
            version,
            packet_type: PacketType::Literal(value),
        };
    }

    let length_type_id = consume(input, 1);

    let mut subpackets = Vec::new();

    match length_type_id {
        0 => {
            let length = consume(input, 15);
            let (mut left, right) = input.split_at(length as usize);
            *input = right;

            while !left.is_empty() {
                subpackets.push(parse(&mut left));
            }
        }
        1 => {
            let count = consume(input, 11);
            for _ in 0..count {
                subpackets.push(parse(input))
            }
        }
        _ => panic!(),
    };

    let op = match type_id {
        0 => Op::Sum,
        1 => Op::Prod,
        2 => Op::Min,
        3 => Op::Max,
        5 => Op::GT,
        6 => Op::LT,
        7 => Op::EQ,
        _ => panic!(),
    };

    Packet {
        version,
        packet_type: PacketType::Operator { op, subpackets },
    }
}

fn consume(input: &mut &str, bits: usize) -> u64 {
    let (left, right) = input.split_at(bits);
    *input = right;
    u64::from_str_radix(left, 2).unwrap()
}
