//! `Nc`

// total: 53
// ---------------------------------------------------
// (X)  1 : wont do
// (…)  4 : TODO / WIP
//
// (f) 45 : unsafe ffi function exported by bindgen
// (w)  0 : safely wrapped ffi function
// (r)  6 : static function manually reimplemented
//
// (m) 38 : method implemented
//
// (t) 13 : unit test done for the function
// (T)  0 : unit test done also for the method
// ---------------------------------------------------
// fm  notcurses_at_yx
// fm  notcurses_bottom
// fm  notcurses_canbraille
// fmt notcurses_canchangecolor
// fmt notcurses_canfade
// fmt notcurses_canopen_images
// fmt notcurses_canopen_videos
// fmt notcurses_cansextant
// fmt notcurses_cantruecolor
// fmt notcurses_canutf8
// fm  notcurses_check_pixel_support
//~f   notcurses_core_init
// fm  notcurses_cursor_disable
// fm  notcurses_cursor_enable
// f   notcurses_cursor_yx
// fmt notcurses_debug
//~f   notcurses_detected_terminal
// fmt notcurses_drop_planes
// fm  notcurses_get
// fm  notcurses_getvec
// fmt notcurses_init
// fm  notcurses_inputready_fd
// fm  notcurses_lex_blitter
// fm  notcurses_lex_margins
// fm  notcurses_lex_scalemode
// fm  notcurses_linesigs_disable
// fm  notcurses_linesigs_enable
// fm  notcurses_mouse_disable
// fm  notcurses_mouse_enable
// fm  notcurses_palette_size
// fm  notcurses_refresh
// fm  notcurses_render
// fm  notcurses_render_to_buffer
// fm  notcurses_render_to_file
// fm  notcurses_stats
// fm  notcurses_stats_alloc
// fm  notcurses_stats_reset
// fm  notcurses_stdplane
// fm  notcurses_stdplane_const
// fmt notcurses_stop
// fm  notcurses_str_blitter
// fm  notcurses_str_scalemode
// fm  notcurses_supported_styles
// fm  notcurses_top
//X    notcurses_ucs32_to_utf8 (not needed in rust)
// fmt notcurses_version
// fm  notcurses_version_components
// rmt notcurses_align
// rm  notcurses_getc_blocking
// rm  notcurses_getc_nblock
//~r   notcurses_stddim_yx           // multiple mutable references errors
//~r   notcurses_stddim_yx_const     //
// rm  notcurses_term_dim_yx

mod methods;

pub(crate) mod helpers;
pub(crate) mod reimplemented;

#[cfg(test)]
mod test;

/// The full **notcurses** context.
///
/// It's built atop the terminfo abstraction layer to provide reasonably
/// portable vivid character displays.
pub type Nc = crate::bindings::ffi::notcurses;

/// Options struct for [`Nc`]
pub type NcOptions = crate::bindings::ffi::notcurses_options;

impl NcOptions {
    /// Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    pub const DRAIN_INPUT: u64 = constants::NCOPTION_DRAIN_INPUT as u64;

    /// Do not call setlocale()
    ///
    /// notcurses_init() will call setlocale() to inspect the current locale. If
    /// that locale is "C" or "POSIX", it will call setlocale(LC_ALL, "") to set
    /// the locale according to the LANG environment variable. Ideally, this will
    /// result in UTF8 being enabled, even if the client app didn't call
    /// setlocale() itself. Unless you're certain that you're invoking setlocale()
    /// prior to notcurses_init(), you should not set this bit. Even if you are
    /// invoking setlocale(), this behavior shouldn't be an issue unless you're
    /// doing something weird (setting a locale not based on LANG).
    pub const INHIBIT_SETLOCALE: u64 = constants::NCOPTION_INHIBIT_SETLOCALE as u64;

    /// Do not enter alternate mode.
    ///
    /// If smcup/rmcup capabilities are indicated, notcurses defaults to making use
    /// of the "alternate screen". This flag inhibits use of smcup/rmcup.
    pub const NO_ALTERNATE_SCREEN: u64 = constants::NCOPTION_NO_ALTERNATE_SCREEN as u64;

    /// Do not try to clear any preexisting bitmaps.
    ///
    /// Note that they might still get cleared even if this is set, and they might
    /// not get cleared even if this is not set.
    pub const NO_CLEAR_BITMAPS: u64 = constants::NCOPTION_NO_CLEAR_BITMAPS as u64;

    /// Do not modify the font.
    ///
    /// Notcurses might attempt to change the font slightly, to support certain
    /// glyphs (especially on the Linux console). If this is set, no such
    /// modifications will be made. Note that font changes will not affect anything
    /// but the virtual console/terminal in which notcurses is running.
    pub const NO_FONT_CHANGES: u64 = constants::NCOPTION_NO_FONT_CHANGES as u64;

    /// Do not handle SIG{ING, SEGV, ABRT, QUIT}.
    ///
    /// A signal handler will usually be installed for SIGINT, SIGQUIT, SIGSEGV,
    /// SIGTERM, and SIGABRT, cleaning up the terminal on such exceptions.
    /// With this flag, the handler will not be installed.
    pub const NO_QUIT_SIGHANDLERS: u64 = constants::NCOPTION_NO_QUIT_SIGHANDLERS as u64;

    /// Do not handle SIGWINCH.
    ///
    /// A signal handler will usually be installed for SIGWINCH, resulting in
    /// NCKEY_RESIZE events being generated on input.
    /// With this flag, the handler will not be installed.
    pub const NO_WINCH_SIGHANDLER: u64 = constants::NCOPTION_NO_WINCH_SIGHANDLER as u64;

    /// Initialize the standard plane's virtual cursor to match the physical cursor
    /// at context creation time.
    ///
    /// Together with
    /// [`NcOptions::NO_ALTERNATE_SCREEN`][NcOptions#associatedconstant.NO_ALTERNATE_SCREEN]
    /// and a scrolling standard plane,
    /// this facilitates easy scrolling-style programs in rendered mode.
    pub const PRESERVE_CURSOR: u64 = constants::NCOPTION_PRESERVE_CURSOR as u64;

    /// Do not print banners.
    ///
    /// Notcurses typically prints version info in notcurses_init() and performance
    /// info in notcurses_stop(). This inhibits that output.
    pub const SUPPRESS_BANNERS: u64 = constants::NCOPTION_SUPPRESS_BANNERS as u64;
}

pub(crate) mod constants {
    /// Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    pub const NCOPTION_DRAIN_INPUT: u64 = crate::bindings::ffi::NCOPTION_DRAIN_INPUT as u64;

    /// Do not call setlocale()
    ///
    /// notcurses_init() will call setlocale() to inspect the current locale. If
    /// that locale is "C" or "POSIX", it will call setlocale(LC_ALL, "") to set
    /// the locale according to the LANG environment variable. Ideally, this will
    /// result in UTF8 being enabled, even if the client app didn't call
    /// setlocale() itself. Unless you're certain that you're invoking setlocale()
    /// prior to notcurses_init(), you should not set this bit. Even if you are
    /// invoking setlocale(), this behavior shouldn't be an issue unless you're
    /// doing something weird (setting a locale not based on LANG).
    pub const NCOPTION_INHIBIT_SETLOCALE: u64 =
        crate::bindings::ffi::NCOPTION_INHIBIT_SETLOCALE as u64;

    /// Do not enter alternate mode.
    ///
    /// If smcup/rmcup capabilities are indicated, notcurses defaults to making use
    /// of the "alternate screen". This flag inhibits use of smcup/rmcup.
    pub const NCOPTION_NO_ALTERNATE_SCREEN: u64 =
        crate::bindings::ffi::NCOPTION_NO_ALTERNATE_SCREEN as u64;

    /// Do not try to clear any preexisting bitmaps.
    ///
    /// Note that they might still get cleared even if this is set, and they might
    /// not get cleared even if this is not set.
    pub const NCOPTION_NO_CLEAR_BITMAPS: u64 =
        crate::bindings::ffi::NCOPTION_NO_CLEAR_BITMAPS as u64;

    /// Do not modify the font.
    ///
    /// Notcurses might attempt to change the font slightly, to support certain
    /// glyphs (especially on the Linux console). If this is set, no such
    /// modifications will be made. Note that font changes will not affect anything
    /// but the virtual console/terminal in which notcurses is running.
    pub const NCOPTION_NO_FONT_CHANGES: u64 = crate::bindings::ffi::NCOPTION_NO_FONT_CHANGES as u64;

    /// Do not handle SIG{ING, SEGV, ABRT, QUIT}.
    ///
    /// A signal handler will usually be installed for SIGINT, SIGQUIT, SIGSEGV,
    /// SIGTERM, and SIGABRT, cleaning up the terminal on such exceptions.
    /// With this flag, the handler will not be installed.
    pub const NCOPTION_NO_QUIT_SIGHANDLERS: u64 =
        crate::bindings::ffi::NCOPTION_NO_QUIT_SIGHANDLERS as u64;

    /// Do not handle SIGWINCH.
    ///
    /// A signal handler will usually be installed for SIGWINCH, resulting in
    /// NCKEY_RESIZE events being generated on input.
    /// With this flag, the handler will not be installed.
    pub const NCOPTION_NO_WINCH_SIGHANDLER: u64 =
        crate::bindings::ffi::NCOPTION_NO_WINCH_SIGHANDLER as u64;

    /// Initialize the standard plane's virtual cursor to match the physical cursor
    /// at context creation time.
    ///
    /// Together with [`NCOPTION_NO_ALTERNATE_SCREEN`] and a scrolling standard plane,
    /// this facilitates easy scrolling-style programs in rendered mode.
    pub const NCOPTION_PRESERVE_CURSOR: u64 = crate::bindings::ffi::NCOPTION_PRESERVE_CURSOR as u64;

    /// Do not print banners.
    ///
    /// Notcurses typically prints version info in notcurses_init() and performance
    /// info in notcurses_stop(). This inhibits that output.
    pub const NCOPTION_SUPPRESS_BANNERS: u64 =
        crate::bindings::ffi::NCOPTION_SUPPRESS_BANNERS as u64;
}
