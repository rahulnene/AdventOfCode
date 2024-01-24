use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    time::{Duration, Instant},
};
pub fn solution() -> ((isize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_18.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (isize, Duration) {
    let now = Instant::now();
    // let mut computer = Computer::new();
    // computer.instructions = lines.lines().map(Instruction::from_str).collect();
    // (computer.run(), now.elapsed())
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let is_waiting_1 = Arc::new(AtomicBool::new(false));
    let is_waiting_2 = Arc::new(AtomicBool::new(false));
    let instrs = lines.lines().map(Instruction::from_str).collect::<Vec<_>>();
    let (sender1, receiver1) = mpsc::channel::<isize>();
    let (sender2, receiver2) = mpsc::channel::<isize>();
    let mut computer1 = Computer::new(
        0,
        sender1,
        receiver2,
        Arc::clone(&is_waiting_1),
        Arc::clone(&is_waiting_2),
    );
    let mut computer2 = Computer::new(
        1,
        sender2,
        receiver1,
        Arc::clone(&is_waiting_2),
        Arc::clone(&is_waiting_1),
    );
    computer1.instructions = instrs.clone();
    computer2.instructions = instrs;
    computer2.registers['p' as usize - 'a' as usize] = 1;

    let thread1 = std::thread::spawn(move || computer1.run());
    let thread2 = std::thread::spawn(move || computer2.run());

    let ans = thread2.join().unwrap();
    (ans, now.elapsed())
}

// #[derive(Debug, Clone)]
// struct ComputerNetwork {
//     computers: [Computer; 2],
//     comp_wait_status: [bool; 2],
//     times_sent: [usize; 2],
// }

// impl ComputerNetwork {
//     fn new() -> Self {
//         ComputerNetwork {
//             computers: [Computer::new(), Computer::new()],
//             comp_wait_status: [false; 2],
//             times_sent: [0; 2],
//         }
//     }

//     fn run(&mut self) -> usize {
//         loop {
//             if self.comp_wait_status[0] && self.comp_wait_status[1] {
//                 return self.times_sent[1];
//             }
//             for comp_index in 0..2 {
//                 let current_instr =
//                     self.computers[comp_index].instructions[self.computers[comp_index].instr_ptr];
//                 match current_instr {
//                     Instruction::SndReg(r) => {
//                         self.computers[comp_index.abs_diff(1)]
//                             .message_queue
//                             .push_back(
//                                 self.computers[comp_index].registers[r as usize - 'a' as usize],
//                             );
//                         self.times_sent[comp_index] += 1;
//                     }
//                     Instruction::SndImm(x) => {
//                         self.computers[comp_index.abs_diff(1)]
//                             .message_queue
//                             .push_back(x);
//                         self.times_sent[comp_index] += 1;
//                     }
//                     Instruction::SetImm(rx, y) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] =
//                             y as isize
//                     }
//                     Instruction::SetReg(rx, ry) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] =
//                             self.computers[comp_index].registers[ry as usize - 'a' as usize]
//                     }
//                     Instruction::AddImm(rx, y) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] +=
//                             y as isize
//                     }
//                     Instruction::AddReg(rx, ry) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] +=
//                             self.computers[comp_index].registers[ry as usize - 'a' as usize]
//                     }
//                     Instruction::MulImm(rx, y) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] *=
//                             y as isize
//                     }
//                     Instruction::MulReg(rx, ry) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] *=
//                             self.computers[comp_index].registers[ry as usize - 'a' as usize]
//                     }
//                     Instruction::ModImm(rx, ry) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] %=
//                             ry as isize
//                     }
//                     Instruction::ModReg(rx, ry) => {
//                         self.computers[comp_index].registers[rx as usize - 'a' as usize] %=
//                             self.computers[comp_index].registers[ry as usize - 'a' as usize]
//                     }
//                     Instruction::RcvImm(x) => {
//                         if x != 0 {
//                             while self.computers[comp_index].message_queue.is_empty() {
//                                 self.comp_wait_status[comp_index] = true;
//                             }
//                             self.comp_wait_status[comp_index] = false;
//                             self.computers[comp_index].registers[x as usize] = self.computers
//                                 [comp_index]
//                                 .message_queue
//                                 .pop_front()
//                                 .unwrap();
//                         }
//                     }
//                     Instruction::RcvReg(rx) => {
//                         if self.computers[comp_index].registers[rx as usize - 'a' as usize] != 0 {
//                             while self.computers[comp_index].message_queue.is_empty() {
//                                 self.comp_wait_status[comp_index] = true;
//                             }
//                             self.comp_wait_status[comp_index] = false;
//                             self.computers[comp_index].registers[rx as usize - 'a' as usize] = self
//                                 .computers[comp_index]
//                                 .message_queue
//                                 .pop_front()
//                                 .unwrap();
//                         }
//                     }
//                     Instruction::JgzImm(rx, y) => {
//                         if self.computers[comp_index].registers[rx as usize - 'a' as usize] > 0 {
//                             self.computers[comp_index].instr_ptr =
//                                 (self.computers[comp_index].instr_ptr as isize + y) as usize;
//                             continue;
//                         }
//                     }
//                     Instruction::JgzReg(rx, ry) => {
//                         if self.computers[comp_index].registers[rx as usize - 'a' as usize]
//                             > self.computers[comp_index].registers[ry as usize - 'a' as usize]
//                         {
//                             self.computers[comp_index].instr_ptr = (self.computers[comp_index]
//                                 .instr_ptr
//                                 as isize
//                                 + self.computers[comp_index].registers[ry as usize - 'a' as usize])
//                                 as usize;
//                             continue;
//                         }
//                     }
//                     Instruction::Jmp(x) => {
//                         self.computers[comp_index].instr_ptr =
//                             (self.computers[comp_index].instr_ptr as isize + x) as usize;
//                         continue;
//                     }
//                 }
//                 self.computers[comp_index].instr_ptr += 1;
//                 if self.computers[comp_index].instr_ptr
//                     >= self.computers[comp_index].instructions.len()
//                 {
//                     return self.times_sent[1];
//                 }
//             }
//         }
//     }
// }

#[derive(Debug)]
struct MessageQueue<T> {
    messages: VecDeque<T>,
    receiver: Receiver<T>,
}

impl<T> MessageQueue<T> {
    fn new(receiver: Receiver<T>) -> Self {
        MessageQueue {
            messages: VecDeque::new(),
            receiver,
        }
    }

    fn check_messages(&mut self) -> bool {
        while let Ok(message) = self.receiver.try_recv() {
            self.messages.push_back(message);
        }
        self.messages.is_empty()
    }

    fn get_message(&mut self) -> Option<T> {
        self.check_messages();
        self.messages.pop_front()
    }
}

#[derive(Debug)]
struct Computer {
    id: usize,
    registers: [isize; 26],
    instructions: Vec<Instruction>,
    instr_ptr: usize,
    last_sound: isize,
    sender: Sender<isize>,
    message_queue: MessageQueue<isize>,
    wait_status_self: Arc<AtomicBool>,
    wait_status_other: Arc<AtomicBool>,
    times_sent: usize,
}

impl Computer {
    fn new(
        id: usize,
        sender: Sender<isize>,
        receiver: Receiver<isize>,
        wait_status_self: Arc<AtomicBool>,
        wait_status_other: Arc<AtomicBool>,
    ) -> Self {
        Computer {
            id,
            registers: [0; 26],
            instructions: Vec::new(),
            instr_ptr: 0,
            last_sound: 0,
            message_queue: MessageQueue::new(receiver),
            sender,
            times_sent: 0,
            wait_status_self,
            wait_status_other,
        }
    }

    fn run(&mut self) -> usize {
        loop {
            if self.wait_status_self.load(Ordering::Relaxed)
                && self.wait_status_other.load(Ordering::Relaxed)
            {
                return self.times_sent;
            }
            let current_instr = self.instructions[self.instr_ptr];
            match current_instr {
                Instruction::SndReg(r) => {
                    self.last_sound = self.registers[r as usize - 'a' as usize];
                    self.sender.send(self.last_sound).unwrap();
                }
                Instruction::SndImm(x) => {
                    self.last_sound = x;
                    self.sender.send(x).unwrap();
                }
                Instruction::SetImm(rx, y) => {
                    self.registers[rx as usize - 'a' as usize] = y as isize
                }
                Instruction::SetReg(rx, ry) => {
                    self.registers[rx as usize - 'a' as usize] =
                        self.registers[ry as usize - 'a' as usize]
                }
                Instruction::AddImm(rx, y) => {
                    self.registers[rx as usize - 'a' as usize] += y as isize
                }
                Instruction::AddReg(rx, ry) => {
                    self.registers[rx as usize - 'a' as usize] +=
                        self.registers[ry as usize - 'a' as usize]
                }
                Instruction::MulImm(rx, y) => {
                    self.registers[rx as usize - 'a' as usize] *= y as isize
                }
                Instruction::MulReg(rx, ry) => {
                    self.registers[rx as usize - 'a' as usize] *=
                        self.registers[ry as usize - 'a' as usize]
                }
                Instruction::ModImm(rx, ry) => {
                    self.registers[rx as usize - 'a' as usize] %= ry as isize
                }
                Instruction::ModReg(rx, ry) => {
                    self.registers[rx as usize - 'a' as usize] %=
                        self.registers[ry as usize - 'a' as usize]
                }
                Instruction::RcvImm(x) => {
                    if x != 0 {
                        while self.message_queue.check_messages() {
                            self.wait_status_self
                                .store(true, std::sync::atomic::Ordering::Relaxed)
                        }
                        self.wait_status_self.store(false, Ordering::Relaxed);
                        if let Some(message) = self.message_queue.get_message() {
                            self.registers[x as usize] = message;
                        }
                    }
                }
                Instruction::RcvReg(rx) => {
                    if self.registers[rx as usize - 'a' as usize] != 0 {
                        while self.message_queue.check_messages() {
                            self.wait_status_self
                                .store(true, std::sync::atomic::Ordering::Relaxed)
                        }
                        self.wait_status_self.store(false, Ordering::Relaxed);
                        if let Some(message) = self.message_queue.get_message() {
                            self.registers[rx as usize - 'a' as usize] = message;
                        }
                    }
                }
                Instruction::JgzImm(rx, y) => {
                    if self.registers[rx as usize - 'a' as usize] > 0 {
                        self.instr_ptr = (self.instr_ptr as isize + y) as usize;
                        continue;
                    }
                }
                Instruction::JgzReg(rx, ry) => {
                    if self.registers[rx as usize - 'a' as usize]
                        > self.registers[ry as usize - 'a' as usize]
                    {
                        self.instr_ptr = (self.instr_ptr as isize
                            + self.registers[ry as usize - 'a' as usize])
                            as usize;
                        continue;
                    }
                }
                Instruction::Jmp(x) => {
                    self.instr_ptr = (self.instr_ptr as isize + x) as usize;
                    continue;
                }
            }
            self.instr_ptr += 1;
            if self.instr_ptr >= self.instructions.len() {
                println!("Reached end of instructions");
                return 0;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SndReg(char),
    SndImm(isize),
    SetImm(char, isize),
    SetReg(char, char),
    AddImm(char, isize),
    AddReg(char, char),
    MulImm(char, isize),
    MulReg(char, char),
    ModImm(char, isize),
    ModReg(char, char),
    RcvImm(isize),
    RcvReg(char),
    Jmp(isize),
    JgzImm(char, isize),
    JgzReg(char, char),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut words = s.split_whitespace();
        let instruction = words.next().unwrap();
        let register = words.next().unwrap();
        match instruction {
            "snd" => {
                if let Some(num) = register.parse::<isize>().ok() {
                    Instruction::SndImm(num)
                } else {
                    Instruction::SndReg(register.chars().next().unwrap())
                }
            }
            "set" => {
                let value = words.next().unwrap();
                if let Some(num) = value.parse::<isize>().ok() {
                    let register = register.chars().next().unwrap();
                    Instruction::SetImm(register, num)
                } else {
                    let register = register.parse().unwrap();
                    Instruction::SetReg(register, value.chars().next().unwrap())
                }
            }
            "add" => {
                let value = words.next().unwrap();
                if let Some(num) = value.parse::<isize>().ok() {
                    let register = register.chars().next().unwrap();
                    Instruction::AddImm(register, num)
                } else {
                    let register = register.parse().unwrap();
                    Instruction::AddReg(register, value.chars().next().unwrap())
                }
            }
            "mul" => {
                let value = words.next().unwrap();
                if let Some(num) = value.parse::<isize>().ok() {
                    let register = register.chars().next().unwrap();
                    Instruction::MulImm(register, num)
                } else {
                    let register = register.parse().unwrap();
                    Instruction::MulReg(register, value.chars().next().unwrap())
                }
            }
            "mod" => {
                let value = words.next().unwrap();
                if let Some(num) = value.parse::<isize>().ok() {
                    let register = register.chars().next().unwrap();
                    Instruction::ModImm(register, num)
                } else {
                    let register = register.parse().unwrap();
                    Instruction::ModReg(register, value.chars().next().unwrap())
                }
            }
            "rcv" => {
                if let Some(num) = register.parse::<isize>().ok() {
                    Instruction::RcvImm(num)
                } else {
                    Instruction::RcvReg(register.chars().next().unwrap())
                }
            }
            "jgz" => {
                if let Some(check_num) = register.parse::<isize>().ok() {
                    if check_num > 0 {
                        let jump_num = words.next().unwrap().parse::<isize>().unwrap();
                        return Instruction::Jmp(jump_num);
                    } else {
                        return Instruction::Jmp(1);
                    }
                } else {
                    let value = words.next().unwrap();
                    if let Some(num) = value.parse::<isize>().ok() {
                        let register = register.chars().next().unwrap();
                        Instruction::JgzImm(register, num)
                    } else {
                        let register = register.parse().unwrap();
                        Instruction::JgzReg(register, value.chars().next().unwrap())
                    }
                }
            }
            _ => panic!("Unknown instruction"),
        }
    }
}
