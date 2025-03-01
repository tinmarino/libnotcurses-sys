//! A selection of the [ffi] bindings intended to be used directly.
//!
//! The full list of bindings is under the [ffi] submodule.
//!
//! The current module publicly re-exports bindgen generated structs, functions,
//! and constants, for their direct usage.

// BUG ISSUES:
// https://github.com/rust-lang/rust-bindgen/issues/1470
#[allow(clippy::all)]
// https://github.com/rust-lang/rust-bindgen/issues/1651
#[allow(unknown_lints, deref_nullptr)]
pub(crate) mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Miscellaneous ---------------------------------------------------------------

#[rustfmt::skip]
#[doc(inline)]
pub use ffi::{
    // functions
    ncstrwidth,
    ncstrwidth_valid,
};

// blitset ---------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// blitset,

// cell ------------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// cell,
//
// // constants
// NCALPHA_BLEND,
// NCALPHA_HIGHCONTRAST,
// NCALPHA_OPAQUE,
// NCALPHA_TRANSPARENT,
// NC_BGDEFAULT_MASK,
// NC_BG_ALPHA_MASK,
// NC_BG_PALETTE,
// NC_BG_RGB_MASK,
// NC_FGDEFAULT_MASK,
// NC_FG_ALPHA_MASK,
// NC_FG_PALETTE,
// NC_FG_RGB_MASK,

#[doc(inline)]
pub use ffi::{
    // functions
    nccell_duplicate,
    nccell_extended_gcluster,
    nccell_load,
    nccell_release,
    nccells_double_box,
    nccells_rounded_box,
};

// channel ---------------------------------------------------------------------
//
// already wrapped:
//
// // constants
// CHANNEL_ALPHA_MASK,

// ncalign ---------------------------------------------------------------------
//
// already wrapped:
//
// // type definitions
// ncalign_e,
//
// // constants
// ncalign_e_NCALIGN_CENTER,
// ncalign_e_NCALIGN_LEFT,
// ncalign_e_NCALIGN_RIGHT,
// ncalign_e_NCALIGN_UNALIGNED,

// ncblitter -------------------------------------------------------------------
//
// already wrapped:
//
// // type definitions
// ncblitter_e,
//
// // constants
// ncblitter_e_NCBLIT_1x1,
// ncblitter_e_NCBLIT_2x1,
// ncblitter_e_NCBLIT_2x2,
// ncblitter_e_NCBLIT_3x2,
// ncblitter_e_NCBLIT_4x1,
// ncblitter_e_NCBLIT_8x1,
// ncblitter_e_NCBLIT_BRAILLE,
// ncblitter_e_NCBLIT_DEFAULT,
// ncblitter_e_NCBLIT_PIXEL,

#[doc(inline)]
pub use ffi::{
    ncblit_bgrx,
    ncblit_rgb_loose,
    ncblit_rgb_packed,
    // functions
    ncblit_rgba,
};

// ncbox -----------------------------------------------------------------------

// // constants
// NCBOXCORNER_MASK,
// NCBOXCORNER_SHIFT,
// NCBOXGRAD_BOTTOM,
// NCBOXGRAD_LEFT,
// NCBOXGRAD_RIGHT,
// NCBOXGRAD_TOP,
// NCBOXMASK_BOTTOM,
// NCBOXMASK_LEFT,
// NCBOXMASK_RIGHT,
// NCBOXMASK_TOP,

// nccapabilit* ----------------------------------------------------------------
//
// already wrapped:
//
// // structs
// nccapabilities,
//
// // functions
// nccapability_canchangecolor,

// ncdirect --------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncdirect,
//
// // functions
// ncdirect_canbraille,
// ncdirect_canopen_videos
// ncdirect_canchangecolor,
// ncdirect_canfade,
// ncdirect_canhalfblock,
// ncdirect_canquadrant,
// ncdirect_cantruecolor,
// ncdirect_capabilities,
// ncdirect_hline_interp,
// ncdirect_vline_interp,
//
// // constants
// NCDIRECT_OPTION_DRAIN_INPUT,
// NCDIRECT_OPTION_INHIBIT_CBREAK,
// NCDIRECT_OPTION_INHIBIT_SETLOCALE,
// NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS,
// NCDIRECT_OPTION_VERBOSE
// NCDIRECT_OPTION_VERY_VERBOSE

#[doc(inline)]
pub use ffi::{
    // functions
    ncdirect_box,
    ncdirect_canget_cursor,
    ncdirect_canopen_images,
    ncdirect_canutf8,
    ncdirect_check_pixel_support,
    ncdirect_clear,
    ncdirect_core_init,
    ncdirect_cursor_disable,
    ncdirect_cursor_down,
    ncdirect_cursor_enable,
    ncdirect_cursor_left,
    ncdirect_cursor_move_yx,
    ncdirect_cursor_pop,
    ncdirect_cursor_push,
    ncdirect_cursor_right,
    ncdirect_cursor_up,
    ncdirect_cursor_yx,
    ncdirect_detected_terminal,
    ncdirect_dim_x,
    ncdirect_dim_y,
    ncdirect_double_box,
    ncdirect_flush,
    ncdirect_get,
    ncdirect_init,
    ncdirect_inputready_fd,
    ncdirect_off_styles,
    ncdirect_on_styles,
    ncdirect_palette_size,
    ncdirect_putegc,
    ncdirect_putstr,
    ncdirect_raster_frame,
    ncdirect_readline,
    ncdirect_render_frame,
    ncdirect_render_image,
    ncdirect_rounded_box,
    ncdirect_set_bg_default,
    ncdirect_set_bg_palindex,
    ncdirect_set_bg_rgb,
    ncdirect_set_fg_default,
    ncdirect_set_fg_palindex,
    ncdirect_set_fg_rgb,
    ncdirect_set_styles,
    ncdirect_stop,
    ncdirect_stream,
    ncdirect_styles,
    ncdirect_supported_styles,
};

// ncdirectf --------------------------------------------------------------------
//
// already wrapped:
//
// // type alias
// ncdirectf,
//

#[doc(inline)]
pub use ffi::{
    ncdirectf_free,
    // functions
    ncdirectf_from_file,
    ncdirectf_geom,
    ncdirectf_render,
};

// ncfadectx -------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncfadectx,

#[doc(inline)]
pub use ffi::{
    // functions
    ncfadectx_free,
    ncfadectx_iterations,
    ncfadectx_setup,
};

// ncinput ---------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncinput,

// ncloglevel ------------------------------------------------------------------
//
// already wrapped:
//
// // type definitions
// ncloglevel_e,
//
// // constants
// ncloglevel_e_NCLOGLEVEL_DEBUG,
// ncloglevel_e_NCLOGLEVEL_ERROR,
// ncloglevel_e_NCLOGLEVEL_FATAL,
// ncloglevel_e_NCLOGLEVEL_INFO,
// ncloglevel_e_NCLOGLEVEL_PANIC,
// ncloglevel_e_NCLOGLEVEL_SILENT,
// ncloglevel_e_NCLOGLEVEL_TRACE,
// ncloglevel_e_NCLOGLEVEL_VERBOSE,
// ncloglevel_e_NCLOGLEVEL_WARNING,

// ncfdplane -------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncfdplane,
// ncfdplane_options,

#[doc(inline)]
pub use ffi::{
    // functions
    ncfdplane_create,
    ncfdplane_destroy,
    ncfdplane_plane,
};

// ncmenu ----------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncmenu,
// ncmenu_item,
// ncmenu_options,
// ncmenu_section,
//
// // constants
// NCMENU_OPTION_BOTTOM,
// NCMENU_OPTION_HIDING,

#[doc(inline)]
pub use ffi::{
    // functions
    ncmenu_create,
    ncmenu_destroy,
    ncmenu_item_set_status,
    ncmenu_mouse_selected,
    ncmenu_nextitem,
    ncmenu_nextsection,
    ncmenu_offer_input,
    ncmenu_plane,
    ncmenu_previtem,
    ncmenu_prevsection,
    ncmenu_rollup,
    ncmenu_selected,
    ncmenu_unroll,
};

// ncmetric --------------------------------------------------------------------
//
// already wrapped:
//
// // functions
// ncmetric
//
// // constants
// PREFIXCOLUMNS,
// PREFIXSTRLEN,
// BPREFIXCOLUMNS,
// BPREFIXSTRLEN,
// IPREFIXCOLUMNS,
// IPREFIXSTRLEN,

// ncmultiselector -------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncmultiselector,
// ncmselector_item,
// ncmultiselector_options,

#[doc(inline)]
pub use ffi::{
    // functions
    ncmultiselector_create,
    ncmultiselector_destroy,
    ncmultiselector_offer_input,
    ncmultiselector_plane,
    ncmultiselector_selected,
};

// ncpile ----------------------------------------------------------------------

#[doc(inline)]
pub use ffi::{
    // functions
    ncpile_bottom,
    ncpile_create,
    ncpile_rasterize,
    ncpile_render,
    ncpile_render_to_buffer,
    ncpile_render_to_file,
    ncpile_top,
};

// ncplane ---------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncplane,
// ncplane_options,
//
// // functions
// ncplane_channels,
// ncplane_gradient,
// ncplane_set_bchannel,
// ncplane_set_channels,
// ncplane_set_fchannel,
//
// // constants
// NCPLANE_OPTION_HORALIGNED,
// NCPLANE_OPTION_MARGINALIZED,
// NCPLANE_OPTION_VERALIGNED,
//
// // type alias
// ncdirectv,

#[doc(inline)]
pub use ffi::{
    // functions
    ncplane_above,
    ncplane_abs_x,
    ncplane_abs_y,
    ncplane_abs_yx,
    ncplane_as_rgba,
    ncplane_at_cursor,
    ncplane_at_cursor_cell,
    ncplane_at_yx,
    ncplane_at_yx_cell,
    ncplane_base,
    ncplane_below,
    ncplane_box,
    ncplane_center_abs,
    ncplane_contents,
    ncplane_create,
    ncplane_cursor_move_rel,
    ncplane_cursor_move_yx,
    ncplane_cursor_yx,
    ncplane_destroy,
    ncplane_dim_yx,
    ncplane_dup,
    ncplane_erase,
    ncplane_erase_region,
    ncplane_fadein,
    ncplane_fadein_iteration,
    ncplane_fadeout,
    ncplane_fadeout_iteration,
    ncplane_format,
    ncplane_greyscale,
    ncplane_highgradient,
    ncplane_highgradient_sized,
    ncplane_hline_interp,
    ncplane_home,
    ncplane_mergedown,
    ncplane_mergedown_simple,
    ncplane_move_above,
    ncplane_move_below,
    ncplane_move_family_above,
    ncplane_move_family_below,
    ncplane_move_yx,
    ncplane_notcurses,
    ncplane_notcurses_const,
    ncplane_off_styles,
    ncplane_on_styles,
    ncplane_parent,
    ncplane_parent_const,
    ncplane_pixelgeom,
    ncplane_polyfill_yx,
    ncplane_pulse,
    ncplane_putc_yx,
    ncplane_putnstr_aligned,
    ncplane_putnstr_yx,
    ncplane_putstr_aligned,
    ncplane_putstr_stained,
    ncplane_putstr_yx,
    ncplane_puttext,
    ncplane_putwegc_stained,
    ncplane_putwstr_stained,
    ncplane_qrcode,
    ncplane_reparent,
    ncplane_reparent_family,
    ncplane_resize,
    ncplane_resize_marginalized,
    ncplane_resize_maximize,
    ncplane_resize_realign,
    ncplane_resizecb,
    ncplane_rotate_ccw,
    ncplane_rotate_cw,
    ncplane_scrolling_p,
    ncplane_scrollup,
    ncplane_scrollup_child,
    ncplane_set_base,
    ncplane_set_base_cell,
    ncplane_set_bg_alpha,
    ncplane_set_bg_default,
    ncplane_set_bg_palindex,
    ncplane_set_bg_rgb,
    ncplane_set_bg_rgb8,
    ncplane_set_fg_alpha,
    ncplane_set_fg_default,
    ncplane_set_fg_palindex,
    ncplane_set_fg_rgb,
    ncplane_set_fg_rgb8,
    ncplane_set_resizecb,
    ncplane_set_scrolling,
    ncplane_set_styles,
    ncplane_set_userptr,
    ncplane_stain,
    ncplane_styles,
    ncplane_translate,
    ncplane_translate_abs,
    ncplane_userptr,
    ncplane_vline_interp,
    ncplane_x,
    ncplane_y,
    ncplane_yx,
};

// ncplot ----------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncdplot, // f64
// ncuplot, // u64
// ncplot_options,
//
// // constants
// NCPLOT_OPTION_DETECTMAXONLY,
// NCPLOT_OPTION_EXPONENTIALD,
// NCPLOT_OPTION_LABELTICKSD,
// NCPLOT_OPTION_NODEGRADE,
// NCPLOT_OPTION_VERTICALI,

#[doc(inline)]
pub use ffi::{
    // functions
    ncdplot_add_sample,
    ncdplot_create,
    ncdplot_destroy,
    ncdplot_plane,
    ncdplot_sample,
    ncdplot_set_sample,

    ncuplot_add_sample,
    ncuplot_create,
    ncuplot_destroy,
    ncuplot_plane,
    ncuplot_sample,
    ncuplot_set_sample,
};

// ncreader --------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncreader,
// ncreader_options,
//
// // constants
// NCREADER_OPTION_CURSOR,
// NCREADER_OPTION_HORSCROLL,
// NCREADER_OPTION_NOCMDKEYS,
// NCREADER_OPTION_VERSCROLL,

#[doc(inline)]
pub use ffi::{
    // functions
    ncreader_clear,
    ncreader_contents,
    ncreader_create,
    ncreader_destroy,
    ncreader_move_down,
    ncreader_move_left,
    ncreader_move_right,
    ncreader_move_up,
    ncreader_offer_input,
    ncreader_plane,
    ncreader_write_egc,
};

// ncprogbar -------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncprogbar,
// ncprogbar_options,
//
// // constants
// NCPROGBAR_OPTION_RETROGRADE,

#[doc(inline)]
pub use ffi::{
    // functions
    ncprogbar_create,
    ncprogbar_destroy,
    ncprogbar_plane,
    ncprogbar_progress,
    ncprogbar_set_progress,
};

// ncreel ----------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncreel,
// ncreel_options,
//
// // constants
// NCREEL_OPTION_CIRCULAR,
// NCREEL_OPTION_INFINITESCROLL,

#[doc(inline)]
pub use ffi::{
    // functions
    ncreel_add,
    ncreel_create,
    ncreel_del,
    ncreel_destroy,
    ncreel_focused,
    ncreel_next,
    ncreel_offer_input,
    ncreel_plane,
    ncreel_prev,
    ncreel_redraw,
    ncreel_tabletcount,
};

// ncscale ---------------------------------------------------------------------
//
// already wrapped:
//
// // type definitions
// ncscale_e,
//
// // constants
// ncscale_e_NCSCALE_NONE,
// ncscale_e_NCSCALE_SCALE,
// ncscale_e_NCSCALE_STRETCH,
// ncscale_e_NCSCALE_NONE_HIRES,
// ncscale_e_NCSCALE_SCALE_HIRES,

// ncselector ------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncselector,
// ncselector_item,
// ncselector_options,

#[doc(inline)]
pub use ffi::{
    // functions
    ncselector_additem,
    ncselector_create,
    ncselector_delitem,
    ncselector_destroy,
    ncselector_nextitem,
    ncselector_offer_input,
    ncselector_plane,
    ncselector_previtem,
    ncselector_selected,
};

// ncstats ---------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncstats,

// ncssubproc ------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncsubproc
// ncsubproc_options

#[doc(inline)]
pub use ffi::{
    // functions
    ncsubproc_createv,
    ncsubproc_createvp,
    ncsubproc_createvpe,
    ncsubproc_destroy,
    ncsubproc_plane,
};

// ncstyle ---------------------------------------------------------------------
//
// already wrapped:
//
// // constants
// NCSTYLE_MASK,
// NCSTYLE_ITALIC,
// NCSTYLE_UNDERLINE,
// NCSTYLE_UNDERCURL,
// NCSTYLE_BOLD,
// NCSTYLE_STRUCK,
// NCSTYLE_NONE,

// nctabbed --------------------------------------------------------------------
//
// // structs
// nctab,
// nctabbed,
// nctabbed_options,
//
// // constants
// NCTABBED_OPTION_BOTTOM,

#[doc(inline)]
pub use ffi::{
    // functions
    nctab_cb,
    nctab_move,
    nctab_move_left,
    nctab_move_right,
    nctab_name,
    nctab_name_width,
    nctab_next,
    nctab_prev,
    nctab_set_cb,
    nctab_set_name,
    nctab_set_userptr,
    nctab_userptr,
    nctabbed_add,
    nctabbed_channels,
    nctabbed_content_plane,
    nctabbed_create,
    nctabbed_del,
    nctabbed_destroy,
    nctabbed_ensure_selected_header_visible,
    nctabbed_leftmost,
    nctabbed_next,
    nctabbed_plane,
    nctabbed_prev,
    nctabbed_redraw,
    nctabbed_rotate,
    nctabbed_select,
    nctabbed_selected,
    nctabbed_separator,
    nctabbed_separator_width,
    nctabbed_set_hdrchan,
    nctabbed_set_selchan,
    nctabbed_set_separator,
    nctabbed_set_sepchan,
    nctabbed_tabcount,
    nctablet_ncplane,
};

// nctablet --------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// nctablet,

#[doc(inline)]
pub use ffi::{
    // functions
    nctablet_plane,
    nctablet_userptr,
};

// nctree ----------------------------------------------------------------------
//
// // structs
// nctree,
// nctree_item,
// nctree_options,
//
#[doc(inline)]
pub use ffi::{
    // functions
    nctree_create,
    nctree_destroy,
    nctree_focused,
    nctree_goto,
    nctree_next,
    nctree_offer_input,
    nctree_plane,
    nctree_prev,
    nctree_redraw,
};

// ncvgeom ---------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncvgeom

// ncvisual --------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncvisual,
// ncvisual_options,
//
// // constants
// NCVISUAL_OPTION_ADDALPHA,
// NCVISUAL_OPTION_BLEND,
// NCVISUAL_OPTION_CHILDPLANE
// NCVISUAL_OPTION_HORALIGNED
// NCVISUAL_OPTION_NODEGRADE,
// NCVISUAL_OPTION_NOINTERPOLATE
// NCVISUAL_OPTION_VERALIGNED,

#[doc(inline)]
pub use ffi::{
    // functions
    ncvisual_at_yx,
    ncvisual_blitter_geom,
    ncvisual_decode,
    ncvisual_decode_loop,
    ncvisual_destroy,
    ncvisual_from_bgra,
    ncvisual_from_file,
    ncvisual_from_palidx,
    ncvisual_from_plane,
    ncvisual_from_rgb_loose,
    ncvisual_from_rgb_packed,
    ncvisual_from_rgba,
    ncvisual_media_defblitter,
    ncvisual_polyfill_yx,
    ncvisual_render,
    ncvisual_resize,
    ncvisual_resize_noninterpolative,
    ncvisual_rotate,
    ncvisual_set_yx,
    ncvisual_simple_streamer,
    ncvisual_stream,
    ncvisual_subtitle, // deprecated
    ncvisual_subtitle_plane,
};

// notcurses -------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// notcurses,
// notcurses_options,
//
// // constants
// NCOPTION_DRAIN_INPUT,
// NCOPTION_INHIBIT_SETLOCALE,
// NCOPTION_NO_ALTERNATE_SCREEN,
// NCOPTION_NO_CLEAR_BITMAPS
// NCOPTION_NO_FONT_CHANGES,
// NCOPTION_NO_QUIT_SIGHANDLERS,
// NCOPTION_NO_WINCH_SIGHANDLER,
// NCOPTION_PRESERVE_CURSOR,
// NCOPTION_SUPPRESS_BANNERS,

#[doc(inline)]
pub use ffi::{
    // functions
    notcurses_accountname,
    notcurses_at_yx,
    notcurses_bottom,
    notcurses_canbraille,
    notcurses_canchangecolor,
    notcurses_canfade,
    notcurses_canhalfblock,
    notcurses_canopen_images,
    notcurses_canopen_videos,
    notcurses_canquadrant,
    notcurses_cansextant,
    notcurses_cantruecolor,
    notcurses_canutf8,
    notcurses_check_pixel_support,
    notcurses_core_init,
    notcurses_cursor_disable,
    notcurses_cursor_enable,
    notcurses_cursor_yx,
    notcurses_debug,
    notcurses_detected_terminal,
    notcurses_drop_planes,
    notcurses_enter_alternate_screen,
    notcurses_get,
    notcurses_getvec,
    notcurses_hostname,
    notcurses_init,
    notcurses_inputready_fd,
    notcurses_leave_alternate_screen,
    notcurses_lex_blitter,
    notcurses_lex_margins,
    notcurses_lex_scalemode,
    notcurses_linesigs_disable,
    notcurses_linesigs_enable,
    notcurses_mouse_disable,
    notcurses_mouse_enable,
    notcurses_palette_size,
    notcurses_refresh,
    notcurses_render,
    notcurses_render_to_buffer, // deprecated
    notcurses_render_to_file,   // deprecated
    notcurses_stats,
    notcurses_stats_alloc,
    notcurses_stats_reset,
    notcurses_stdplane,
    notcurses_stdplane_const,
    notcurses_stop,
    notcurses_str_blitter,
    notcurses_str_scalemode,
    notcurses_supported_styles,
    notcurses_top,
    notcurses_ucs32_to_utf8,
    notcurses_version,
    notcurses_version_components,
};

// palette ---------------------------------------------------------------------
//
// already wrapped:
//
// // structs
// ncpalette,
//
// // constants
// NCPALETTESIZE,

#[doc(inline)]
pub use ffi::{
    // functions
    ncpalette_free,
    ncpalette_new,
    ncpalette_use,
};

// fade callback ---------------------------------------------------------------
//
// already wrapped:
//
// // types
// fadecb,
