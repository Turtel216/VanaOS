pub mod task;

use crate::task::{Task, TaskContext, TASKS};
use alloc::vec::Vec;

/// Create a new user task (simple bare-metal “function as process”)
pub fn spawn(entry: extern "C" fn() -> !) {
    const STACK_SIZE: usize = 4096 * 4;
    let mut stack: Vec<u8> = Vec::with_capacity(STACK_SIZE);
    let stack_top = stack.as_mut_ptr() as u64 + STACK_SIZE as u64;

    // Build initial context so that when we switch to it,
    // it “returns” into the function entry().
    let context = TaskContext {
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        rbx: 0,
        rbp: 0,
        rip: entry as u64,
    };

    let task = Task {
        context,
        stack_top,
        page_table: x86_64::registers::control::Cr3::read(),
    };
    TASKS.lock().push(task);
}
