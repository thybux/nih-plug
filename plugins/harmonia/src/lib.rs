use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

// Définition des paramètres du plugin
#[derive(Params)]
struct TrackGenPluginParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    // Paramètres de base
    #[id = "preset"]
    pub preset: EnumParam<PresetChoice>,

    #[id = "track_style"]
    pub track_style: EnumParam<TrackStyle>,

    #[id = "track_type"]
    pub track_type: EnumParam<TrackType>,

    // Paramètre de bypass
    #[id = "bypass"]
    pub bypass: BoolParam,
}

// Définition des options pour les menus déroulants
#[derive(Enum, PartialEq)]
enum PresetChoice {
    #[name = "No preset"]
    NoPreset,
    // Ajoutez d'autres presets ici
}

#[derive(Enum, PartialEq)]
enum TrackStyle {
    #[name = "Track"]
    Default,
    // Ajoutez d'autres styles ici
}

#[derive(Enum, PartialEq)]
enum TrackType {
    #[name = "Type"]
    Default,
    // Ajoutez d'autres types ici
}

impl Default for TrackGenPluginParams {
    fn default() -> Self {
        Self {
            editor_state: Arc::new(ViziaState::new(|| (600, 300))),

            preset: EnumParam::new("Preset", PresetChoice::NoPreset),
            track_style: EnumParam::new("Style", TrackStyle::Default),
            track_type: EnumParam::new("Type", TrackType::Default),

            bypass: BoolParam::new("Bypass", false)
                .with_value_to_string(formatters::v2s_bool_bypass()),
        }
    }
}

struct TrackGenPlugin {
    params: Arc<TrackGenPluginParams>,
}

impl Default for TrackGenPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(TrackGenPluginParams::default()),
        }
    }
}

impl Plugin for TrackGenPlugin {
    const NAME: &'static str = "Track Generator";
    const VENDOR: &'static str = "YourName";
    const URL: &'static str = "https://your-website.com";
    const EMAIL: &'static str = "your.email@example.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // Définir les entrées/sorties audio (stereo)
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_vizia_editor(
            self.params.editor_state.clone(),
            ViziaTheming::Custom,
            |cx, params| {
                // Thème et polices
                assets::register_noto_sans_light(cx);

                // Interface principale
                VStack::new(cx, |cx| {
                    // Barre supérieure avec présets et contrôles
                    HStack::new(cx, |cx| {
                        // Menu de preset
                        Dropdown::new(
                            cx,
                            // Utiliser le binding Lens à la place
                            params.make_lens(|params| &params.preset),
                            |cx, lens| Label::new(cx, lens.map(|p| p.to_string())),
                        )
                        .width(Pixels(160.0));

                        // Bouton New
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "+ New"))
                            .background_color(Color::rgb(80, 80, 80))
                            .width(Pixels(80.0));

                        // Bouton Parameter
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "Parameter"))
                            .background_color(Color::rgb(80, 80, 80))
                            .width(Pixels(120.0));

                        // Label input/output
                        Label::new(cx, "2 in 2 out")
                            .width(Pixels(120.0))
                            .background_color(Color::rgb(80, 80, 80));

                        // Bouton rond (placeholder)
                        Element::new(cx)
                            .background_color(Color::rgb(200, 50, 100))
                            .border_radius(Pixels(20.0))
                            .height(Pixels(40.0))
                            .width(Pixels(40.0));

                        // Bouton ON
                        Button::new(
                            cx,
                            |_| {
                                // Utiliser le système d'événements de paramètre de NIH-plug
                                params.bypass.set_parameter(|v| !v);
                            },
                            |cx| {
                                Label::new(
                                    cx,
                                    params.map(|p| if p.bypass.value() { "OFF" } else { "ON" }),
                                )
                            },
                        )
                        .background_color(Color::rgb(200, 50, 100))
                        .width(Pixels(80.0));
                    })
                    .height(Pixels(50.0))
                    .background_color(Color::rgb(30, 30, 30))
                    .child_space(Stretch(1.0));

                    // Espace central
                    HStack::new(cx, |cx| {
                        // Les trois menus déroulants
                        Dropdown::new(
                            cx,
                            params.make_lens(|params| &params.track_style),
                            |cx, lens| Label::new(cx, lens.map(|p| p.to_string())),
                        )
                        .width(Pixels(140.0))
                        .margin_right(Pixels(10.0));

                        Dropdown::new(
                            cx,
                            params.make_lens(|params| &params.track_type),
                            |cx, lens| Label::new(cx, lens.map(|p| p.to_string())),
                        )
                        .width(Pixels(140.0));

                        // Espace flexible au milieu
                        Element::new(cx).width(Stretch(1.0));

                        // Bouton Generate
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "Generate"))
                            .background_color(Color::rgb(80, 80, 80))
                            .width(Pixels(150.0))
                            .height(Pixels(50.0));
                    })
                    .height(Pixels(250.0))
                    .background_color(Color::rgb(20, 20, 20))
                    .child_top(Pixels(20.0))
                    .child_left(Pixels(20.0))
                    .child_right(Pixels(20.0));
                })
                .background_color(Color::rgb(20, 20, 20))
                .row_between(Pixels(0.0));
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Si le bypass est activé, ne rien faire
        if self.params.bypass.value() {
            return ProcessStatus::Normal;
        }

        // Ici, vous ajouterez votre traitement audio
        // Pour le moment, nous laissons l'audio inchangé

        ProcessStatus::Normal
    }
}

impl ClapPlugin for TrackGenPlugin {
    const CLAP_ID: &'static str = "com.your-name.track-generator";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Track Generator Plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for TrackGenPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"TrackGenPluginXX";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(TrackGenPlugin);
nih_export_vst3!(TrackGenPlugin);

