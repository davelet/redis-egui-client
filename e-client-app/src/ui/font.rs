use egui::{Context, FontData, FontDefinitions, FontFamily, FontTweak};
use std::sync::Arc;

fn font_data_with_tweak(data: Vec<u8>, y_offset_factor: f32, scale: f32) -> Arc<FontData> {
    Arc::new(FontData::from_owned(data).tweak(FontTweak {
        y_offset_factor,
        scale,
        ..Default::default()
    }))
}
fn append_builtin_emoji_fallbacks(fonts: &mut FontDefinitions) {
    let has_noto_emoji = fonts.font_data.contains_key("NotoEmoji-Regular");
    let has_emoji_icon_font = fonts.font_data.contains_key("emoji-icon-font");

    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        let entry = fonts.families.entry(family).or_insert_with(Vec::new);
        if has_noto_emoji && !entry.iter().any(|name| name == "NotoEmoji-Regular") {
            entry.push("NotoEmoji-Regular".to_owned());
        }
        if has_emoji_icon_font && !entry.iter().any(|name| name == "emoji-icon-font") {
            entry.push("emoji-icon-font".to_owned());
        }
    }
}

/// Setup fonts for egui context.
///
/// Strategy: Load system fonts from disk, register them as FontData, then
/// build a fallback chain per family. Every name in `families` must have
/// corresponding FontData in `font_data` — egui panics otherwise.
///
/// Returns `true` if global monospace is enabled but SFNSMono font was not found.
pub fn setup_fonts(ctx: &Context, global_monospace: bool) -> bool {
    let mut fonts = FontDefinitions::default();
    let mut missing_monospace_warning = false;

    #[cfg(target_os = "macos")]
    {
        use tracing::{info, warn};

        // ── macOS ───────────────────────────────────────────────────────────
        //
        // Strategy: STHeiti Light (54 MB, Apple's CJK-optimized font) is used as
        // the PRIMARY font for all glyphs. It covers:
        //   - All CJK characters (simplified/traditional Chinese, Japanese, Korean)
        //   - All Latin glyphs with Apple's rendering engine
        //
        // SFNS (8 MB) is the SECONDARY font — it supplements Latin glyphs for
        // users who prefer the pure San Francisco look over STHeiti's style.
        //
        // WHY this order: putting STHeiti Light first eliminates mid-run font
        // switching for CJK. With SFNS first, even common Chinese chars would
        // trigger a switch to STHeiti Light mid-text, causing baseline mismatch.
        //
        // Monospace: only TRUE monospace fonts — sfns_mono, then menlo, then monaco.
        // STHeiti Light is a proportional font; it must NEVER be in Monospace
        // families or code/indentation alignment will break.

        let stheiti_light_data = std::fs::read("/System/Library/Fonts/STHeiti Light.ttc").ok();
        let sfns_data = std::fs::read("/System/Library/Fonts/SFNS.ttf").ok();
        let sfmono_data = std::fs::read("/System/Library/Fonts/SFNSMono.ttf").ok();

        fonts.families.remove(&FontFamily::Proportional);
        fonts.families.remove(&FontFamily::Monospace);

        if global_monospace {
            if let Some(data) = sfmono_data.clone() {
                let size_kb = data.len() / 1024;
                fonts
                    .font_data
                    .insert("sfns_mono".to_owned(), Arc::new(FontData::from_owned(data)));
                let entry = fonts
                    .families
                    .entry(FontFamily::Proportional)
                    .or_insert_with(Vec::new);
                entry.push("sfns_mono".to_owned());
                info!("Font loaded: SFNSMono.ttf (Monospace primary, {size_kb} KB)");
            } else {
                missing_monospace_warning = true;
                warn!("Font missing: SFNSMono.ttf — global monospace enabled but font not found");
            }
        }
        // Proportional: STHeiti Light (CJK + Latin primary) → SFNS (Latin supplement),
        // then egui built-in emoji fallbacks are appended later.
        if let Some(data) = stheiti_light_data.clone() {
            let size_kb = data.len() / 1024;
            fonts.font_data.insert(
                "stheiti_light".to_owned(),
                Arc::new(FontData::from_owned(data)),
            );
            let entry = fonts
                .families
                .entry(FontFamily::Proportional)
                .or_insert_with(Vec::new);
            entry.push("stheiti_light".to_owned());
            info!("Font loaded: STHeiti Light.ttc (Proportional primary, {size_kb} KB)");
        } else {
            warn!("Font missing: STHeiti Light.ttc — CJK text may not render correctly");
        }
        if let Some(data) = sfns_data.clone() {
            let size_kb = data.len() / 1024;
            fonts
                .font_data
                .insert("sfns".to_owned(), Arc::new(FontData::from_owned(data)));
            let entry = fonts
                .families
                .entry(FontFamily::Proportional)
                .or_insert_with(Vec::new);
            entry.push("sfns".to_owned());
            info!("Font loaded: SFNS.ttf (Proportional supplement, {size_kb} KB)");
        } else {
            warn!("Font missing: SFNS.ttf — Latin supplement unavailable");
        }

        // Monospace: true monospace fonts first, then egui built-in emoji fallbacks
        // are appended later for icon glyphs in buttons.
        if let Some(data) = sfmono_data {
            let size_kb = data.len() / 1024;
            fonts
                .font_data
                .insert("sfns_mono".to_owned(), Arc::new(FontData::from_owned(data)));
            let entry = fonts
                .families
                .entry(FontFamily::Monospace)
                .or_insert_with(Vec::new);
            entry.push("sfns_mono".to_owned());
            info!("Font loaded: SFNSMono.ttf (Monospace primary, {size_kb} KB)");
        } else {
            warn!("Font missing: SFNSMono.ttf — falling back to menlo for code");
            if let Ok(data) = std::fs::read("/System/Library/Fonts/Menlo.ttc") {
                let size_kb = data.len() / 1024;
                fonts
                    .font_data
                    .insert("menlo".to_owned(), Arc::new(FontData::from_owned(data)));
                let entry = fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_insert_with(Vec::new);
                entry.push("menlo".to_owned());
                info!("Font loaded: Menlo.ttc (Monospace fallback, {size_kb} KB)");
            } else if let Ok(data) = std::fs::read("/System/Library/Fonts/Monaco.ttf") {
                let size_kb = data.len() / 1024;
                fonts
                    .font_data
                    .insert("monaco".to_owned(), Arc::new(FontData::from_owned(data)));
                let entry = fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_insert_with(Vec::new);
                entry.push("monaco".to_owned());
                info!("Font loaded: Monaco.ttf (Monospace fallback, {size_kb} KB)");
            }
        }
        if let Some(data) = stheiti_light_data {
            fonts.font_data.insert(
                "stheiti_light".to_owned(),
                Arc::new(FontData::from_owned(data)),
            );
            let entry = fonts
                .families
                .entry(FontFamily::Monospace)
                .or_insert_with(Vec::new);
            entry.push("stheiti_light".to_owned());
        }
        if let Some(data) = sfns_data.clone() {
            let size_kb = data.len() / 1024;
            fonts
                .font_data
                .insert("sfns".to_owned(), Arc::new(FontData::from_owned(data)));
            let entry = fonts
                .families
                .entry(FontFamily::Monospace)
                .or_insert_with(Vec::new);
            entry.push("sfns".to_owned());
            info!("Font loaded: SFNS.ttf (Proportional supplement, {size_kb} KB)");
        }

        let prop_fonts = fonts.families.get(&FontFamily::Proportional);
        let mono_fonts = fonts.families.get(&FontFamily::Monospace);
        info!(
            "Font families: Proportional={:?}, Monospace={:?}",
            prop_fonts, mono_fonts
        );
    }

    #[cfg(target_os = "windows")]
    {
        use tracing::{info, warn};

        // ── Windows ─────────────────────────────────────────────────────────
        // Proportional: Microsoft YaHei for CJK. Monospace: Cascadia Code, with
        // msyh as fallback (CJK glyphs in code blocks when Cascadia is missing).

        const MSYH: &str = r"C:\Windows\Fonts\msyh.ttc";
        const CASCADIA: &str = r"C:\Windows\Fonts\CascadiaCode-Regular.ttf";
        const CONSOLAS: &str = r"C:\Windows\Fonts\consola.ttf";
        const SIMSUN: &str = r"C:\Windows\Fonts\simsun.ttc";
        const NSIMSUN: &str = r"C:\Windows\Fonts\nsimsun.ttc";

        let msyh = std::fs::read(MSYH).ok();
        let cascadia = std::fs::read(CASCADIA).ok();
        let consolas = std::fs::read(CONSOLAS).ok();
        let simsun = std::fs::read(SIMSUN).ok();
        let nsimsun = std::fs::read(NSIMSUN).ok();

        fonts.families.remove(&FontFamily::Proportional);
        fonts.families.remove(&FontFamily::Monospace);

        if global_monospace {
            let prop = fonts.families.entry(FontFamily::Proportional).or_default();
            let mut mono_name = None;

            // Try Cascadia Code first
            if let Some(cc) = &cascadia {
                fonts.font_data.insert(
                    "cascadia".into(),
                    Arc::new(FontData::from_owned(cc.clone())),
                );
                prop.push("cascadia".into());
                mono_name = Some("Cascadia Code");
            } else if let Some(con) = &consolas {
                // Fallback to Consolas
                fonts.font_data.insert(
                    "consolas".into(),
                    Arc::new(FontData::from_owned(con.clone())),
                );
                prop.push("consolas".into());
                mono_name = Some("Consolas");
            }

            if let Some(name) = mono_name {
                info!("Global monospace enabled: using {} as primary font", name);
            } else {
                missing_monospace_warning = true;
                warn!("No suitable monospace font found for global monospace");
            }

            // For global monospace (UI), we use Microsoft YaHei as CJK fallback
            // instead of NSimSun because it has much better baseline alignment
            // with modern Latin fonts, preventing the "high-low drop" issue.
            if let Some(data) = &msyh {
                if !fonts.font_data.contains_key("msyh") {
                    // Tune CJK fallback to better match Cascadia/Consolas line metrics in global
                    // monospace mode and reduce mixed-script "high-low" visual mismatch.
                    fonts.font_data.insert(
                        "msyh".into(),
                        font_data_with_tweak(data.clone(), -0.05, 0.94),
                    );
                }
                prop.push("msyh".into());
                info!(
                    "Global monospace: using Microsoft YaHei as CJK fallback for better baseline alignment"
                );
            }
        } else {
            // Normal mode: Microsoft YaHei is the primary font
            if let Some(data) = &msyh {
                let kb = data.len() / 1024;
                fonts
                    .font_data
                    .insert("msyh".into(), Arc::new(FontData::from_owned(data.clone())));
                fonts
                    .families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .push("msyh".into());
                info!("Normal mode: loaded msyh.ttc (Proportional primary, {kb} KB)");
            } else {
                warn!("Font missing: msyh.ttc — Chinese text may not render correctly");
            }
        }

        // Monospace: use true monospace fonts only, never add proportional fonts like msyh
        // to the Monospace family or code alignment will break.
        let mono = fonts.families.entry(FontFamily::Monospace).or_default();
        if let Some(cc) = &cascadia {
            if !fonts.font_data.contains_key("cascadia") {
                fonts.font_data.insert(
                    "cascadia".into(),
                    Arc::new(FontData::from_owned(cc.clone())),
                );
            }
            mono.push("cascadia".into());
        }
        if let Some(con) = &consolas {
            if !fonts.font_data.contains_key("consolas") {
                fonts.font_data.insert(
                    "consolas".into(),
                    Arc::new(FontData::from_owned(con.clone())),
                );
            }
            mono.push("consolas".into());
        }

        // Add CJK fallbacks for code blocks.
        // We keep true monospace fonts first for ASCII alignment, and place msyh before
        // older SimSun variants so Chinese text in code blocks remains readable.
        if let Some(data) = &msyh {
            if !fonts.font_data.contains_key("msyh") {
                fonts
                    .font_data
                    .insert("msyh".into(), Arc::new(FontData::from_owned(data.clone())));
            }
            mono.push("msyh".into());
        }
        if let Some(ns) = &nsimsun {
            if !fonts.font_data.contains_key("nsimsun") {
                fonts
                    .font_data
                    .insert("nsimsun".into(), Arc::new(FontData::from_owned(ns.clone())));
            }
            mono.push("nsimsun".into());
        } else if let Some(sun) = &simsun {
            if !fonts.font_data.contains_key("simsun") {
                fonts
                    .font_data
                    .insert("simsun".into(), Arc::new(FontData::from_owned(sun.clone())));
            }
            mono.push("simsun".into());
        }

        if mono.is_empty() {
            warn!("No monospace fonts found — code alignment may be broken");
        }
    }

    append_builtin_emoji_fallbacks(&mut fonts);

    ctx.set_fonts(fonts);

    missing_monospace_warning
}
