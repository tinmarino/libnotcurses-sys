//! `Nc*` methods and associated functions.

use core::ptr::{null, null_mut};

use crate::{
    c_api::{self, notcurses_init},
    cstring, error, error_ref_mut, rstring, rstring_free, Nc, NcAlign, NcBlitter, NcChannels,
    NcDim, NcError, NcFile, NcInput, NcLogLevel, NcOptions, NcPixelImpl, NcPlane, NcResult,
    NcScale, NcStats, NcStyle, NcStyleApi, NcTime,
};

/// # `NcOptions` Constructors
impl NcOptions {
    /// New `NcOptions`.
    pub const fn new() -> Self {
        Self::with_all_options(0, 0, 0, 0, 0, 0)
    }

    /// New `NcOptions`, with margins.
    pub const fn with_margins(top: NcDim, right: NcDim, bottom: NcDim, left: NcDim) -> Self {
        Self::with_all_options(0, top, right, bottom, left, 0)
    }

    /// New `NcOptions`, with flags.
    pub const fn with_flags(flags: u64) -> Self {
        Self::with_all_options(0, 0, 0, 0, 0, flags)
    }

    /// New `NcOptions`, with all the options.
    ///
    /// ## Arguments
    ///
    /// - loglevel
    ///
    ///   Progressively higher log levels result in more logging to stderr. By
    ///   default, nothing is printed to stderr once fullscreen service begins.
    ///
    /// - margin_t, margin_r, margin_b, margin_l
    ///
    ///   Desirable margins (top, right, bottom, left).
    ///
    ///   If all are 0 (default), we will render to the entirety of the screen.
    ///   If the screen is too small, we do what we can.
    ///   Absolute coordinates are relative to the rendering area
    ///   ((0, 0) is always the origin of the rendering area).
    ///
    /// - flags
    ///
    ///   General flags; This is expressed as a bitfield so that future options
    ///   can be added without reshaping the struct.
    ///   Undefined bits must be set to 0.
    ///
    ///   - [`NcOptions::INHIBIT_SETLOCALE`][crate::NcOptions::INHIBIT_SETLOCALE]
    ///   - [`NcOptions::NO_ALTERNATE_SCREEN`]
    ///   - [`NcOptions::NO_FONT_CHANGES`][crate::NcOptions::NO_FONT_CHANGES]
    ///   - [`NcOptions::NO_QUIT_SIGHANDLERS`][crate::NcOptions::NO_QUIT_SIGHANDLERS]
    ///   - [`NcOptions::NO_WINCH_SIGHANDLER`][crate::NcOptions::NO_WINCH_SIGHANDLER]
    ///   - [`NcOptions::SUPPRESS_BANNERS`]
    ///
    pub const fn with_all_options(
        loglevel: NcLogLevel,
        margin_t: NcDim,
        margin_r: NcDim,
        margin_b: NcDim,
        margin_l: NcDim,
        flags: u64,
    ) -> Self {
        Self {
            termtype: null(),
            loglevel,
            renderfp: null_mut(),
            margin_t: margin_t as i32,
            margin_r: margin_r as i32,
            margin_b: margin_b as i32,
            margin_l: margin_l as i32,
            flags,
        }
    }
}

/// # `Nc` Constructors
//
// TODO: rethink constructors
impl Nc {
    /// New notcurses context (without banners).
    pub fn new<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(NcOptions::SUPPRESS_BANNERS)
    }

    /// New notcurses context in CLI mode.
    ///
    /// It has the following flags:
    /// - [`NcOptions::SUPPRESS_BANNERS`]
    /// - [`NcOptions::NO_ALTERNATE_SCREEN`]
    /// - [`NcOptions::NO_CLEAR_BITMAPS`]
    /// - [`NcOptions::PRESERVE_CURSOR`]
    pub fn new_cli<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(
            NcOptions::SUPPRESS_BANNERS
                | NcOptions::NO_ALTERNATE_SCREEN
                | NcOptions::NO_CLEAR_BITMAPS
                | NcOptions::PRESERVE_CURSOR,
        )
    }

    /// New notcurses context, with banners.
    ///
    /// This is the default in the C library.
    pub fn with_banners<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(0)
    }

    /// New notcurses context, without an alternate screen (nor banners).
    #[deprecated]
    #[doc(hidden)]
    pub fn without_altscreen<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(NcOptions::NO_ALTERNATE_SCREEN | NcOptions::SUPPRESS_BANNERS)
    }

    /// New notcurses context, expects `NcOptions::*` flags.
    pub fn with_flags<'a>(flags: u64) -> NcResult<&'a mut Nc> {
        Self::with_options(NcOptions::with_flags(flags))
    }

    /// New notcurses context, expects [NcOptions].
    pub fn with_options<'a>(options: NcOptions) -> NcResult<&'a mut Nc> {
        let res = unsafe { notcurses_init(&options, null_mut()) };
        error_ref_mut![res, "Nc.with_options()"]
    }

    /// New notcurses context, expects [NcLogLevel] and flags.
    pub fn with_debug<'a>(loglevel: NcLogLevel, flags: u64) -> NcResult<&'a mut Nc> {
        Self::with_options(NcOptions::with_all_options(loglevel, 0, 0, 0, 0, flags))
    }
}

/// # `Nc` methods
impl Nc {
    /// Returns the offset into `availcols` at which `cols` ought be output given
    /// the requirements of `align`.
    ///
    /// Returns `-`[`NcIntResult::MAX`][crate::NcIntResult::MAX] if
    /// [NcAlign::UNALIGNED][NcAlign#associatedconstant.UNALIGNED]
    /// or invalid [`NcAlign`].
    ///
    /// *C style function: [notcurses_align()][c_api::notcurses_align].*
    //
    // TODO: handle error rightfully.
    pub fn align(availcols: NcDim, align: NcAlign, cols: NcDim) -> NcResult<()> {
        error![c_api::notcurses_align(availcols, align, cols)]
    }

    /// Retrieves the current contents of the specified [NcCell][crate::NcCell]
    /// as last rendered, returning the `EGC` (or None on error) and writing
    /// out the [`NcStyle`] and the [`NcChannels`].
    ///
    /// *C style function: [notcurses_at_yx()][c_api::notcurses_at_yx].*
    pub fn at_yx(
        &mut self,
        y: NcDim,
        x: NcDim,
        stylemask: &mut NcStyle,
        channels: &mut NcChannels,
    ) -> Option<String> {
        let egc = unsafe { c_api::notcurses_at_yx(self, x as i32, y as i32, stylemask, channels) };
        if egc.is_null() {
            return None;
        }
        Some(rstring_free![egc])
    }

    /// Returns the bottommost [`NcPlane`] on the standard pile,
    /// of which there is always at least one.
    ///
    /// *C style function: [notcurses_bottom()][c_api::notcurses_bottom].*
    pub fn bottom(&mut self) -> &mut NcPlane {
        unsafe { &mut *c_api::notcurses_bottom(self) }
    }

    /// Returns true if we can reliably use Unicode Braille.
    ///
    /// See also [`NcBlitter::BRAILLE`][NcBlitter#BRAILLE].
    ///
    /// *C style function: [notcurses_canbraille()][c_api::notcurses_canbraille].*
    pub fn canbraille(&self) -> bool {
        unsafe { c_api::notcurses_canbraille(self) }
    }

    /// Returns true if it's possible to set the "hardware" palette.
    ///
    /// Requires the "ccc" terminfo capability.
    ///
    /// *C style function: [notcurses_canchangecolor()][c_api::notcurses_canchangecolor].*
    pub fn canchangecolor(&self) -> bool {
        unsafe { c_api::notcurses_canchangecolor(self) }
    }

    /// Returns true if fading is possible.
    ///
    /// Fading requires either the "rgb" or "ccc" terminfo capability.
    ///
    /// *C style function: [notcurses_canfade()][c_api::notcurses_canfade].*
    pub fn canfade(&self) -> bool {
        unsafe { c_api::notcurses_canfade(self) }
    }

    /// Returns true if we can reliably use Unicode half blocks.
    ///
    /// See also [`Blitter::BLIT_2x1`][NcBlitter#associatedconstant.BLIT_2x1].
    ///
    /// *C style function: [notcurses_canhalfblock()][c_api::notcurses_canhalfblock].*
    pub fn canhalfblock(&self) -> bool {
        unsafe { c_api::notcurses_canhalfblock(self) }
    }

    /// Returns true if loading images is possible.
    ///
    /// This requires being built against FFmpeg/OIIO.
    ///
    /// *C style function: [notcurses_canopen_images()][c_api::notcurses_canopen_images].*
    pub fn canopen_images(&self) -> bool {
        unsafe { c_api::notcurses_canopen_images(self) }
    }

    /// Returns true if loading videos is possible.
    ///
    /// This requires being built against FFmpeg.
    ///
    /// *C style function: [notcurses_canopen_videos()][c_api::notcurses_canopen_videos].*
    pub fn canopen_videos(&self) -> bool {
        unsafe { c_api::notcurses_canopen_videos(self) }
    }

    /// Returns true if we can reliably use Unicode quadrant blocks.
    ///
    /// See also [`NcBlitter::BLIT_2x2`][NcBlitter#associatedconstant.NCBLIT_2x2].
    ///
    /// *C style function: [notcurses_canquadrant()][c_api::notcurses_canquadrant].*
    pub fn canquadrant(&self) -> bool {
        unsafe { c_api::notcurses_canquadrant(self) }
    }

    /// Returns true if we can reliably use Unicode 13 sextants.
    ///
    /// See also [`NcBlitter::BLIT_3x2`][NcBlitter#associatedconstant.NCBLIT_3x2].
    ///
    /// *C style function: [notcurses_cansextant()][c_api::notcurses_cansextant].*
    pub fn cansextant(&self) -> bool {
        unsafe { c_api::notcurses_cansextant(self) }
    }

    /// Returns true if it's possible to directly specify RGB values per cell,
    /// or false if it's only possible to use palettes.
    ///
    /// *C style function: [notcurses_cantruecolor()][c_api::notcurses_cantruecolor].*
    pub fn cantruecolor(&self) -> bool {
        unsafe { c_api::notcurses_cantruecolor(self) }
    }

    /// Returns true if the encoding is UTF-8.
    ///
    /// Requires `LANG` being set to a UTF-8 locale.
    ///
    /// *C style function: [notcurses_canutf8()][c_api::notcurses_canutf8].*
    pub fn canutf8(&self) -> bool {
        unsafe { c_api::notcurses_canutf8(self) }
    }

    /// Checks for pixel support.
    ///
    /// Returns [`NcPixelImpl`] with a non-zero constant corresponding to some
    /// pixel-blitting mechanism if bitmap support (via any mechanism) has been
    /// detected, or else 0 (NCPIXEL_NONE).
    ///
    /// *C style function: [notcurses_check_pixel_support()][c_api::notcurses_check-pixel_support].*
    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn check_pixel_support(&self) -> NcPixelImpl {
        unsafe { c_api::notcurses_check_pixel_support(self) }
    }

    /// Disables the terminal's cursor, if supported.
    ///
    /// Immediate effect (no need for a call to notcurses_render()).
    ///
    /// *C style function: [notcurses_cursor_disable()][c_api::notcurses_cursor_disable].*
    pub fn cursor_disable(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_cursor_disable(self) }]
    }

    /// Enables the terminal's cursor, if supported, placing it at `y`, `x`.
    ///
    /// Immediate effect (no need for a call to notcurses_render()).
    /// It is an error if `y`, `x` lies outside the standard plane.
    ///
    /// *C style function: [notcurses_cursor_enable()][c_api::notcurses_cursor_enable].*
    pub fn cursor_enable(&mut self, y: NcDim, x: NcDim) -> NcResult<()> {
        error![unsafe { c_api::notcurses_cursor_enable(self, y as i32, x as i32) }]
    }

    /// Shifts to the alternate screen, if available.
    ///
    /// If already using the alternate screen, this returns Ok(()) immediately.
    ///
    /// If the alternate screen is not available, returns an Error immediately.
    ///
    /// Entering the alternate screen turns off scrolling for the standard plane.
    ///
    /// *C style function:
    /// [notcurses_enter_alternate_screen()][c_api::notcurses_enter_alternate_screen].*
    pub fn enter_alternate_screen(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_enter_alternate_screen(self) }]
    }

    /// Exits the alternate screen.
    ///
    /// Immediately returns Ok(()) if not currently using the alternate screen.
    ///
    /// *C style function:
    /// [notcurses_leave_alternate_screen()][c_api::notcurses_leave_alternate_screen].*
    pub fn leave_alternate_screen(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_leave_alternate_screen(self) }]
    }

    /// Dumps notcurses state to the supplied `debugfp`.
    ///
    /// Output is freeform, and subject to change. It includes geometry of all
    /// planes, from all piles.
    ///
    /// *C style function: [notcurses_debug()][c_api::notcurses_debug].*
    pub fn debug(&mut self, debugfp: &mut NcFile) {
        unsafe {
            c_api::notcurses_debug(self, debugfp.as_nc_ptr());
        }
    }

    /// Returns the name of the user under which we are running.
    ///
    /// *C style function: [notcurses_accountname()][c_api::notcurses_accountname].*
    pub fn accountname() -> String {
        rstring_free![c_api::notcurses_accountname()]
    }

    /// Returns the name of the local hostname.
    ///
    /// *C style function: [notcurses_hostname()][c_api::notcurses_hostname].*
    pub fn hostname() -> String {
        rstring_free![c_api::notcurses_hostname()]
    }

    /// Returns the name of the detected terminal.
    ///
    /// *C style function: [notcurses_detected_terminal()][c_api::notcurses_detected_terminal].*
    pub fn detected_terminal(&self) -> String {
        rstring_free![c_api::notcurses_detected_terminal(self)]
    }

    /// Destroys all [`NcPlane`]s other than the stdplane.
    ///
    /// *C style function: [notcurses_drop_planes()][c_api::notcurses_drop_planes].*
    pub fn drop_planes(&mut self) {
        unsafe {
            c_api::notcurses_drop_planes(self);
        }
    }

    #[doc(hidden)]
    #[deprecated = "use `get` method instead"]
    pub fn getc(&mut self, time: Option<NcTime>, input: Option<&mut NcInput>) -> NcResult<char> {
        self.get(time, input)
    }

    /// Returns a [char] representing a single unicode point.
    ///
    /// If an event is processed, the return value is the `id` field from that
    /// event.
    ///
    /// Provide a None `time` to block at length, a `time` of 0 for non-blocking
    /// operation, and otherwise a timespec to bound blocking.
    ///
    /// *C style function: [notcurses_get()][c_api::notcurses_get].*
    pub fn get(&mut self, time: Option<NcTime>, input: Option<&mut NcInput>) -> NcResult<char> {
        let ntime;
        if let Some(time) = time {
            ntime = &time as *const _;
        } else {
            ntime = null();
        }

        let ninput;
        if let Some(input) = input {
            ninput = input as *mut _;
        } else {
            ninput = null_mut();
        }

        let res = unsafe { c_api::notcurses_get(self, ntime, ninput) };
        core::char::from_u32(res)
            .ok_or_else(|| NcError::with_msg(res as i32, &format!["Nc.get(time: {:?})", time]))
    }

    /// Acquire up to 'vcount' [`NcInput`]s at the vector 'ni'.
    ///
    /// The number read will be returned, or 0 on timeout.
    ///
    /// *C style function: [notcurses_getvec()][c_api::notcurses_getvec].*
    pub fn getvec(
        &mut self,
        time: Option<NcTime>,
        ni: &mut Vec<NcInput>,
        vcount: u32,
    ) -> NcResult<u32> {
        let ntime;
        if let Some(time) = time {
            ntime = &time as *const _;
        } else {
            ntime = null();
        }
        let nivec = ni.as_mut_ptr() as *mut NcInput;

        let res = unsafe { c_api::notcurses_getvec(self, ntime, nivec, vcount as i32) };
        error![res, "", res as u32]
    }

    /// Reads input blocking until an event is processed or a signal is received.
    ///
    /// Will optionally write the event details in `input`.
    ///
    /// In the case of a valid read, a [`char`] is returned.
    ///
    /// *C style function: [notcurses_getc_blocking()][c_api::notcurses_getc_blocking].*
    pub fn getc_blocking(&mut self, input: Option<&mut NcInput>) -> NcResult<char> {
        let res = c_api::notcurses_getc_blocking(self, input);
        core::char::from_u32(res as u32).ok_or_else(|| NcError::with_msg(res, "Nc.getc_blocking()"))
    }

    /// Reads input without blocking.
    ///
    /// In the case of a valid read, a [`char`] is returned.
    ///
    /// If no event is ready, returns 0.
    ///
    /// *C style function: [notcurses_getc_nblock()][c_api::notcurses_getc_nblock].*
    pub fn getc_nblock(&mut self, input: Option<&mut NcInput>) -> NcResult<char> {
        let res = c_api::notcurses_getc_nblock(self, input);
        core::char::from_u32(res as u32).ok_or_else(|| NcError::with_msg(res, "Nc.getc_nblock()"))
    }

    /// Gets a file descriptor suitable for input event poll()ing.
    ///
    /// When this descriptor becomes available, you can call
    /// [getc_nblock()][Nc#method.getc_nblock], and input ought be ready.
    ///
    /// This file descriptor is not necessarily the file descriptor associated
    /// with stdin (but it might be!).
    ///
    /// *C style function: [notcurses_inputready_fd()][c_api::notcurses_inputready_fd].*
    pub fn inputready_fd(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_inputready_fd(self) }]
    }

    /// Returns an [`NcBlitter`] from a string representation.
    ///
    /// *C style function: [notcurses_lex_blitter()][c_api::notcurses_lex_blitter].*
    pub fn lex_blitter(blitter_str: &str) -> NcResult<NcBlitter> {
        let mut blitter = 0;
        error![
            unsafe { c_api::notcurses_lex_blitter(cstring![blitter_str], &mut blitter) },
            "Invalid blitter name", blitter
        ]
    }

    /// Lexes a margin argument according to the standard notcurses definition.
    ///
    /// There can be either a single number, which will define all margins equally,
    /// or there can be four numbers separated by commas.
    ///
    /// *C style function: [notcurses_lex_margins()][c_api::notcurses_lex_margins].*
    pub fn lex_margins(margins_str: &str, options: &mut NcOptions) -> NcResult<()> {
        error![unsafe { c_api::notcurses_lex_margins(cstring![margins_str], options) }]
    }

    /// Returns an [`NcScale`] from a string representation.
    ///
    /// *C style function: [notcurses_lex_scalemode()][c_api::notcurses_lex_scalemode].*
    pub fn lex_scalemode(scalemode_str: &str) -> NcResult<NcScale> {
        let mut scalemode = 0;
        error![
            unsafe { c_api::notcurses_lex_scalemode(cstring![scalemode_str], &mut scalemode) },
            "", scalemode
        ]
    }

    /// Returns an [`NcStyle`] from a string representation.
    ///
    /// It is case-insensitive, and supports multiple styles separated by
    /// spaces.
    ///
    /// The supported styles are: `italic`, `underline`, `undercurl`,
    /// `struck`, `bold`, and `none`.
    ///
    /// If a style is are not recognized returns an error.
    ///
    /// *(No equivalent C style function)*
    pub fn lex_styles(styles_str: &str) -> NcResult<NcStyle> {
        let mut style = NcStyle::NOSTYLE;
        let mut errstr = String::new();

        for s in styles_str.split(' ') {
            match s.to_lowercase().as_str() {
                "italic" => style.add(NcStyle::ITALIC),
                "underline" => style.add(NcStyle::UNDERLINE),
                "undercurl" => style.add(NcStyle::UNDERCURL),
                "struck" => style.add(NcStyle::STRUCK),
                "bold" => style.add(NcStyle::BOLD),
                "none" => (),
                _ => {
                    errstr.push_str(s);
                    errstr.push(' ');
                }
            }
        }
        if errstr.is_empty() {
            Ok(style)
        } else {
            let _ = errstr.pop();
            Err(NcError::new_msg(&format![
                "the following styles are not recognized: '{}'",
                errstr
            ]))
        }
    }

    /// Disables signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z). They are enabled by default.
    ///
    /// *C style function: [notcurses_linesigs_disable()][c_api::notcurses_linesigs_disable].*
    pub fn linesigs_disable(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_linesigs_disable(self) }]
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    ///
    /// *C style function: [notcurses_linesigs_enable()][c_api::notcurses_linesigs_enable].*
    pub fn linesigs_enable(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_linesigs_enable(self) }]
    }

    /// Disables mouse events.
    ///
    /// Any events in the input queue can still be delivered.
    ///
    /// *C style function: [notcurses_mouse_disable()][c_api::notcurses_mouse_disable].*
    pub fn mouse_disable(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_mouse_disable(self) }]
    }

    /// Enable the mouse in "button-event tracking" mode with focus detection
    /// and UTF8-style extended coordinates.
    ///
    /// On success, mouse events will be published to [getc()][Nc#method.getc].
    ///
    /// *C style function: [notcurses_mouse_enable()][c_api::notcurses_mouse_enable].*
    pub fn mouse_enable(&mut self) -> NcResult<()> {
        error![
            unsafe { c_api::notcurses_mouse_enable(self) },
            "Nc.mouse_enable()"
        ]
    }

    /// Returns the number of simultaneous colors claimed to be supported,
    /// if there is color support.
    ///
    /// Note that several terminal emulators advertise more colors than they
    /// actually support, downsampling internally.
    ///
    /// *C style function: [notcurses_palette_size()][c_api::notcurses_palette_size].*
    pub fn palette_size(&self) -> NcResult<u32> {
        let res = unsafe { c_api::notcurses_palette_size(self) };
        if res == 1 {
            return Err(NcError::with_msg(1, "No color support ← Nc.palette_size()"));
        }
        Ok(res)
    }

    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [`render`][crate::Nc#method.render]).
    ///
    /// Returns the current screen geometry (`y`, `x`).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if an
    /// [NcKey::RESIZE][crate::NcKey#associatedconstant.RESIZE] event
    /// has been read and you're not yet ready to render.
    ///
    /// *C style function: [notcurses_refresh()][c_api::notcurses_refresh].*
    //
    pub fn refresh(&mut self) -> NcResult<(NcDim, NcDim)> {
        let (mut y, mut x) = (0, 0);
        error![
            unsafe { c_api::notcurses_refresh(self, &mut y, &mut x) },
            "",
            (y as NcDim, x as NcDim)
        ]
    }

    /// Renders and rasterizes the standard pile in one shot. Blocking call.
    ///
    /// *C style function: [notcurses_render()][c_api::notcurses_render].*
    pub fn render(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_render(self) }, "Nc.render()"]
    }

    /// Performs the rendering and rasterization portion of
    /// [`render`][Nc#method.render] but do not write the resulting buffer
    /// out to the terminal.
    ///
    /// Using this function, the user can control the writeout process,
    /// and render a second frame while writing another.
    ///
    // possible BUG? CHECK:
    /// The returned buffer must be freed by the caller.
    ///
    /// *C style function: [notcurses_render_to_buffer()][c_api::notcurses_render_to_buffer].*
    //
    // CHECK that this works.
    #[deprecated]
    pub fn render_to_buffer(&mut self, buffer: &mut Vec<u8>) -> NcResult<()> {
        let len = buffer.len() as u32;

        // https://github.com/dankamongmen/notcurses/issues/1339
        #[cfg(any(target_arch = "x86_64", target_arch = "i686", target_arch = "x86"))]
        let mut buf = buffer.as_mut_ptr() as *mut i8;
        #[cfg(not(any(target_arch = "x86_64", target_arch = "i686", target_arch = "x86")))]
        let mut buf = buffer.as_mut_ptr() as *mut u8;

        error![unsafe { c_api::notcurses_render_to_buffer(self, &mut buf, &mut len.into()) }]
    }

    /// Writes the last rendered frame, in its entirety, to 'fp'.
    ///
    /// If [`render`][Nc#method.render] has not yet been called,
    /// nothing will be written.
    ///
    /// *C style function: [notcurses_render_to_file()][c_api::notcurses_render_to_file].*
    #[deprecated]
    pub fn render_to_file(&mut self, fp: &mut NcFile) -> NcResult<()> {
        error![unsafe { c_api::notcurses_render_to_file(self, fp.as_nc_ptr()) }]
    }

    /// Acquires an atomic snapshot of the notcurses object's stats.
    ///
    /// *C style function: [notcurses_stats()][c_api::notcurses_stats].*
    pub fn stats(&mut self, stats: &mut NcStats) {
        unsafe {
            c_api::notcurses_stats(self, stats);
        }
    }

    /// Allocates an [`NcStats`] object.
    ///
    /// Use this rather than allocating your own, since future versions of
    /// notcurses might enlarge this structure.
    ///
    /// *C style function: [notcurses_stats_alloc()][c_api::notcurses_stats_alloc].*
    pub fn stats_alloc(&mut self) -> &mut NcStats {
        unsafe { &mut *c_api::notcurses_stats_alloc(self) }
    }

    /// Resets all cumulative stats (immediate ones, such as fbbytes, are not reset).
    ///
    /// *C style function: [notcurses_stats_reset()][c_api::notcurses_stats_reset].*
    pub fn stats_reset(&mut self, stats: &mut NcStats) {
        unsafe {
            c_api::notcurses_stats_reset(self, stats);
        }
    }

    // TODO: decide what to do with these two:
    //
    // /// [notcurses_stdplane()][c_api::notcurses_stdplane], plus free bonus
    // /// dimensions written to non-NULL y/x!
    // ///
    // /// *C style function: [notcurses_stddim_yx()][c_api::notcurses_stddim_yx].*
    // #[inline]
    // pub fn stddim_yx<'a>(
    //     &'a mut self,
    //     y: &mut NcDim,
    //     x: &mut NcDim,
    // ) -> NcResult<&'a mut NcPlane> {
    //     c_api::notcurses_stddim_yx(self, y, x)
    // }

    // /// [stdplane_const()][Nc#method.stdplane_const], plus free
    // /// bonus dimensions written to non-NULL y/x!
    // ///
    // /// *C style function: [notcurses_stddim_yx()][c_api::notcurses_stddim_yx].*
    // #[inline]
    // pub fn stddim_yx_const<'a>(
    //     &'a self,
    //     y: &mut NcDim,
    //     x: &mut NcDim,
    // ) -> NcResult<&'a NcPlane> {
    //     c_api::notcurses_stddim_yx_const(self, y, x)
    // }

    /// Returns a mutable reference to the standard [`NcPlane`] for this terminal.
    ///
    /// The standard plane always exists, and its origin is always at the
    /// uppermost, leftmost cell.
    ///
    /// *C style function: [notcurses_stdplane()][c_api::notcurses_stdplane].*
    pub fn stdplane<'a>(&mut self) -> &'a mut NcPlane {
        unsafe { &mut *c_api::notcurses_stdplane(self) }
    }

    /// Returns a reference to the standard [`NcPlane`] for this terminal.
    ///
    /// The standard plane always exists, and its origin is always at the
    /// uppermost, leftmost cell.
    ///
    /// *C style function: [notcurses_stdplane_const()][c_api::notcurses_stdplane_const].*
    pub fn stdplane_const<'a>(&self) -> &'a NcPlane {
        unsafe { &*c_api::notcurses_stdplane_const(self) }
    }

    /// Destroys the notcurses context.
    ///
    /// *C style function: [notcurses_stop()][c_api::notcurses_stop].*
    pub fn stop(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_stop(self) }]
    }

    /// Gets the name of an [`NcBlitter`] blitter.
    ///
    /// *C style function: [notcurses_str_blitter()][c_api::notcurses_str_blitter].*
    pub fn str_blitter(blitter: NcBlitter) -> String {
        rstring![c_api::notcurses_str_blitter(blitter)].to_string()
    }

    /// Gets the name of an [`NcScale`] scaling mode.
    ///
    /// *C style function: [notcurses_str_scalemode()][c_api::notcurses_str_scalemode].*
    pub fn str_scalemode(scalemode: NcScale) -> String {
        rstring![c_api::notcurses_str_scalemode(scalemode)].to_string()
    }

    /// Gets the lowercase name (or names) of the styles included in an [`NcStyle`].
    ///
    /// *(No equivalent C style function)*
    pub fn str_styles(style: NcStyle) -> String {
        let mut string = String::new();
        for s in style.to_vec() {
            string.push_str(match s {
                NcStyle::ITALIC => "italic",
                NcStyle::UNDERLINE => "underline",
                NcStyle::UNDERCURL => "undercurl",
                NcStyle::STRUCK => "struck",
                NcStyle::BOLD => "bold",
                #[allow(unreachable_patterns)] // FIXME
                NcStyle::NOSTYLE => "none",
                _ => "none",
            });
            string.push(' ');
        }
        let _ = string.pop();
        string
    }

    /// Returns an [`NcStyle`] with the supported curses-style attributes.
    ///
    /// The attribute is only indicated as supported if the terminal can support
    /// it together with color.
    ///
    /// For more information, see the "ncv" capability in terminfo(5).
    ///
    /// *C style function: [notcurses_supported_styles()][c_api::notcurses_supported_styles].*
    pub fn supported_styles(&self) -> NcStyle {
        unsafe { c_api::notcurses_supported_styles(self) as NcStyle }
    }

    /// Returns our current idea of the terminal dimensions in rows and cols.
    ///
    /// *C style function: [notcurses_term_dim_yx()][c_api::notcurses_term_dim_yx].*
    pub fn term_dim_yx(&self) -> (NcDim, NcDim) {
        c_api::notcurses_term_dim_yx(self)
    }

    /// Returns the topmost [`NcPlane`], of which there is always at least one.
    ///
    /// *C style function: [notcurses_top()][c_api::notcurses_top].*
    pub fn top(&mut self) -> &mut NcPlane {
        unsafe { &mut *c_api::notcurses_top(self) }
    }

    /// Returns a human-readable string describing the running notcurses version.
    ///
    /// *C style function: [notcurses_version()][c_api::notcurses_version].*
    pub fn version() -> String {
        rstring![c_api::notcurses_version()].to_string()
    }

    /// Returns the running notcurses version components
    /// (major, minor, patch, tweak).
    ///
    /// *C style function: [notcurses_version_components()][c_api::notcurses_version_components].*
    pub fn version_components() -> (u32, u32, u32, u32) {
        let (mut major, mut minor, mut patch, mut tweak) = (0, 0, 0, 0);
        unsafe {
            c_api::notcurses_version_components(&mut major, &mut minor, &mut patch, &mut tweak);
        }
        (major as u32, minor as u32, patch as u32, tweak as u32)
    }
}
