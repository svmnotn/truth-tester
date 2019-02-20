use truth_tester::test_xpr;

fn main() {
    const OP5: usize = 0;
    const OP4: usize = 1;
    const OP3: usize = 2;
    const OP2: usize = 3;
    const OP1: usize = 4;
    const OP0: usize = 5;
    const S3: usize = 6;
    const S2: usize = 7;
    const S1: usize = 8;
    const S0: usize = 9;

    println!(
        "Are they the same? {}",
        test_xpr(10, &|vars| 
            (   
                (!(vars[OP5] || vars[OP4] || vars[OP3] || vars[OP2] || vars[OP0] || vars[S3] || vars[S2] || vars[S1]) && vars[OP1] && vars[S0]) || 
                (!(vars[OP5] || vars[OP4] || vars[OP3] || vars[OP1] || vars[OP0] || vars[S3] || vars[S2] || vars[S1]) && vars[OP2] && vars[S0])
            )
                == 
            (
                !(vars[OP5] || vars[OP4] || vars[OP3] || vars[OP0] || vars[S3] || vars[S2] || vars[S1]) && vars[S0] && (vars[OP1] ^ vars[OP2])
            )
        )
    );
}
