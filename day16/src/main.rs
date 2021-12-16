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

    let (packet, _) = parse(&input);

    println!(
        "Part 1: {}\nPart 2: {}",
        packet.version_sum(),
        packet.evaluate()
    );
}

fn parse(pkg: &str) -> (Packet, &str) {
    let (version, rest) = pkg.split_at(3);
    let (type_id, mut rest) = rest.split_at(3);

    let version = u64::from_str_radix(version, 2).unwrap();
    let type_id = u64::from_str_radix(type_id, 2).unwrap();

    if type_id == 4 {
        let mut literal = 0;
        loop {
            literal = literal << 4;
            literal += u64::from_str_radix(&rest[1..5], 2).unwrap();
            let is_last = rest.starts_with('0');
            rest = &rest[5..];
            if is_last {
                break;
            }
        }
        return (
            Packet {
                version,
                packet_type: PacketType::Literal(literal),
            },
            rest,
        );
    }

    let (length_type_id, mut rest) = rest.split_at(1);

    let mut subpackets = Vec::new();

    rest = match length_type_id {
        "0" => {
            let (length, rest) = rest.split_at(15);
            let length = usize::from_str_radix(length, 2).unwrap();
            let (mut rest, ret) = rest.split_at(length);

            while !rest.is_empty() {
                let (subpacket, subrest) = parse(rest);
                subpackets.push(subpacket);
                rest = subrest;
            }
            ret
        }
        "1" => {
            let (count, mut rest) = rest.split_at(11);
            let count = usize::from_str_radix(count, 2).unwrap();

            for _ in 0..count {
                let (subpacket, subrest) = parse(rest);
                rest = subrest;
                subpackets.push(subpacket);
            }
            rest
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

    (
        Packet {
            version,
            packet_type: PacketType::Operator { op, subpackets },
        },
        rest,
    )
}
