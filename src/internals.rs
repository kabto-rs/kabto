use crate::{scheduler, JSResult};
use crate::fiber::{Fiber, FiberNode};


#[derive(Clone)]
pub(crate) struct Internals {
    pub(super) next_unit_of_work: Option<FiberNode>,
    pub(super) current_root:      Option<(/* todo */)>,
    pub(super) wip_rot:           Option<(/* todo */)>,
    pub(super) deletions:         Option<(/* todo */)>,
    pub(super) wip_fiber:         Option<(/* todo */)>,
    pub(super) hook_index:        Option<(/* todo */)>,
}

impl Internals {
    /// SAFETY: single thread
    pub(super) unsafe fn get() -> &'static mut Self {
        static mut INTERNALS: Internals = Internals {
            next_unit_of_work: None,
            current_root:      None,
            wip_rot:           None,
            deletions:         None,
            wip_fiber:         None,
            hook_index:        None,
        };

        #[allow(static_mut_refs)]
        &mut INTERNALS
    }
}

impl Internals {
    fn commit_root(&'static self) {
        // do nothing
    }

    pub(crate) fn flush_sync(&'static self) -> JSResult<()> {
        scheduler::schedule_callback(
            Self::commit_root,
            Fiber::perform_unit_of_work,
            self
        )
    }
}
