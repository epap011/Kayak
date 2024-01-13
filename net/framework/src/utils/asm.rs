use std::arch::asm;

#[inline]
pub fn cpuid() {
    unsafe {
	//llvm_asm!("movl $$0x2, $eax":::"eax");
        asm!("movl $$0x2, %eax", "eax");
        //llvm_asm!("movl $$0x0, %ecx":::"ecx");
	    asm!("movl $$0x0, %ecx", "ecx");
        //llvm_asm!("cpuid"
        //     :
        //     :
        //     : "rax rbx rcx rdx");
    	asm!("cpuid"
        );
    }
}

#[inline]
pub fn rdtsc_unsafe() -> u64 {
    unsafe {
        let low: u32;
        let high: u32;
        //llvm_asm!("rdtsc"
        //     : "={eax}" (low), "={edx}" (high)
        //     :
        //     : "rdx rax"
        //     : "volatile");
	    asm!("rdtsc"
             , out("eax") low, out("edx") high
             , options(nomem, nostack)
        );
        ((high as u64) << 32) | (low as u64)
    }
}

#[inline]
pub fn rdtscp_unsafe() -> u64 {
    let high: u32;
    let low: u32;
    unsafe {
        //llvm_asm!("rdtscp"
        //     : "={eax}" (low), "={edx}" (high)
        //     :
        //     : "ecx"
        //     : "volatile");
        asm!("rdtscp"
             , out("eax") low, out("edx") high
             , lateout("ecx")_
             , options(nomem, nostack)
        );
        ((high as u64) << 32) | (low as u64)
    }
}

#[inline]
pub fn pause() {
    unsafe {
        //llvm_asm!("pause"::::"volatile");
	    asm!("pause", options(nomem, nostack));
    }
}
