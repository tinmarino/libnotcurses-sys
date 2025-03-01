//! `NcCell` methods and associated functions.

use crate::{
    c_api::{self, nccell_load},
    cstring, error, rstring, NcAlpha, NcCell, NcChannels, NcComponent, NcError, NcIntResult,
    NcIntResultApi, NcPaletteIndex, NcPlane, NcResult, NcRgb, NcStyle,
};

#[allow(unused_imports)] // for the doc comments
use crate::NcChannel;

/// # NcCell constructors
impl NcCell {
    /// New `NcCell`, expects a 7-bit [`char`].
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    pub fn from_char7b(ch: char) -> NcResult<Self> {
        if !ch.is_ascii() {
            return Err(NcError::new());
        }
        Ok(NcCell {
            gcluster: (ch as u32).to_le(),
            gcluster_backstop: 0,
            width: 0_u8,
            stylemask: 0 as NcStyle,
            channels: 0 as NcChannels,
        })
    }

    /// New `NcCell`, from a [`char`].
    ///
    /// Expects a plane where to save the extra data if it's greater than 4 bytes.
    #[inline]
    pub fn from_char(plane: &mut NcPlane, ch: char) -> NcResult<Self> {
        let mut cell = Self::new();
        let res = unsafe { nccell_load(plane, &mut cell, cstring![ch.to_string()]) };
        if res == NcIntResult::ERR {
            return Err(NcError::new());
        }
        Ok(cell)
    }

    /// New `NcCell`, from a [`&str`].
    ///
    /// Expects a plane where to save the extra data if it's greater than 4 bytes.
    #[inline]
    pub fn from_str(plane: &mut NcPlane, string: &str) -> NcResult<Self> {
        let mut cell = Self::new();
        let res = unsafe { nccell_load(plane, &mut cell, cstring![string]) };
        if res == NcIntResult::ERR {
            return Err(NcError::new());
        }
        Ok(cell)
    }

    /// New empty `NcCell`.
    #[inline]
    pub fn new() -> Self {
        Self::from_char7b(0 as char).unwrap()
    }

    /// Breaks the UTF-8 string in `egc` down, setting up this `NcCell`,
    /// and returns the number of bytes copied out of `egc`.
    ///
    /// The styling of the cell is left untouched, but any resources are released.
    /// *C style function: [nccell_load()][c_api::nccell_load].*
    pub fn load(plane: &mut NcPlane, cell: &mut NcCell, egc: &str) -> NcResult<u32> {
        let bytes = unsafe { c_api::nccell_load(plane, cell, cstring![egc]) };
        error![
            bytes,
            &format!["NcCell.load(NcPlane, NcCell, {:?})", egc],
            bytes as u32
        ]
    }

    /// Same as [load][NcCell#method.load], plus blasts the styling with
    /// `style` and `channels`.
    ///
    /// - Breaks the UTF-8 string in `gcluster` down, setting up this NcCell.
    /// - Returns the number of bytes copied out of `gcluster`.
    /// - Any resources are released.
    /// - Blasts the styling with `style` and `channels`.
    ///
    /// *C style function: [nccell_prime()][c_api::nccell_prime].*
    pub fn prime(
        plane: &mut NcPlane,
        cell: &mut NcCell,
        gcluster: &str,
        style: NcStyle,
        channels: NcChannels,
    ) -> NcResult<u32> {
        let bytes = c_api::nccell_prime(plane, cell, gcluster, style, channels);
        error![bytes, "", bytes as u32]
    }

    /// Duplicate this `NcCell` into another one.
    ///
    /// Both must be or will be bound to `common_plane`.
    ///
    /// *C style function: [nccell_duplicate()][c_api::nccell_duplicate].*
    pub fn duplicate(&self, common_plane: &mut NcPlane) -> NcResult<NcCell> {
        let mut target = NcCell::new();
        let res = unsafe { c_api::nccell_duplicate(common_plane, &mut target, self) };
        error![res, "NcCell.duplicate()", target]
    }

    /// Initializes (zeroes out) this `NcCell`.
    ///
    /// *C style function: [nccell_init()][c_api::nccell_init].*
    #[inline]
    pub fn init(&mut self) {
        c_api::nccell_init(self);
    }

    /// Releases resources held by the current cell in the [NcPlane] `plane`.
    ///
    /// *C style function: [nccell_release()][c_api::nccell_release].*
    pub fn release(&mut self, plane: &mut NcPlane) {
        unsafe {
            c_api::nccell_release(plane, self);
        }
    }
}

// -----------------------------------------------------------------------------
/// ## NcCell methods: bg|fg `NcChannel`s manipulation.
impl NcCell {
    /// Returns the [`NcChannels`] of this `NcCell`.
    ///
    /// *(No equivalent C style function)*
    pub fn channels(&mut self, plane: &mut NcPlane) -> NcChannels {
        let (mut _styles, mut channels) = (0, 0);
        let _egc = c_api::nccell_extract(plane, self, &mut _styles, &mut channels);
        channels
    }

    /// Extracts the background [`NcAlpha`] (shifted to LSBs).
    ///
    /// *C style function: [nccell_bg_alpha()][c_api::nccell_bg_alpha].*
    pub fn bg_alpha(&self) -> NcAlpha {
        c_api::nccell_bg_alpha(self)
    }

    /// Is the background [`NcChannel`] using the "default background color"?
    ///
    /// *C style function: [nccell_bg_default_p()][c_api::nccell_bg_default_p].*
    pub fn bg_default_p(&self) -> bool {
        c_api::nccell_bg_default_p(self)
    }

    /// Gets the [`NcPaletteIndex`] of the background [`NcChannel`].
    ///
    /// *C style function: [nccell_bg_palindex()][c_api::nccell_bg_palindex].*
    pub fn bg_palindex(&self) -> NcPaletteIndex {
        c_api::nccell_bg_palindex(self)
    }

    /// Is the background [`NcChannel`] using an [`NcPaletteIndex`] indexed
    /// [`NcPalette`][crate::NcPalette] color?
    ///
    /// *C style function: [nccell_bg_palindex_p()][c_api::nccell_bg_palindex_p].*
    pub fn bg_palindex_p(&self) -> bool {
        c_api::nccell_bg_palindex_p(self)
    }

    /// Gets the background [`NcRgb`] (shifted to LSBs).
    ///
    /// *C style function: [nccell_bg_rgb()][c_api::nccell_bg_rgb].*
    pub fn bg_rgb(&self) -> NcRgb {
        c_api::nccell_bg_rgb(self)
    }

    /// Gets the background RGB [`NcComponent`]s.
    ///
    /// *C style function: [nccell_bg_rgb8()][c_api::nccell_bg_rgb8].*
    pub fn bg_rgb8(&self) -> (NcComponent, NcComponent, NcComponent) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::nccell_bg_rgb8(self, &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Extracts the foreground [`NcAlpha`] (shifted to LSBs).
    ///
    /// *C style function: [nccell_fg_alpha()][c_api::nccell_fg_alpha].*
    pub fn fg_alpha(&self) -> NcAlpha {
        c_api::nccell_fg_alpha(self)
    }

    /// Is the foreground [`NcChannel`] using the "default foreground color"?
    ///
    /// *C style function: [nccell_fg_default_p()][c_api::nccell_fg_default_p].*
    pub fn fg_default_p(&self) -> bool {
        c_api::nccell_fg_default_p(self)
    }

    /// Gets the [`NcPaletteIndex`] of the foreground [`NcChannel`].
    ///
    /// *C style function: [nccell_fg_palindex()][c_api::nccell_fg_palindex].*
    pub fn fg_palindex(&self) -> NcPaletteIndex {
        c_api::nccell_fg_palindex(self)
    }

    /// Is the foreground [`NcChannel`] using an [`NcPaletteIndex`] indexed
    /// [`NcPalette`][crate::NcPalette] color?
    ///
    /// *C style function: [nccell_fg_palindex_p()][c_api::nccell_fg_palindex_p].*
    pub fn fg_palindex_p(&self) -> bool {
        c_api::nccell_fg_palindex_p(self)
    }

    /// Gets the foreground [`NcRgb`] (shifted to LSBs).
    ///
    /// *C style function: [nccell_fg_rgb()][c_api::nccell_fg_rgb].*
    pub fn fg_rgb(&self) -> NcRgb {
        c_api::nccell_fg_rgb(self)
    }

    /// Gets the foreground RGB [`NcComponent`]s.
    ///
    /// *C style function: [nccell_fg_rgb8()][c_api::nccell_fg_rgb8].*
    pub fn fg_rgb8(&self) -> (NcComponent, NcComponent, NcComponent) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::nccell_fg_rgb8(self, &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Sets the background [`NcAlpha`].
    ///
    /// *C style function: [nccell_set_bg_alpha()][c_api::nccell_set_bg_alpha].*
    pub fn set_bg_alpha(&mut self, alpha: NcAlpha) {
        c_api::nccell_set_bg_alpha(self, alpha);
    }

    /// Indicates to use the "default color" for the background [`NcChannel`].
    ///
    /// *C style function: [nccell_set_bg_default()][c_api::nccell_set_bg_default].*
    pub fn set_bg_default(&mut self) {
        c_api::nccell_set_bg_default(self);
    }

    /// Sets the background [`NcPaletteIndex`].
    ///
    /// Also sets
    /// [`NcChannels::BG_PALETTE_MASK`][crate::NcChannels#associatedconstant.BG_PALETTE_MASK]
    /// and
    /// [`NcAlpha::OPAQUE`][NcAlpha#associatedconstant.OPAQUE], and clears out
    /// [`NcChannels::BG_DEFAULT_MASK`][NcChannels#associatedconstant.BG_DEFAULT_MASK].
    ///
    /// *C style function: [nccell_set_bg_palindex()][c_api::nccell_set_bg_palindex].*
    pub fn set_bg_palindex(&mut self, index: NcPaletteIndex) {
        c_api::nccell_set_bg_palindex(self, index);
    }

    /// Sets the background [`NcRgb`] and marks it as not using the default color.
    ///
    /// *C style function: [nccell_set_bg_rgb()][c_api::nccell_set_bg_rgb].*
    pub fn set_bg_rgb(&mut self, rgb: NcRgb) {
        c_api::nccell_set_bg_rgb(self, rgb);
    }

    /// Sets the background RGB [`NcComponent`]s, and marks it as not using
    /// the "default color".
    ///
    /// *C style function: [nccell_set_bg_rgb8()][c_api::nccell_set_bg_rgb8].*
    pub fn set_bg_rgb8(&mut self, red: NcComponent, green: NcComponent, blue: NcComponent) {
        c_api::nccell_set_bg_rgb8(self, red, green, blue);
    }

    /// Sets the foreground [`NcAlpha`].
    ///
    /// *C style function: [nccell_set_fg_alpha()][c_api::nccell_set_fg_alpha].*
    pub fn set_fg_alpha(&mut self, alpha: NcAlpha) {
        c_api::nccell_set_fg_alpha(self, alpha);
    }

    /// Indicates to use the "default color" for the foreground [`NcChannel`].
    ///
    /// *C style function: [nccell_set_fg_default()][c_api::nccell_set_fg_default].*
    pub fn set_fg_default(&mut self) {
        c_api::nccell_set_fg_default(self);
    }

    /// Sets the foreground [`NcPaletteIndex`].
    ///
    /// Also sets
    /// [`NcChannels::FG_PALETTE_MASK`][crate::NcChannels#associatedconstant.FG_PALETTE_MASK]
    /// and
    /// [`NcAlpha::OPAQUE`][NcAlpha#associatedconstant.OPAQUE], and clears out
    /// [`NcChannels::FG_DEFAULT_MASK`][NcChannels#associatedconstant.FG_DEFAULT_MASK].
    ///
    /// *C style function: [nccell_set_fg_palindex()][c_api::nccell_set_fg_palindex].*
    pub fn set_fg_palindex(&mut self, index: NcPaletteIndex) {
        c_api::nccell_set_fg_palindex(self, index);
    }

    /// Sets the foreground [`NcRgb`] and marks it as not using the default color.
    ///
    /// *C style function: [nccell_set_fg_rgb()][c_api::nccell_set_fg_rgb].*
    pub fn set_fg_rgb(&mut self, rgb: NcRgb) {
        c_api::nccell_set_fg_rgb(self, rgb);
    }

    /// Sets the foreground RGB [`NcComponent`]s, and marks it as not using
    /// the "default color".
    ///
    /// *C style function: [nccell_set_fg_rgb8()][c_api::nccell_set_fg_rgb8].*
    pub fn set_fg_rgb8(&mut self, red: NcComponent, green: NcComponent, blue: NcComponent) {
        c_api::nccell_set_fg_rgb8(self, red, green, blue);
    }
}

/// # `NcCell` methods: other components
impl NcCell {
    /// Returns true if the two cells have distinct `EGC`s, attributes,
    /// or [`NcChannel`]s.
    ///
    /// The actual egcpool index needn't be the same--indeed, the planes
    /// needn't even be the same. Only the expanded `EGC` must be bit-equal.
    ///
    /// *C style function: [nccellcmp()][c_api::nccellcmp].*
    pub fn compare(plane1: &NcPlane, cell1: &NcCell, plane2: &NcPlane, cell2: &NcCell) -> bool {
        c_api::nccellcmp(plane1, cell1, plane2, cell2)
    }

    /// Saves the [`NcStyle`] and the [`NcChannels`], and returns the `EGC`.
    /// (These are the three elements of an `NcCell`).
    ///
    /// *C style function: [nccell_fg_alpha()][c_api::nccell_fg_alpha].*
    pub fn extract(
        &mut self,
        plane: &mut NcPlane,
        styles: &mut NcStyle,
        channels: &mut NcChannels,
    ) -> String {
        c_api::nccell_extract(plane, self, styles, channels)
    }

    /// Returns the `EGC` of the `NcCell`.
    ///
    /// See also: [extended_gcluster][NcCell#method.extended_gcluster] method.
    ///
    /// *(No equivalent C style function)*
    pub fn egc(&mut self, plane: &mut NcPlane) -> String {
        let (mut _styles, mut _channels) = (0, 0);
        c_api::nccell_extract(plane, self, &mut _styles, &mut _channels)
    }

    /// Returns the [`NcStyle`] bits.
    ///
    /// *C style function: [nccell_styles()][c_api::nccell_styles].*
    pub fn styles(&mut self) -> NcStyle {
        c_api::nccell_styles(self)
    }

    /// Removes the specified [`NcStyle`] bits.
    ///
    /// *C style function: [nccell_off_styles()][c_api::nccell_off_styles].*
    pub fn styles_off(&mut self, stylebits: NcStyle) {
        c_api::nccell_off_styles(self, stylebits)
    }

    /// Adds the specified [`NcStyle`] bits.
    ///
    /// *C style function: [nccell_on_styles()][c_api::nccell_on_styles].*
    pub fn styles_on(&mut self, stylebits: NcStyle) {
        c_api::nccell_on_styles(self, stylebits)
    }

    /// Sets just the specified [`NcStyle`] bits.
    ///
    /// *C style function: [nccell_set_styles()][c_api::nccell_set_styles].*
    pub fn styles_set(&mut self, stylebits: NcStyle) {
        c_api::nccell_set_styles(self, stylebits)
    }
}

/// # `NcCell` methods: text
impl NcCell {
    /// Returns the number of columns occupied by the cell.
    ///
    /// See [`ncstrwidth`][c_api::ncstrwidth] for an equivalent for multiple EGCs.
    ///
    /// *C style function: [nccell_cols()][c_api::nccell_cols].*
    pub const fn cols(&self) -> u8 {
        c_api::nccell_cols(self)
    }

    /// Returns a pointer to the `EGC` of this NcCell in the `plane`.
    ///
    /// This pointer can be invalidated by any further operation on the referred
    /// plane, so… watch out!
    ///
    /// *C style function: [nccell_extended_gcluster()][c_api::nccell_wide_left_p].*
    pub fn extended_gcluster(&self, plane: &NcPlane) -> &str {
        let egcpointer = unsafe { c_api::nccell_extended_gcluster(plane, self) };
        rstring![egcpointer]
    }

    /// Copies the UTF8-encoded `EGC` out of this NcCell,
    /// whether simple or complex.
    ///
    /// The result is not tied to the [NcPlane],
    /// and persists across erases and destruction.
    ///
    /// *C style function: [nccell_strdup()][c_api::nccell_strdup].*
    pub fn strdup(&self, plane: &NcPlane) -> String {
        c_api::nccell_strdup(plane, self)
    }

    /// Does this NcCell contain a wide codepoint?
    ///
    /// *C style function: [nccell_double_wide_p()][c_api::nccell_double_wide_p].*
    pub fn double_wide_p(&self) -> bool {
        c_api::nccell_double_wide_p(self)
    }

    /// Is this the left half of a wide character?
    ///
    /// *C style function: [nccell_wide_left_p()][c_api::nccell_wide_left_p].*
    pub fn wide_left_p(&self) -> bool {
        c_api::nccell_wide_right_p(self)
    }

    /// Is this the right side of a wide character?
    ///
    /// *C style function: [nccell_wide_right_p()][c_api::nccell_wide_right_p].*
    pub fn wide_right_p(&self) -> bool {
        c_api::nccell_wide_right_p(self)
    }
}

/// # `NcCell` methods: boxes
impl NcCell {
    /// Loads up six cells with the `EGC`s necessary to draw a box.
    ///
    /// On error, any [`NcCell`]s this function might have loaded before the error
    /// are [release][NcCell#method.release]d.
    /// There must be at least six `EGC`s in `gcluster`.
    ///
    /// *C style function: [nccells_load_box()][c_api::nccells_load_box].*
    pub fn load_box(
        plane: &mut NcPlane,
        style: NcStyle,
        channels: NcChannels,
        ul: &mut NcCell,
        ur: &mut NcCell,
        ll: &mut NcCell,
        lr: &mut NcCell,
        hl: &mut NcCell,
        vl: &mut NcCell,
        gcluster: &str,
    ) -> NcResult<()> {
        error![c_api::nccells_load_box(
            plane, style, channels, ul, ur, ll, lr, hl, vl, gcluster
        )]
    }

    /// NcCell.[load_box()][NcCell#method.box] with the double box-drawing characters.
    ///
    /// *C style function: [nccells_double_box()][c_api::nccells_double_box].*
    pub fn double_box(
        plane: &mut NcPlane,
        style: NcStyle,
        channels: NcChannels,
        ul: &mut NcCell,
        ur: &mut NcCell,
        ll: &mut NcCell,
        lr: &mut NcCell,
        hl: &mut NcCell,
        vl: &mut NcCell,
    ) -> NcResult<()> {
        error![unsafe {
            c_api::nccells_double_box(plane, style as u32, channels, ul, ur, ll, lr, hl, vl)
        }]
    }

    /// NcCell.[load_box()][NcCell#method.box] with the rounded box-drawing characters.
    ///
    /// *C style function: [nccells_rounded_box()][c_api::nccells_double_box].*
    pub fn rounded_box(
        plane: &mut NcPlane,
        style: NcStyle,
        channels: NcChannels,
        ul: &mut NcCell,
        ur: &mut NcCell,
        ll: &mut NcCell,
        lr: &mut NcCell,
        hl: &mut NcCell,
        vl: &mut NcCell,
    ) -> NcResult<()> {
        error![unsafe {
            c_api::nccells_rounded_box(plane, style as u32, channels, ul, ur, ll, lr, hl, vl)
        }]
    }

    /// NcCell.[load_box()][NcCell#method.box] with ASCII characters.
    ///
    /// *C style function: [nccells_ascii_box()][c_api::nccells_ascii_box].*
    pub fn ascii_box(
        plane: &mut NcPlane,
        style: NcStyle,
        channels: NcChannels,
        ul: &mut NcCell,
        ur: &mut NcCell,
        ll: &mut NcCell,
        lr: &mut NcCell,
        hl: &mut NcCell,
        vl: &mut NcCell,
    ) -> NcResult<()> {
        error![c_api::nccells_ascii_box(
            plane, style, channels, ul, ur, ll, lr, hl, vl
        )]
    }
    /// NcCell.[load_box()][NcCell#method.box] with the heavy line
    /// box-drawing characters.
    ///
    /// *C style function: [nccells_heavy_box()][c_api::nccells_heavy_box].*
    pub fn heavy_box(
        plane: &mut NcPlane,
        style: NcStyle,
        channels: NcChannels,
        ul: &mut NcCell,
        ur: &mut NcCell,
        ll: &mut NcCell,
        lr: &mut NcCell,
        hl: &mut NcCell,
        vl: &mut NcCell,
    ) -> NcResult<()> {
        error![c_api::nccells_heavy_box(
            plane, style, channels, ul, ur, ll, lr, hl, vl
        )]
    }

    /// NcCell.[load_box()][NcCell#method.box] with the light line
    /// box-drawing characters.
    ///
    /// *C style function: [nccells_light_box()][c_api::nccells_light_box].*
    pub fn light_box(
        plane: &mut NcPlane,
        style: NcStyle,
        channels: NcChannels,
        ul: &mut NcCell,
        ur: &mut NcCell,
        ll: &mut NcCell,
        lr: &mut NcCell,
        hl: &mut NcCell,
        vl: &mut NcCell,
    ) -> NcResult<()> {
        error![c_api::nccells_light_box(
            plane, style, channels, ul, ur, ll, lr, hl, vl
        )]
    }
}
