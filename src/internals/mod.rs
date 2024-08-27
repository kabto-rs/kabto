use crate::{scheduler, JSResult};
use crate::fiber::Fiber;
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
                    wip_rot:           None,
                    deletions:         None,
                    wip_fiber:         None,
                    hook_index:        None,
                }));
            Self(INTERNALS.clone())
        }
    }
};

impl Internals {
    fn commit_root(self) {
        // do nothing
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
    use crate::{fiber::FiberNode, util::RcX};
    use std::sync::LazyLock;

    #[derive(Clone)]
    pub(crate) struct Internals {
        pub(crate) next_unit_of_work: Option<FiberNode>,
        pub(crate) current_root:      Option<(/* todo */)>,
        pub(crate) wip_rot:           Option<(/* todo */)>,
        pub(crate) deletions:         Option<(/* todo */)>,
        pub(crate) wip_fiber:         Option<(/* todo */)>,
        pub(crate) hook_index:        Option<(/* todo */)>,
    }
}
