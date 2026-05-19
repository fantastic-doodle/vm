mod vm;
fn main() {
    let mut vm = vm::VirtualMachine::new();
    println!("==========================vm==========================");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("error");
        let code: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a u8 number");
                return;
            }
        };
        if code == 255 {
            vm.execute(0);
            break;
        }else {
            vm.execute(code);   
        }
    }
}
//01 10 00 01