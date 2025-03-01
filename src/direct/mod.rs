//! `NcDirect`

// total: 63
// ---------------------------------------------------
// (X)  1 : wont do
// (~)  3 : TODO / WIP
//
// (f) 47 : unsafe ffi function exported by bindgen
// (w)  1 : safely wrapped ffi function
// (r) 11 : static function manually reimplemented
//
// (m) 55 : method implemented
//
// (t)  0 : unit test done for the function
// (T)  0 : unit test done also for the method
// ---------------------------------------------------
// fm  ncdirect_bg_default
// fm  ncdirect_bg_palindex
// fm  ncdirect_bg_rgb
// fm  ncdirect_box
// rm  ncdirect_canbraille
// rm  ncdirect_canchangecolor
// fm  ncdirect_canget_cursor
// rm  ncdirect_canfade
// rm  ncdirect_canhalfblock
// fm  ncdirect_canopen_images
// rm  ncdirect_canopen_videos
// rm  ncdirect_canquadrant
// rm  ncdirect_cantruecolor
// fm  ncdirect_canutf8
// wm  ncdirect_capabilities
// fm  ncdirect_check_pixel_support
// fm  ncdirect_clear
//~f   ncdirect_core_init
// fm  ncdirect_cursor_disable
// fm  ncdirect_cursor_down
// fm  ncdirect_cursor_enable
// fm  ncdirect_cursor_left
// fm  ncdirect_cursor_move_yx
// fm  ncdirect_cursor_pop
// fm  ncdirect_cursor_push
// fm  ncdirect_cursor_right
// fm  ncdirect_cursor_up
// fm  ncdirect_cursor_yx
// fm  ncdirect_detected_terminal
// fm  ncdirect_dim_x
// fm  ncdirect_dim_y
// fm  ncdirect_double_box
// fm  ncdirect_fg_default
// fm  ncdirect_fg_palindex
// fm  ncdirect_fg_rgb
// fm  ncdirect_flush
// fm  ncdirect_get
//~r   ncdirect_heavy_box,
// fm  ncdirect_hline_interp
// fm  ncdirect_init
// fm  ncdirect_inputready_fd
//~r   ncdirect_light_box,
// fm  ncplane_on_styles
// fm  ncplane_off_styles
// fm  ncdirect_palette_size
//X    ncdirect_printf_aligned
// f   ncdirect_putegc
// fm  ncdirect_putstr
// fm  ncdirect_raster_frame
// fm  ncdirect_readline
// fm  ncdirect_render_frame
// fm  ncdirect_render_image
// fm  ncdirect_rounded_box
// fm  ncdirect_set_styles
// fm  ncdirect_stop
// f   ncdirect_stream
// f   ncdirect_styles
// f   ncdirect_supported_styles
// fm  ncdirect_vline_interp
// rm  ncdirect_bg_rgb8
// rm  ncdirect_fg_rgb8
// rm  ncdirect_getc_nblock
// rm  ncdirect_getc_nblocking

#[cfg(test)]
mod test;

mod methods;
pub(crate) mod reimplemented;

/// Minimal notcurses instance for styling text.
pub type NcDirect = crate::bindings::ffi::ncdirect;

/// Flags (options) for [`NcDirect`]
pub type NcDirectFlags = u64;

crate::impl_api![
    NcDirectFlags,
    NcDirectFlagsApi,
    /// Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    const DRAIN_INPUT: u64 = constants::NCDIRECT_OPTION_DRAIN_INPUT as u64;,
    /// Flag that avoids placing the terminal into cbreak mode
    /// (disabling echo and line buffering)
    ///
    const INHIBIT_CBREAK: NcDirectFlags =
        constants::NCDIRECT_OPTION_INHIBIT_CBREAK as NcDirectFlags;,
    /// Flag that avoids calling setlocale(LC_ALL, NULL)
    ///
    /// If the result is either "C" or "POSIX", it will print a
    /// diagnostic to stderr, and then call setlocale(LC_ALL, "").
    ///
    /// This will attempt to set the locale based off the LANG
    /// environment variable. Your program should call setlocale(3)
    /// itself, usually as one of the first lines.
    ///
    const INHIBIT_SETLOCALE: NcDirectFlags =
        constants::NCDIRECT_OPTION_INHIBIT_SETLOCALE as NcDirectFlags;,
    /// Flag that inhibits registration of the SIGINT, SIGSEGV, SIGABRT & SIGQUIT
    /// signal handlers.
    const NO_QUIT_SIGHANDLERS: NcDirectFlags =
        constants::NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS as NcDirectFlags;,
    /// Flag that enables showing detailed information.
    const VERBOSE: NcDirectFlags = constants::NCDIRECT_OPTION_VERBOSE as NcDirectFlags;,
    /// Flag that enables showing all diagnostics (equivalent to
    /// [`NcLogLevel::TRACE`][crate::NcLogLevel#associatedconstant.TRACE]).
    /// Implies [`NcDirectFlags::VERBOSE`][NcDirectFlags].
    const VERY_VERBOSE: NcDirectFlags = constants::NCDIRECT_OPTION_VERY_VERBOSE as NcDirectFlags;
];

pub(crate) mod constants {
    use crate::NcDirectFlags;

    /// Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    pub const NCDIRECT_OPTION_DRAIN_INPUT: u64 =
        crate::bindings::ffi::NCDIRECT_OPTION_DRAIN_INPUT as u64;

    /// Flag that avoids placing the terminal into cbreak mode
    /// (disabling echo and line buffering)
    ///
    pub const NCDIRECT_OPTION_INHIBIT_CBREAK: NcDirectFlags =
        crate::bindings::ffi::NCDIRECT_OPTION_INHIBIT_CBREAK as NcDirectFlags;

    /// Flag that avoids calling setlocale(LC_ALL, NULL)
    ///
    /// If the result is either "C" or "POSIX", it will print a
    /// diagnostic to stderr, and then call setlocale(LC_ALL, "").
    ///
    /// This will attempt to set the locale based off the LANG
    /// environment variable. Your program should call setlocale(3)
    /// itself, usually as one of the first lines.
    ///
    pub const NCDIRECT_OPTION_INHIBIT_SETLOCALE: NcDirectFlags =
        crate::bindings::ffi::NCDIRECT_OPTION_INHIBIT_SETLOCALE as NcDirectFlags;

    /// Flag that inhibits registration of the SIGINT, SIGSEGV, SIGABRT & SIGQUIT
    /// signal handlers.
    pub const NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS: NcDirectFlags =
        crate::bindings::ffi::NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS as NcDirectFlags;

    /// Flag that enables showing detailed information.
    pub const NCDIRECT_OPTION_VERBOSE: NcDirectFlags =
        crate::bindings::ffi::NCDIRECT_OPTION_VERBOSE as NcDirectFlags;

    /// Flag that enables showing all diagnostics (equivalent to
    /// [`NcLogLevel::TRACE`][crate::NcLogLevel#associatedconstant.TRACE]).
    /// Implies [`NCDIRECT_OPTION_VERBOSE`].
    pub const NCDIRECT_OPTION_VERY_VERBOSE: NcDirectFlags =
        crate::bindings::ffi::NCDIRECT_OPTION_VERY_VERBOSE as NcDirectFlags;
}
