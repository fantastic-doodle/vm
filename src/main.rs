fn main() {
    // 예시: mov a, 1 (001 00 1 01 = 33) 실행
    vm(37); 
}

fn vm(code: u8) {
    // 1. 레지스터를 배열로 관리하면 인덱스로 접근하기 편합니다.
    // [0] = a, [1] = b, [2] = c, [3] = d
    let mut regs: [u8; 4] = [0, 0, 0, 0];

    // 2. 비트 추출 (Decoding)
    // specification.txt 구조: [op 3bit] [dst 2bit] [prefix 1bit] [src 2bit]
    let op     = (code >> 5) & 0b111; // 상위 3비트
    let dst    = ((code >> 3) & 0b11) as usize; // 그 다음 2비트
    let prefix = (code >> 2) & 0b1;   // 그 다음 1비트
    let src    = code & 0b11;       // 하위 2비트

    // 3. 소스 값 결정 (레지스터 값인지, 즉시값(imm)인지)
    let src_val = if prefix == 0 {
        regs[src as usize] // 레지스터인 경우 해당 레지스터의 값
    } else {
        src // 즉시값(imm)인 경우 비트 값 그대로 사용 (0~3)
    };

    // 4. 명령어(Operand) 실행
    match op {
        0b000 => { /* nop: 아무것도 안 함 */ },
        0b001 => { // mov
            regs[dst] = src_val;
        },
        0b010 => { // add
            regs[dst] = regs[dst].wrapping_add(src_val);
        },
        0b011 => { // sub
            regs[dst] = regs[dst].wrapping_sub(src_val);
        },
        0b100 => {
            if src_val == 0 {
                println!("Error: Division by zero!");
            }else {
                regs[dst] = regs[dst].wrapping_div(src_val);
            }
        },
        0b101 => {
            regs[dst] = regs[dst].wrapping_mul(src_val);
        },
        0b110 => {
            regs[dst] = !(regs[dst] & src_val);
        },
        _ => println!("unknown opcode"),
    }

    // 결과 출력
    println!("register:\na: {}, b: {}, c: {}, d: {}", regs[0], regs[1], regs[2], regs[3]);
}