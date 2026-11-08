//! Frosted-glass contrast tests.

use app::window as app_window;
use cosmic::Theme;
use cosmic::theme::CosmicColor;
use wicara_core::theme as core_theme;

/// WCAG contrast threshold.
const MIN_CONTRAST_RATIO: f32 = 4.5;

/// Contrast black/white.
const MAX_CONTRAST_RATIO: f32 = 21.0;

const BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);
const WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);

/// Linearise sRGB channel.
fn srgb_channel_to_linear(channel: f32) -> f32 {
    let channel = channel.clamp(0.0, 1.0);
    if channel <= 0.03928 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

/// Relative luminance.
fn relative_luminance(rgb: (f32, f32, f32)) -> f32 {
    let (r, g, b) = (
        srgb_channel_to_linear(rgb.0),
        srgb_channel_to_linear(rgb.1),
        srgb_channel_to_linear(rgb.2),
    );
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Contrast ratio.
fn contrast_ratio(fg: (f32, f32, f32), bg: (f32, f32, f32)) -> f32 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);
    let (max, min) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (max + 0.05) / (min + 0.05)
}

/// Alpha-blend colors.
fn blend_over_backdrop(srgba: (f32, f32, f32, f32), backdrop: (f32, f32, f32)) -> (f32, f32, f32) {
    let alpha = srgba.3.clamp(0.0, 1.0);
    (
        srgba.0 * alpha + backdrop.0 * (1.0 - alpha),
        srgba.1 * alpha + backdrop.1 * (1.0 - alpha),
        srgba.2 * alpha + backdrop.2 * (1.0 - alpha),
    )
}

/// Worst-case contrast.
fn worst_bg_contrast(fg: (f32, f32, f32), translucent: (f32, f32, f32, f32)) -> f32 {
    let over_black = contrast_ratio(fg, blend_over_backdrop(translucent, BLACK));
    let over_white = contrast_ratio(fg, blend_over_backdrop(translucent, WHITE));
    over_black.min(over_white)
}

fn srgb_of(color: CosmicColor) -> (f32, f32, f32) {
    (color.color.red, color.color.green, color.color.blue)
}

fn srgba_of(color: CosmicColor) -> (f32, f32, f32, f32) {
    (
        color.color.red,
        color.color.green,
        color.color.blue,
        color.alpha,
    )
}

/// Contrast report text.
fn contrast_report(theme: &Theme) -> String {
    let fg = srgb_of(core_theme::on_bg_color(theme));
    let bg = srgb_of(core_theme::bg_color(theme));
    let component = srgb_of(core_theme::bg_component_color(theme));
    let solid = srgb_of(core_theme::solid_bg_color(theme));
    let translucent = srgba_of(core_theme::solid_bg_color(theme));
    format!(
        "on_bg vs bg={:.2} vs bg_component={:.2} vs solid={:.2} worst_bg={:.2} (need >= {:.1})",
        contrast_ratio(fg, bg),
        contrast_ratio(fg, component),
        contrast_ratio(fg, solid),
        worst_bg_contrast(fg, translucent),
        MIN_CONTRAST_RATIO,
    )
}

#[test]
fn frosted_build_window_asks_blur_when_supported() {
    let (settings, app) = app_window::build_window("Frosted".to_owned(), 400);
    let expect_blur = app_window::blur_supported() && !app_window::prefers_reduced_transparency();

    assert_eq!(
        core_theme::blur_supported(),
        app_window::blur_supported(),
        "app/core blur_supported() must agree on the display policy"
    );
    assert_eq!(
        core_theme::prefers_reduced_transparency(),
        app_window::prefers_reduced_transparency(),
        "app/core reduced-transparency policy must agree"
    );
    const {
        assert!(
            core_theme::ENABLE_BLUR,
            "frosted glass is requested with enable_blur(true)"
        )
    };

    assert_eq!(
        settings.blur, expect_blur,
        "build_window must ask blur (enable_blur(true)) when supported"
    );
    assert_eq!(
        settings.transparent, expect_blur,
        "transparent must follow blur (glass needs a transparent window)"
    );

    let glass = core_theme::glass_window_settings("Frosted", 400, 200, 300);
    let core_expect = core_theme::ensure_blur(true, core_theme::blur_supported());
    assert_eq!(
        glass.blur, core_expect,
        "glass_window_settings must request blur when supported"
    );
    assert_eq!(glass.transparent, glass.blur);
    assert_eq!(glass.size.width, 400.0);
    assert_eq!(glass.size.height, 200.0);

    assert_eq!(app.title, "Frosted");
    assert_eq!(settings.size.width, 400.0);
    let min_size = settings
        .min_size
        .expect("build_window must set min_size from the requested width");
    assert_eq!(min_size.width, 400.0);
    let core_min = glass
        .min_size
        .expect("glass_window_settings must set min_size");
    assert_eq!(core_min.width, 300.0);
}

#[test]
fn frosted_enable_blur_helper_sets_blur_and_transparent_together() {
    let mut s = cosmic::iced::window::Settings::default();
    app_window::enable_blur(&mut s);
    let expect_blur = app_window::blur_supported() && !app_window::prefers_reduced_transparency();
    assert_eq!(
        s.blur, expect_blur,
        "window::enable_blur must request blur when supported"
    );
    assert_eq!(s.transparent, s.blur, "transparent must track blur");

    let mut core_s = cosmic::iced::window::Settings::default();
    core_theme::apply_blur(&mut core_s);
    assert_eq!(
        core_s.blur,
        core_theme::ensure_blur(core_theme::ENABLE_BLUR, core_theme::blur_supported()),
        "theme::apply_blur must follow ensure_blur(true, supported)"
    );
    assert_eq!(core_s.transparent, core_s.blur);

    assert!(!core_theme::ensure_blur(true, false));
    assert!(!core_theme::ensure_blur(false, true));
    assert!(!core_theme::ensure_blur(false, false));
    if core_theme::prefers_reduced_transparency() {
        assert!(!core_theme::ensure_blur(true, true));
    } else {
        assert!(core_theme::ensure_blur(true, true));
    }
}

#[test]
fn frosted_solid_fallback_never_blurs() {
    let s = app_window::solid_settings("Solid", 400);
    assert!(
        !s.blur,
        "solid_settings must never blur (fallback when blur unavailable)"
    );
    assert!(
        !s.transparent,
        "solid_settings must be opaque (backup when blur unavailable)"
    );
    let tiny = app_window::solid_settings("Solid", 1);
    assert!(tiny.size.width >= app_window::DEFAULT_WIDTH as f32);
    assert!(!tiny.blur);

    let core_solid = core_theme::solid_window_settings("Solid", 400, 200, 300);
    assert!(!core_solid.blur, "solid_window_settings must never blur");
    assert!(!core_solid.transparent);
    assert_eq!(core_solid.size.width, 400.0);
    assert_eq!(core_solid.size.height, 200.0);

    for theme in [Theme::light(), Theme::dark()] {
        let solid = core_theme::solid_bg_color(&theme);
        assert_eq!(solid.alpha, 1.0, "solid fallback must be opaque");
    }
}

#[test]
fn frosted_is_dark_toggles_light_and_dark() {
    assert!(
        core_theme::is_dark(&Theme::dark()),
        "dark theme must report is_dark"
    );
    assert!(
        !core_theme::is_dark(&Theme::light()),
        "light theme must not report is_dark"
    );
    assert_ne!(
        core_theme::is_dark(&Theme::dark()),
        core_theme::is_dark(&Theme::light()),
        "is_dark must toggle with the theme"
    );

    let dark_bg = core_theme::bg_color(&Theme::dark());
    let light_bg = core_theme::bg_color(&Theme::light());
    assert_ne!(srgb_of(dark_bg), srgb_of(light_bg));
    assert_ne!(
        srgb_of(core_theme::on_bg_color(&Theme::dark())),
        srgb_of(core_theme::on_bg_color(&Theme::light()))
    );

    let _active: bool = core_theme::is_dark_active();
}

#[test]
fn frosted_contrast_ratio_calc_meets_wcag_aa_worst_bg() {
    assert!(
        (contrast_ratio(WHITE, BLACK) - MAX_CONTRAST_RATIO).abs() < 0.01,
        "black vs white must be 21:1"
    );
    assert!(
        (contrast_ratio(WHITE, WHITE) - 1.0).abs() < 0.01,
        "identical colours must be 1:1"
    );
    let mid_gray = (0.5, 0.5, 0.5);
    assert!(
        (contrast_ratio(mid_gray, BLACK) - contrast_ratio(BLACK, mid_gray)).abs() < 0.01,
        "contrast ratio must be order-independent"
    );

    for theme in [Theme::dark(), Theme::light()] {
        let label = if core_theme::is_dark(&theme) {
            "dark"
        } else {
            "light"
        };
        let fg = srgb_of(core_theme::on_bg_color(&theme));

        for (name, bg) in [
            ("bg", core_theme::bg_color(&theme)),
            ("bg_component", core_theme::bg_component_color(&theme)),
            ("solid_bg", core_theme::solid_bg_color(&theme)),
        ] {
            let ratio = contrast_ratio(fg, srgb_of(bg));
            assert!(
                ratio >= MIN_CONTRAST_RATIO,
                "{label} on_bg vs {name}: contrast {ratio:.2} < {MIN_CONTRAST_RATIO} ({})",
                contrast_report(&theme)
            );
        }

        let translucent = srgba_of(theme.cosmic().background(true).base);
        let worst = worst_bg_contrast(fg, translucent);
        assert!(
            worst >= MIN_CONTRAST_RATIO,
            "{label} worst-bg (translucent over black/white): contrast {worst:.2} < {MIN_CONTRAST_RATIO} ({})",
            contrast_report(&theme)
        );

        let opaque = srgba_of(core_theme::solid_bg_color(&theme));
        let opaque_worst = worst_bg_contrast(fg, opaque);
        assert!(
            (opaque_worst - contrast_ratio(fg, srgb_of(core_theme::solid_bg_color(&theme)))).abs()
                < 0.01,
            "{label} opaque worst-bg must equal the opaque ratio"
        );
        assert!(
            opaque_worst >= MIN_CONTRAST_RATIO,
            "{label} solid fallback worst-bg {opaque_worst:.2} < {MIN_CONTRAST_RATIO}"
        );
    }
}
