use bevy::reflect::TypePath;

use super::script::*;
use crate::data::*;
use crate::prelude::*;

/// Sprite Animation Asset type
///
/// Would typically be loaded from TOML files.
#[derive(Asset, Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(TypePath)]
pub struct SpriteAnimation {
    /// Any customization configs
    #[serde(default)]
    pub config: ScriptConfig,
    /// General animation parameters
    pub settings: ExtendedScriptSettings<SpriteAnimationSettings>,
    /// Optional frame bookmarks to help during scripting
    #[serde(default)]
    pub frame_bookmarks: HashMap<String, u32>,
    /// Optional "script": list of actions to perform during playback
    #[serde(default)]
    pub script: Vec<
        ExtendedScript<
            SpriteAnimationScriptParams,
            SpriteAnimationScriptRunIf,
            SpriteAnimationScriptAction,
        >,
    >,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SpriteAnimationSettings {
    pub atlas_asset_key: Option<String>,
    pub image_asset_key: Option<String>,
    pub ticks_per_frame: u32,
    pub frame_start: u32,
    pub frame_min: u32,
    pub frame_max: u32,
    #[serde(default)]
    pub play_reversed: bool,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SpriteAnimationScriptParams {
    pub frame_bookmark: Option<String>,
    pub if_frame_lt: Option<FrameIndexOrBookmark>,
    pub if_frame_le: Option<FrameIndexOrBookmark>,
    pub if_frame_gt: Option<FrameIndexOrBookmark>,
    pub if_frame_ge: Option<FrameIndexOrBookmark>,
    pub if_frame_is: Option<OneOrMany<FrameIndexOrBookmark>>,
    pub if_playing_reversed: Option<bool>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum SpriteAnimationScriptRunIf {
    #[serde(rename = "run_at_frame")]
    Frame(OneOrMany<FrameIndexOrBookmark>),
    #[serde(rename = "run_every_n_frames")]
    FrameQuant(Quant),
}

/// The various actions that can be performed from an animation script
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum SpriteAnimationScriptAction {
    /// Change playback speed
    SetTicksPerFrame {
        /// The new frame rate
        ticks_per_frame: u32,
    },
    /// Immediately change to the given frame, without waiting for `ticks_per_frame`
    SetFrameNow {
        /// Use this bookmark
        to_frame_bookmark: Option<String>,
        /// The frame index
        frame_index: Option<u32>,
    },
    /// Change the next frame to be displayed, after `ticks_per_frame` elapses.
    SetFrameNext {
        /// Use this bookmark
        to_frame_bookmark: Option<String>,
        /// The frame index
        frame_index: Option<u32>,
    },
    /// Set sprite colorization
    SetSpriteColor {
        /// The new sprite color
        color: ColorRepr,
    },
    /// Set sprite X/Y flip
    SetSpriteFlip {
        /// Set flip on the X axis
        flip_x: Option<bool>,
        /// Set flip on the Y axis
        flip_y: Option<bool>,
    },
    /// Reverse Playback
    ReversePlayback {
        /// Set a specific direction.
        /// If omitted, toggles the current direction.
        reversed: Option<bool>,
    },
    /// Transform: relative translation
    TransformMove {
        x: Option<Frac>,
        y: Option<Frac>,
        z: Option<Frac>,
    },
    /// Transform: set absolute translation
    TransformTeleport { x: Frac, y: Frac, z: Option<Frac> },
    /// Transform: rotate by N turns (1 turn = 360 degrees)
    TransformRotateTurns { turns: Frac },
    /// Transform: rotate by N degrees
    TransformRotateDegrees { degrees: Frac },
    /// Transform: set the rotation to a specific value
    TransformSetRotationTurns { turns: Frac },
    /// Transform: set the rotation to a specific value
    TransformSetRotationDegrees { degrees: Frac },
    /// Transform: set scale
    TransformSetScale { x: Frac, y: Frac },
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FrameIndexOrBookmark {
    Index(u32),
    Bookmark(String),
}

impl SpriteAnimation {
    pub fn resolve_image_atlas(
        &self,
        preloaded: &PreloadedAssets,
        anim_key: Option<&str>,
    ) -> Option<(Handle<Image>, Handle<TextureAtlasLayout>)> {
        let mut default_image_key;
        let image_key = if let Some(key) = &self.settings.extended.image_asset_key {
            key
        } else {
            default_image_key = anim_key?.to_owned();
            default_image_key.push_str(".image");
            &default_image_key
        };
        let mut default_layout_key;
        let layout_key = if let Some(key) = &self.settings.extended.atlas_asset_key {
            key
        } else {
            default_layout_key = anim_key?.to_owned();
            default_layout_key.push_str(".atlas");
            &default_layout_key
        };
        Some((
            preloaded.get_single_asset(image_key)?,
            preloaded.get_single_asset(layout_key)?,
        ))
    }
}
