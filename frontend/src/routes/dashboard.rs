use dioxus::fermi::prelude::*;
use dioxus::prelude::*;

pub fn Page(cx: Scope) -> Element {
    let timetable = use_atom_state(&cx, crate::state::TIMETABLE);
    let update_rate = use_atom_state(&cx, crate::state::UPDATE_RATE);

    use_future(&cx, (update_rate), |(update_rate)| async move {
        loop {
            // timetable.set()
            tokio::time::sleep(*update_rate.get()).await;
        }
    });

    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "center",

            height: "calc(100vh - 16px)",

            div {}

            crate::components::timetable::Timetable {
                state: timetable.get().clone(),
            }

            div {}
        }
    )
}
