mod io;

use vm::Vm;

pub fn load_module(vm: &mut Vm) {
    vm.load_native_function("print", io::print);
}