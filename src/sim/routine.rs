
use crate::sim::*;

pub(crate) trait Routine<T: FromStack, R: ToStack> {
    fn run(self, sim: &mut Simulator);
}

impl<T: FromStack, R: ToStack, F: FnOnce(T) -> R> Routine<T, R> for F {
    fn run(self, sim: &mut Simulator) {
        let args = T::take(sim);        
        let result = self(args);
        R::push(result, sim);
    }
}

pub(crate) trait FromStack {
    fn take(sim: &mut Simulator) -> Self;
}

pub(crate) trait ToStack {
    fn push(value: Self, sim: &mut Simulator);
}

impl ToStack for () { fn push(_: Self, _: &mut Simulator) {} }
impl ToStack for u64 { fn push(value: Self, sim: &mut Simulator) { sim.push("int", value.to_ne_bytes()); } }
impl ToStack for (u64, u64) { fn push(value: Self, sim: &mut Simulator) { sim.push("int", value.0.to_ne_bytes()); sim.push("int", value.1.to_ne_bytes()); } }
impl ToStack for (u64, u64, u64) { fn push(value: Self, sim: &mut Simulator) { sim.push("int", value.0.to_ne_bytes()); sim.push("int", value.1.to_ne_bytes()); sim.push("int", value.2.to_ne_bytes()); } }
impl ToStack for (u64, u64, u64, u64) { fn push(value: Self, sim: &mut Simulator) { sim.push("int", value.0.to_ne_bytes()); sim.push("int", value.1.to_ne_bytes()); sim.push("int", value.2.to_ne_bytes()); sim.push("int", value.3.to_ne_bytes()); } }

impl FromStack for () { fn take(_: &mut Simulator) {} }
impl FromStack for u64 { fn take(sim: &mut Simulator) -> Self { sim.pop().view() } }

macro_rules! implfrsk { ($(($($t:ident,)*)),*) => { $( impl FromStack for ($($t, )*) { fn take(sim: &mut Simulator) -> Self { ($(sim.pop().view() as $t, )*) } } )* }; }

implfrsk!((u64, u64, ), (u64, u64, u64, ), (u64, u64, u64, u64, ));
