use crate::{
    c_api::{self, nctree_create},
    error, error_ref_mut,
    widgets::{NcTree, NcTreeItem, NcTreeOptions},
    NcError, NcInput, NcIntResult, NcIntResultApi, NcPlane, NcResult,
};

/// # `NcTree` constructors & destructors
impl NcTree {
    /// Creates an [NcTree] with the specified options.
    ///
    /// *C style function: [nctree_create()][c_api::nctree_create].*
    pub fn new<'a>(plane: &mut NcPlane, options: NcTreeOptions) -> NcResult<&'a mut Self> {
        error_ref_mut![unsafe { nctree_create(plane, &options) }, "Creating NcTree"]
    }

    /// Destroys an NcTree created with [new()][NcTree#method.new].
    ///
    /// *C style function: [nctree_destroy()][c_api::nctree_destroy].*
    pub fn destroy(&mut self) {
        unsafe { c_api::nctree_destroy(self) };
    }
}

/// # `NcTree` methods
impl NcTree {
    // NOTE: not implemented yet in C API
    //
    // /// Goes to the item specified by the array |spec|, terminated by UINT_MAX.
    // ///
    // /// If the spec is invalid, returns an error and the depth of the first
    // /// invalid spec is written to *|failspec|.
    // ///
    // /// Otherwise, the true depth is written to *|failspec|,
    // /// and the curry is returned (|failspec| is necessary because the
    // /// curry could itself be NULL).
    // ///
    // /// *C style function: [nctree_goto()][c_api::nctree_goto].*
    // pub fn goto(&mut self, spec: ... , failspec: ...) -> NcResult<&mut NcTreeItem> {
    //     let res = unsafe { c_api::nctree_goto(self) };
    //     if !res.is_null() {
    //         Ok(unsafe { &mut *(res as *mut NcTreeItem) })
    //     } else {
    //         Err(NcError::with_msg(NcIntResult::ERR, "NcTree.goto()"))
    //     }
    // }

    /// Returns the focused item, if any items are present.
    ///
    /// This is not a copy; be careful to use it only for the duration of a
    /// critical section.
    ///
    /// *C style function: [nctree_focused()][c_api::nctree_focused].*
    pub fn focused(&mut self) -> NcResult<&mut NcTreeItem> {
        let res = unsafe { c_api::nctree_focused(self) };
        if !res.is_null() {
            Ok(unsafe { &mut *(res as *mut NcTreeItem) })
        } else {
            Err(NcError::with_msg(NcIntResult::ERR, "NcTree.focused()"))
        }
    }

    /// Changes the focus to the next item, and returns it.
    ///
    /// *C style function: [nctree_next()][c_api::nctree_next].*
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> NcResult<&mut NcTreeItem> {
        let res = unsafe { c_api::nctree_next(self) };
        if !res.is_null() {
            Ok(unsafe { &mut *(res as *mut NcTreeItem) })
        } else {
            Err(NcError::with_msg(NcIntResult::ERR, "NcTree.next()"))
        }
    }

    /// Changes the focus to the previous item, and returns it.
    ///
    /// *C style function: [nctree_prev()][c_api::nctree_prev].*
    pub fn prev(&mut self) -> NcResult<&mut NcTreeItem> {
        let res = unsafe { c_api::nctree_prev(self) };
        if !res.is_null() {
            Ok(unsafe { &mut *(res as *mut NcTreeItem) })
        } else {
            Err(NcError::with_msg(NcIntResult::ERR, "NcTree.prev()"))
        }
    }

    /// Offers the `input` to this NcTree.
    ///
    /// If it's relevant, this function returns true,
    /// and the input ought not be processed further.
    /// If it's irrelevant to the tree, false is returned.
    ///
    /// Relevant inputs include:
    ///
    /// - a mouse click on an item (focuses item)
    /// - a mouse scrollwheel event (srolls tree)
    /// - up, down, pgup, or pgdown (navigates among items)
    ///
    /// *C style function: [nctree_offer_input()][c_api::nctree_offer_input].*
    pub fn offer_input(&mut self, input: NcInput) -> bool {
        unsafe { c_api::nctree_offer_input(self, &input) }
    }

    /// Returns the [NcPlane] backing this NcTree.
    ///
    /// *C style function: [nctree_plane()][c_api::nctree_plane].*
    pub fn plane(&mut self) -> NcResult<&NcPlane> {
        error_ref_mut![unsafe { c_api::nctree_plane(self) }, "NcTree.plane()"]
    }

    /// Redraws the NcTree in its entirety.
    ///
    /// The tree will be cleared, and items will be laid out, using the focused
    /// item as a fulcrum.
    ///
    /// Item-drawing callbacks will be invoked for each visible item.
    ///
    /// *C style function: [nctree_redraw()][c_api::nctree_redraw].*
    pub fn redraw(&mut self) -> NcResult<()> {
        error![unsafe { c_api::nctree_redraw(self) }, "NcTree.redraw()"]
    }
}
