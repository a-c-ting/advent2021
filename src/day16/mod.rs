use std::cmp;

use crate::shared_utils::read_input;
const BIT4: u8 = 4;

pub fn execute() {
    let file_contents = read_input(".\\input\\day16.txt");

    let mut reader = convert_from_utf8(file_contents.trim_end());
    reader.analyze_packets();
    reader.process_packets();

}

struct PacketReader {
    packet: Vec<usize>,
    stack: Vec<Packet>,
    packet_versions: Vec<usize>,
}

enum State {
    ReadVer,
    ReadId,
    ReadLiteral,
    ReadOperator,
}

enum Packet {
    Literal(usize, usize), //(Len, Num)
    Operator(OpType, usize, LenType, usize), // (Opertor, len, type, len of subpackets)

}

enum OpType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

enum LenType {
    Bits,
    Count,
}

fn convert_from_utf8(sequence: &str) -> PacketReader {
    let mut bit_sequence: Vec<usize> = Vec::new();
    for x in sequence.chars() {
        if !x.is_ascii_hexdigit() {
            panic!("Only hex allowed.");
        }
        let rev: u8 = ( (x.to_digit(16).unwrap() as u8).reverse_bits() ) >> BIT4;
        for shift in 0..4 {
            let mask = 0b0001;
            bit_sequence.push((rev as usize >> shift) & mask);
        }
    }

    bit_sequence.reverse();
    PacketReader::new(bit_sequence)
}

impl PacketReader {
    fn new(packet: Vec<usize>) -> PacketReader {
        PacketReader {
            packet,
            stack: Vec::new(),
            packet_versions: Vec::new(),
        }
    }

    fn analyze_packets(self: &mut Self) {
        let mut state = State::ReadVer;
        let mut packet_ver: usize;
        let mut packet_id: usize = 0;

        while !self.packet.is_empty() {
            match state {
                State::ReadVer => {
                    if let Some(ver) = self.get_next_bits(3) {
                        packet_ver = ver;
                        self.packet_versions.push(packet_ver);

                        state = State::ReadId;
                    } else {
                        break; //leftover bits are tossed
                    }
                },
                State::ReadId => {
                    if let Some(id) = self.get_next_bits(3) {
                        packet_id = id;

                        match packet_id {
                            4 => state = State::ReadLiteral,
                            _ => state = State::ReadOperator,
                        }
                    } else {
                        break; //leftover bits are tossed
                    }
                },
                State::ReadLiteral => {
                    let mut num;
                    let mut continue_bit;
                    let mut size = 5 + 6; //6 is from header

                    if let Some(chunk) = self.get_next_bits(5) {
                        continue_bit = chunk & 10000;
                        num = chunk & 0b01111;
                    } else {
                        break; //leftover bits are tossed
                    }

                    while continue_bit != 0 {
                        if let Some(chunk) = self.get_next_bits(5) {
                            continue_bit = chunk & 10000;
                            num = (num << BIT4) | (chunk & 0b01111);
                            size += 5;
                        }
                    }

                    let lit = Packet::Literal(size, num);

                    self.stack.push(lit);

                    state = State::ReadVer;
                },
                State::ReadOperator => {
                    let len_bits_len: usize;
                    let l_type: LenType;

                    if let Some(len_type) = self.get_next_bits(1) {
                        match len_type {
                            0 => {
                                len_bits_len = 15;
                                l_type = LenType::Bits;
                            },
                            1 => {
                                len_bits_len = 11;
                                l_type = LenType::Count;
                            },
                            _ => unreachable!(),
                        }
                    } else {
                        break;
                    }

                    let operand_len;
                    if let Some(len) = self.get_next_bits(len_bits_len) {
                        operand_len = len;
                    } else {
                        unreachable!();
                    }

                    let total_len = len_bits_len + 1 + 6;//6 is header
                    let oper = match packet_id {
                        0 => Packet::Operator(OpType::Sum, total_len, l_type, operand_len),
                        1 => Packet::Operator(OpType::Product, total_len, l_type, operand_len),
                        2 => Packet::Operator(OpType::Min, total_len, l_type, operand_len),
                        3 => Packet::Operator(OpType::Max, total_len, l_type, operand_len),
                        5 => Packet::Operator(OpType::GreaterThan, total_len, l_type, operand_len),
                        6 => Packet::Operator(OpType::LessThan, total_len, l_type, operand_len),
                        7 => Packet::Operator(OpType::Equal,total_len, l_type, operand_len),
                        _ => unreachable!()
                    };

                    self.stack.push(oper);

                    state = State::ReadVer;
                },
            }
        }
        println!("Sum of all Packet Ver:\n{}\n", self.add_packet_vers());
    }

    fn process_packets(&mut self) {
        let mut held_num: Vec<(usize, usize)> = Vec::new(); //len, num
        while !self.stack.is_empty() {
            match self.stack.pop().unwrap() {
                Packet::Literal(len, num) => {
                    held_num.push((len, num));
                },
                Packet::Operator(sub, len, typ, operand_len) => {
                    let mut t_oplen = operand_len;
                    let mut temp: Vec<usize> = Vec::new();

                    let mut result_len;
                    match typ {
                        LenType::Bits => {
                            let mut total_len = 0;
                            while t_oplen != 0 {
                                let (t_num_len, t_num) = held_num.pop().unwrap();

                                temp.push(t_num);
                                t_oplen -= t_num_len;
                                total_len += t_num_len;
                            }
                            result_len = total_len;//+header
                        },
                        LenType::Count => {
                            let mut total_len = 0;
                            while t_oplen != 0 {
                                let (t_num_len, t_num) = held_num.pop().unwrap();

                                temp.push(t_num);
                                t_oplen -= 1;
                                total_len += t_num_len;
                            }
                            result_len = total_len;//+header
                        },
                    }

                    let mut result;
                    match sub {
                        OpType::Sum => {
                            result = 0;
                            for x in temp {
                                result += x;
                            }
                        },
                        OpType::Product => {
                            result = 1;
                            for x in temp {
                                result *= x;
                            }
                        },
                        OpType::Min => {
                            result = usize::MAX;
                            for x in temp {
                                result = cmp::min(result, x);
                            }
                        }
                        OpType::Max => {
                            result = usize::MIN;
                            for x in temp {
                                result = cmp::max(result, x);
                            }
                        }
                        OpType::GreaterThan => {
                            if temp[0] > temp[1] {
                                result = 1;
                            } else {
                                result = 0;
                            }
                        }
                        OpType::LessThan => {
                            if temp[0] < temp[1] {
                                result = 1;
                            } else {
                                result = 0;
                            }
                        }
                        OpType::Equal => {
                            if temp[1] == temp[0] {
                                result = 1;
                            } else {
                                result = 0;
                            }
                        }
                    }

                    result_len += len;
                    held_num.push((result_len, result));
                },
            }
        }

        println!("Answer to decoded BITS transmission:\n{}", held_num[0].1);
    }

    fn get_next_bits(&mut self, num_of_bits: usize) -> Option<usize> {
        if num_of_bits == 0 {
            println!("Can't get none.");
            return None
        }

        if num_of_bits > self.packet.len() {
            println!("Not enough bits. Parsing Ends. Leftover: {} bits\n", self.packet.len());
            return None
        }

        let mut x_bits: usize = self.packet.pop().unwrap();
        for _ in (0..num_of_bits).skip(1) {
            x_bits = (x_bits << 1) | self.packet.pop().unwrap();
        }
        Some(x_bits)
    }

    fn add_packet_vers(&self) -> usize {
        let mut sum = 0;
        for ver in &self.packet_versions {
            sum += ver;
        }
        sum
    }
}
