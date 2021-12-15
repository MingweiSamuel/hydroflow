fn main() {
    type Pid = &'static str;
    type Name = &'static str;
    type Phone = &'static str;
    type DateTime = usize;

    build_hydroflow(|mut build_ctx| {
        let (loop_send, loop_recv) = build_ctx.make_handoff::<(Pid, DateTime)>();
        let (notifs_send, notifs_recv) = build_ctx.make_handoff::<(Pid, DateTime)>();

        let exposed = loop_recv
            .map(|(pid, t)| (pid, (t, t + TRANSMISSIBLE_DURATION)))
            .chain(build_ctx.make_input::<(Pid, (DateTime, DateTime))>("diagnosed_recv"));

        build_ctx
            .make_input::<(Pid, DateTime)>("contacts_recv")
            .flat_map(|(pid_a, pid_b, t)| [(pid_a, (pid_b, t)), (pid_b, (pid_a, t))])
            .join(exposed, &mut build_ctx)
            .filter(|(_pid_a, (pid_b, t_contact), (t_from, t_to))| (t_from..=t_to).contains(&t_contact))
            .map(|(_pid_a, pid_b_t_contact, _t_from_to)| pid_b_t_contact)
            .pusherator()
            .tee(loop_send)
            .push_to(notifs_send);

        notifs_recv
            .handoff(&mut build_ctx)
            .join(build_ctx.make_input("peoples_recv"))
            .pusherator()
            .for_each(|(_pid, (name, phone), exposure)| {
                println!(
                    "[{}] To {}: Possible Exposure at t = {}",
                    name, phone, exposure
                );
            });

        // Q: how does timely do loops
    });
}
