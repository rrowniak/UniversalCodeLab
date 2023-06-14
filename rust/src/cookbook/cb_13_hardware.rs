pub fn main() {
    cpu::main();
}

mod cpu {
    pub fn main() {
        println!("Number of logical cores is {}", num_cpus::get());
    }
}
