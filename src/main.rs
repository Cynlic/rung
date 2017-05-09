// Rung: A Rust Nga VM

#![feature(tuple_indexing)]

use std::default::Default;
use std::fmt;

type CellInt = i32;

struct VM {
    sp:        usize,
    ip:        usize,
    rp:        usize,
    data:      [Cell; STACK_DEPTH],
    address:   [Cell; ADDRESSES],

    ports:     [Cell; PORTS],

    image:     Box<Image>,
    max_rsp:   u32,
    max_sp:    u32,
    filename:  String,
    request:   String,
}

impl VM {
    fn tos(&self) -> Cell {
        self.data[self.sp]
    }

    fn nos(&self) -> Cell {
        self.data[self.sp - 1]
    }

    fn tors(&self) -> Cell {
        self.address[self.rp]
    }

    //see Nga.md line 140
    fn load_image(path: &str) -> CellInt {}

    //516
    fn process_opcode(&self, opcode: VM_opcode) {} 
}

impl Default for VM {
    fn default() -> VM {
        VM {
            sp:        0,
            ip:        0,
            rp:        0,
            data:      [Cell(NOP); STACK_DEPTH],
            address:   [Cell(NOP); ADDRESSES],
            ports:     [Cell(NOP); PORTS],
            image:     Box::new(Image([Cell(NOP); IMAGE_SIZE])),
            max_rsp:   ADDRESSES as u32,
            max_sp:    STACK_DEPTH as u32,
            filename:  String::new(),
            request:   String::new(),

        }
    }
}

impl fmt::Display for VM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str("Stack: ");
        s.push_str(self.ip.to_string().as_str());
        write!(f,"{}",s)
    }

}

struct Image([Cell; IMAGE_SIZE]);

#[derive (Clone, Copy)]
struct Cell (VM_opcode);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{}", self.0)
    }
}

//Virtual Machine Parameters
const IMAGE_SIZE:          usize =  524288;
const ADDRESSES:           usize =  128;
const STACK_DEPTH:         usize =  32;

const PORTS:               usize =  12;
const MAX_FILE_NAME:       u32 =  1024;
const MAX_REQUEST_LENGTH:  u32 =  1024;
const MAX_OPEN_FILES:      u32 =  8;
const LOCAL:       &'static str = "retroImage" ;
const CELLSIZE:            u32 = 32;

//Nga VM Opcodes
#[derive (Clone, Copy)]
enum VM_opcode {
    NOP,
    LIT,
    DUP,
    DROP,
    SWAP,
    PUSH,
    POP,
    JUMP,
    CALL,
    CCALL,
    RETURN,
    EQ,
    NEQ,
    LT,
    GT,
    FETCH,
    STORE,
    ADD,
    SUB,
    MUL,
    DIVMOD,
    AND,
    OR,
    XOR,
    SHIFT,
    ZRET,
    END,
}

impl fmt::Display for VM_opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{}", self as i32)
    }
}

fn from_i32(n: i32) -> Option<VM_opcode> {
    if n >= NOP as i32 && n <= END as i32 {
        Some(unsafe {std::mem::transmute(n)})
    } else {
        None
    }
}

use VM_opcode::*;

const NUM_OPS: CellInt = (END as i32) + 1 ;

fn main() {
    let mut vm: VM = Default::default(); //syntax for overide { ..Default::default() };
 //   let (Cell(rsp), Cell(sp), Cell(ip)) = (vm.rp, vm.sp, vm.ip);
    println!("VM State: x: {} , y: {}, rp: {} ", vm.sp, vm.ip, vm.rp);
    println!("VM (Formatted) : {}", vm);
    let image = &vm.image.0;
    println!("Printed from the Image: {:08x}", image[5].0);

}
