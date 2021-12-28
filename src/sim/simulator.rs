
use crate::coder::*;
use crate::sim::*;

pub(crate) type Stack = Vec<Value>;

pub(crate) struct Simulator {
    pub(crate) stack: Stack, // todo mabye use ´VecDeque` here, because of the rotl and rotr intrinsics
    consts: Constants,
    procs: Procedures,
    structs: Structures,
}

impl Simulator {

    pub(crate) fn new() -> Self {
        Self {
            stack: Stack::new(),
            consts: Constants::new(),
            procs: Procedures::new(),
            structs: Structures::new()
        }
    }

    pub(crate) fn setup(&mut self, consts: Constants, procs: Procedures, structs: Structures) {
        self.consts = consts;
        self.procs = procs;
        self.structs = structs;
    }

    pub(crate) fn run(&mut self, bcode: Bytecode) {

        use Instruction::*;

        for instr in bcode.instrs {

            match instr {

                Add => self.modify(|(b, a)| b + a),
                Subtract => self.modify(|(b, a)| a - b), // the order here is reversed, so it is not as confusing
                Multiply => self.modify(|(b, a)| b * a),
                Divide => self.modify(|(b, a)| a / b),

                Dup => self.modify(|a: u64| (a, a)),
                Swap => self.modify(|(b, a)| (b, a)),
                Over => self.modify(|(b, a)| (a, b, a)),
                Drop => self.modify(|_: u64| ()),
                RotLeft => {
                    let elem = self.stack.remove(0);
                    self.stack.push(elem);
                },
                RotRight => {
                    let elem = self.pop();
                    self.stack.insert(0, elem);
                },

                Push(value) => {
                    self.stack.push(value);
                }

                // other => todo!("This instruction is not implemented yet: {:?}", other),

            }

        }

    }

    pub(crate) fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow while simulating.")
    }

    pub(crate) fn push<const S: usize>(&mut self, id: u32, data: [u8; S]) {
        self.stack.push(Value::make(id, &self.structs[id as usize], data));
    }

    pub(crate) fn modify<T: FromStack, R: ToStack, F: Routine<T, R>>(&mut self, routine: F) {
        routine.run(self);
    }

}
