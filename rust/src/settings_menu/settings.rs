use gdnative::prelude::*;
use gdnative::api::*;

pub(super) static DEFAULT_SETTINGS: &[GameSetting] = &[
    GameSetting::WindowMaximized(false),
    GameSetting::WindowSize(1280, 720),
    GameSetting::SkillOverlayMode(SkillOverlayMode::default()),
    GameSetting::Language(Language::default()),
    GameSetting::TargetFramerate(60),
    GameSetting::DialogueTextSpeed(100),
    GameSetting::Vsync(true),
    GameSetting::MainVolume(50),
    GameSetting::MusicVolume(50),
    GameSetting::SfxVolume(50),
    GameSetting::VoiceVolume(50),
];

#[derive(Clone, Copy, Debug, FromVariant, ToVariant)]
pub enum SkillOverlayMode {
    Auto { delay_ms: i64 },
    WaitForInput,
}

pub(super) static ALL_OVERLAY_MODES: &[SkillOverlayMode] = &[
    SkillOverlayMode::Auto { delay_ms: 3000 },
    SkillOverlayMode::WaitForInput,
];

impl Default for SkillOverlayMode { fn default() -> Self { SkillOverlayMode::Auto { delay_ms: 3000 } } }

impl SkillOverlayMode {
    pub(super) fn display_name(&self) -> &'static str {
        match self {
            SkillOverlayMode::Auto { .. } => "Auto",
            SkillOverlayMode::WaitForInput => "Wait for Input",
        }
    }

    pub(super) fn index(&self) -> i64 {
        match self {
            SkillOverlayMode::Auto { .. } => 0,
            SkillOverlayMode::WaitForInput => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, FromVariant, ToVariant)]
pub enum Language {
    English
}

pub static ALL_LANGUAGES: &[Language] = &[Language::English];

impl Default for Language { fn default() -> Self { Language::English } }

impl Language {
    pub(super) fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
        }
    }

    pub(super) fn index(&self) -> i64 {
        match self {
            Language::English => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, FromVariant, ToVariant)]
pub enum GameSetting {
    WindowMaximized(bool),
    WindowSize(i32, i32),
    SkillOverlayMode(SkillOverlayMode),
    Language(Language),
    TargetFramerate(i64),
    DialogueTextSpeed(i32),
    Vsync(bool),
    MainVolume(i32),
    MusicVolume(i32),
    SfxVolume(i32),
    VoiceVolume(i32),
}

impl GameSetting {
    pub fn key(&self) -> &'static str {
        match self {
            GameSetting::WindowMaximized  (_) => "window_maximized",
            GameSetting::WindowSize       (_, _) => "resolution",
            GameSetting::SkillOverlayMode (_) => "skill_overlay_mode",
            GameSetting::Language         (_) => "language",
            GameSetting::TargetFramerate  (_) => "target_framerate",
            GameSetting::DialogueTextSpeed(_) => "dialogue_text_speed",
            GameSetting::Vsync            (_) => "vsync",
            GameSetting::MainVolume       (_) => "main_volume",
            GameSetting::MusicVolume      (_) => "music_volume",
            GameSetting::SfxVolume        (_) => "sfx_volume",
            GameSetting::VoiceVolume      (_) => "voice_volume",
        }
    }

    pub const fn default_value(variant: &GameSetting) -> GameSetting {
        match variant {
            GameSetting::WindowMaximized  (_) => GameSetting::WindowMaximized(false),
            GameSetting::WindowSize       (_, _) => GameSetting::WindowSize(1280, 720),
            GameSetting::SkillOverlayMode (_) => GameSetting::SkillOverlayMode(SkillOverlayMode::default()),
            GameSetting::Language         (_) => GameSetting::Language(Language::default()),
            GameSetting::TargetFramerate  (_) => GameSetting::TargetFramerate(60),
            GameSetting::DialogueTextSpeed(_) => GameSetting::DialogueTextSpeed(100),
            GameSetting::Vsync            (_) => GameSetting::Vsync(true),
            GameSetting::MainVolume       (_) => GameSetting::MainVolume(50),
            GameSetting::MusicVolume      (_) => GameSetting::MusicVolume(50),
            GameSetting::SfxVolume        (_) => GameSetting::SfxVolume(50),
            GameSetting::VoiceVolume      (_) => GameSetting::VoiceVolume(50),
        }
    }

    pub fn get_saved<T>(key: &str, default: T) -> T where T: FromVariant + ToVariant {
        let config = ConfigFile::new();
        if let Err(error) = config.load(crate::config_path) {
            godot_warn!("Failed to load config file: {}", error);
            return default;
        }

        let result = config.get_value("player_prefs", key, default);
        match result.try_to::<T>() {
            Ok(ok) => return ok,
            Err(error) => {
                godot_warn!("Failed converting: {result} into {t}, in setting: {key}, returning default.\n Error: {error}", t = std::any::type_name::<T>());
                return default;
            },
        }
    }
}