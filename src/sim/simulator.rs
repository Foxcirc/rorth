
use crate::*;

pub(crate) type Stack<'a> = Vec<Value<'a>>;

pub(crate) struct Simulator<'a> {
    pub(crate) stack: Stack<'a>, // todo mabye use ´VecDeque` here, because of the rotl and rotr intrinsics
    env: Environment<'a>,
    isinit: bool,
}

impl<'a> Simulator<'a> {

    pub(crate) fn new() -> Self {
        Self {
            stack: Stack::new(),
            env: Environment::blank(),
            isinit: false,
        }
    }

    pub(crate) fn setup(&mut self, env: Environment<'a>) {
        self.env = env;
        self.isinit = true;
    }

    pub(crate) fn run(&mut self, main: Procedure<'a>) {

        use Instruction::*;

        if !self.isinit { fatal!("The simulation environment is not initialized") };

        let bcode = main.body;

        for op in bcode.ops {

            match op {

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

    pub(crate) fn pop(&mut self) -> Value<'a> {
        self.stack.pop().expect("Stack underflow while simulating.")
    }

    pub(crate) fn push<const S: usize>(&mut self, sname: &'a str, data: [u8; S]) {
        self.stack.push(Value::make(sname, &self.env.structs[sname], data));
    }

    pub(crate) fn modify<T: FromStack, R: ToStack, F: Routine<T, R>>(&mut self, routine: F) {
        routine.run(self);
    }

}
