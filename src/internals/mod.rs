use crate::{scheduler, JSResult};
use crate::fiber::{Effect, Fiber};
use crate::util::RcX;
use std::sync::LazyLock;


#[derive(Clone)]
pub(crate) struct Internals(
    RcX<internal::Internals>
);

const _: () = {
    impl std::ops::Deref for Internals {
        type Target = internal::Internals;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl std::ops::DerefMut for Internals {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Internals {
        pub(super) fn get() -> Self {
            static INTERNALS: LazyLock<RcX<internal::Internals>> =
                LazyLock::new(|| RcX::new(internal::Internals {
                    next_unit_of_work: None,
                    current_root:      None,
                    wip_root:          None,
                    deletions:         vec![],
                    wip_fiber:         None,
                    hook_index:        None,
                }));
            Self(INTERNALS.clone())
        }
    }
};

impl Internals {
    fn commit_root(mut self) {
        if self.wip_root.is_none() {
            return
        }

        fn commit_work(fiber: Option<Fiber>) {    
            use web_sys::wasm_bindgen::UnwrapThrowExt;

            let Some(fiber) = fiber else {return};

            let parent = fiber.parent().unwrap();
            let parent = parent.dom();
            match fiber.effect {
                None => (),
                Some(Effect::Create) => {
                    parent.append_child(fiber.dom()).unwrap_throw();
                }
                Some(Effect::Delete) => {
                    parent.remove_child(fiber.dom()).unwrap_throw();
                }
                Some(Effect::Update) => {
                    if let Some(old_props) = fiber.alternate.as_ref().expect_throw("`alternate` is unexpectedly None")
                        .vdom.props()
                    {
                        if let Some(attrs) = &old_props.attributes {
                            for (name, _) in &**attrs {
                                fiber.dom().set_attribute(name, "");
                            }
                        }
                        if let Some(handlers) = &old_props.eventhandlers {
                            for (event, _) in &**handlers {
                                fiber.dom().remove_event_listener_with_callback(
                                    event,
                                    listener
                                ).unwrap_throw();
                            }
                        }
                    }

                    if let Some(new_props) = fiber
                        .vdom.props()
                    {

                    }
                }
            }
            commit_work(fiber.child());
            commit_work(fiber.sibling());
        }

        self.deletions.iter().cloned().map(Some).for_each(commit_work);
        commit_work(self.wip_root.as_ref().unwrap().child());
        self.current_root = self.wip_root.clone();
        self.wip_root = None
    }

    pub(crate) fn flush_sync(self) -> JSResult<()> {
        scheduler::schedule_callback(
            Self::commit_root,
            Fiber::perform_unit_of_work,
            self
        )
    }
}

mod internal {
    use crate::fiber::Fiber;

    #[derive(Clone)]
    pub(crate) struct Internals {
        pub(crate) next_unit_of_work: Option<Fiber>,
        pub(crate) current_root:      Option<Fiber>,
        pub(crate) wip_root:          Option<Fiber>,
        pub(crate) deletions:         Vec<Fiber>,
        pub(crate) wip_fiber:         Option<(/* todo */)>,
        pub(crate) hook_index:        Option<(/* todo */)>,
    }
}
