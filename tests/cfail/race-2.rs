extern crate cortex_m_rtfm as rtfm;

use rtfm::{C2, C4, P1, P3, Resource};

static R1: Resource<i32, C2> = Resource::new(0);
static R2: Resource<i32, C4> = Resource::new(0);

fn j1(prio: P1) {
    let ceil = prio.as_ceiling();

    ceil.raise(&R1, |ceil| {
        let r1 = R1.borrow(&prio, ceil);

        // Would preempt this critical section
        // rtfm::request(j2);
    });
}

fn j2(prio: P3) {
    let ceil = prio.as_ceiling();

    ceil.raise(&R2, |ceil| {
        // OK  C2 (R1's ceiling) <= C4 (system ceiling)
        // BAD C2 (R1's ceiling) <  P3 (j2's priority)
        let r1 = R1.borrow(&prio, ceil);
        //~^ error
    });
}
