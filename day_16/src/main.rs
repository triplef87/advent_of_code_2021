use std::fs;

struct Packet {
    ver: usize,
    type_id: usize,
    length_info: (bool, usize),
    parent: Option<usize>,
    val: Option<usize>,
    sub_vals: Vec<usize>,
}

impl Packet {
    fn new() -> Packet {
        Packet {
            ver: 0,
            type_id: 0,
            length_info: (true, 0),
            parent: None,
            val: None,
            sub_vals: Vec::new(),
        }
    }

    fn process(&mut self, transmission: &[bool], idx: &mut usize) {
        match self.type_id {
            4 => {
                self.val = Some(decode_literal_val(transmission, idx));
            }
            _ => {
                *idx += 1;

                match transmission[*idx - 1] {
                    true => {
                        self.length_info = (true, compute_digits(transmission, idx, 11));
                    }
                    false => {
                        self.length_info = (false, compute_digits(transmission, idx, 15) + *idx);
                    }
                }
            }
        }
    }

    fn check_op_finish(&mut self, idx: &usize) -> bool {
        match self.length_info.0 {
            true => {
                self.length_info.1 -= 1;
                self.length_info.1 == 0
            }
            false => self.length_info.1 <= *idx,
        }
    }

    fn compute_op(&mut self) {
        match self.type_id {
            0 => {
                self.val = Some(self.sub_vals.iter().sum());
            }
            1 => {
                self.val = Some(self.sub_vals.iter().product());
            }
            2 => {
                self.val = Some(*self.sub_vals.iter().min().unwrap());
            }
            3 => {
                self.val = Some(*self.sub_vals.iter().max().unwrap());
            }
            5 => {
                self.val = if self.sub_vals[0] > self.sub_vals[1] {
                    Some(1)
                } else {
                    Some(0)
                };
            }
            6 => {
                self.val = if self.sub_vals[0] < self.sub_vals[1] {
                    Some(1)
                } else {
                    Some(0)
                };
            }
            7 => {
                self.val = if self.sub_vals[0] == self.sub_vals[1] {
                    Some(1)
                } else {
                    Some(0)
                };
            }
            _ => {}
        }
    }
}

fn main() {
    let transmission: Vec<bool> = read_from_input("input");
    let packets: Vec<Packet> = decode_transmission(&transmission);

    println!(
        "Part 1: {}",
        packets.iter().fold(0, |base, packet| base + packet.ver)
    );
    println!("Part 2: {}", packets[0].val.unwrap());
}

fn read_from_input(filename: &str) -> Vec<bool> {
    let mut transmission: Vec<bool> = Vec::new();

    fs::read_to_string(filename)
        .expect("Read file error")
        .chars()
        .for_each(|ch| match ch {
            '0' => transmission.append(&mut vec![false, false, false, false]),
            '1' => transmission.append(&mut vec![false, false, false, true]),
            '2' => transmission.append(&mut vec![false, false, true, false]),
            '3' => transmission.append(&mut vec![false, false, true, true]),
            '4' => transmission.append(&mut vec![false, true, false, false]),
            '5' => transmission.append(&mut vec![false, true, false, true]),
            '6' => transmission.append(&mut vec![false, true, true, false]),
            '7' => transmission.append(&mut vec![false, true, true, true]),
            '8' => transmission.append(&mut vec![true, false, false, false]),
            '9' => transmission.append(&mut vec![true, false, false, true]),
            'A' => transmission.append(&mut vec![true, false, true, false]),
            'B' => transmission.append(&mut vec![true, false, true, true]),
            'C' => transmission.append(&mut vec![true, true, false, false]),
            'D' => transmission.append(&mut vec![true, true, false, true]),
            'E' => transmission.append(&mut vec![true, true, true, false]),
            'F' => transmission.append(&mut vec![true, true, true, true]),
            _ => {}
        });
    transmission
}

fn decode_transmission(transmission: &[bool]) -> Vec<Packet> {
    let mut idx: usize = 0;
    let mut parent_idx: Option<usize> = None;
    let mut packets: Vec<Packet> = Vec::new();

    while packets.is_empty() || packets[0].val.is_none() {
        packets.push(Packet::new());

        // Parse packet header
        let packet: &mut Packet = packets.last_mut().unwrap();
        packet.ver = compute_digits(transmission, &mut idx, 3);
        packet.type_id = compute_digits(transmission, &mut idx, 3);
        packet.parent = parent_idx;

        // Process packet header
        packet.process(transmission, &mut idx);

        // Update parent index
        if packet.type_id != 4 {
            parent_idx = Some(packets.len() - 1);
            continue;
        }

        let mut tmp_parent: Option<usize> = packet.parent;
        let mut tmp_val: usize = packet.val.unwrap();
        while let Some(tmp_idx) = tmp_parent {
            let parent_packet: &mut Packet = packets.get_mut(tmp_idx).unwrap();
            parent_packet.sub_vals.push(tmp_val);

            // Check operation is finish
            if !parent_packet.check_op_finish(&idx) {
                parent_idx = tmp_parent;
                break;
            }

            // Compute operation value and update iter
            parent_packet.compute_op();
            tmp_val = parent_packet.val.unwrap();
            tmp_parent = parent_packet.parent;
        }
    }

    packets
}

fn compute_digits(transmission: &[bool], idx: &mut usize, len: usize) -> usize {
    let mut val: usize = 0;

    (0..len).for_each(|offset| {
        if transmission[*idx] {
            val += 1 << (len - 1 - offset);
        }
        *idx += 1;
    });

    val
}

fn decode_literal_val(transmission: &[bool], idx: &mut usize) -> usize {
    let mut val_stack: Vec<bool> = Vec::new();

    loop {
        *idx += 5;
        val_stack.push(transmission[*idx - 4]);
        val_stack.push(transmission[*idx - 3]);
        val_stack.push(transmission[*idx - 2]);
        val_stack.push(transmission[*idx - 1]);

        if !transmission[*idx - 5] {
            break;
        }
    }

    let val: usize =
        val_stack.iter().rev().enumerate().fold(
            0,
            |base, (idx, bit)| {
                if *bit {
                    base + (1 << idx)
                } else {
                    base
                }
            },
        );

    val
}
