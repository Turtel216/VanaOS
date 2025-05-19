use core::arch::asm;
use x86_64::registers::control::Cr3;
use x86_64::structures::idt::InterruptStackFrame;

// Saved CPU state for a context-switchable task
#[repr(C)]
pub struct TaskContext {
    // General-purpose registers (callee-saved first, per SysV ABI)
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    // Instruction pointer to resume execution
    rip: u64,
}

// Simplest task descriptor: just context + stack + page table
pub struct Task {
    pub context: TaskContext,
    pub stack_top: u64,
    pub page_table: Cr3,
}

// The global task list and current index
use spin::Mutex;
static TASKS: Mutex<Vec<Task>> = Mutex::new(Vec::new());
static mut CURRENT: usize = 0;

/// Performs the low-level switch.
/// Saves callee-saved regs + RIP into `old`, then loads from `new`.
#[naked]
pub unsafe extern "C" fn switch_context(old: *mut TaskContext, new: *const TaskContext) {
    asm!(
        // Save callee-saved registers onto the *old* context struct
        "mov [rdi + 0x00], r15",
        "mov [rdi + 0x08], r14",
        "mov [rdi + 0x10], r13",
        "mov [rdi + 0x18], r12",
        "mov [rdi + 0x20], rbx",
        "mov [rdi + 0x28], rbp",
        // Store the return RIP (address after the call to `switch_context`)
        "lea rax, [rip + 0f]",
        "mov [rdi + 0x30], rax",
        // Load new context
        "mov r15, [rsi + 0x00]",
        "mov r14, [rsi + 0x08]",
        "mov r13, [rsi + 0x10]",
        "mov r12, [rsi + 0x18]",
        "mov rbx, [rsi + 0x20]",
        "mov rbp, [rsi + 0x28]",
        "mov rax, [rsi + 0x30]",
        // Jump to new RIP
        "jmp rax",
        "0:",
        "ret",
        options(noreturn)
    )
}
