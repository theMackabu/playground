pub const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "type",
    "type.builtin",
    "type.enum",
    "type.enum.variant",
    "constructor",
    "constant",
    "constant.builtin",
    "constant.builtin.boolean",
    "constant.character",
    "constant.character.escape",
    "constant.numeric",
    "constant.numeric.integer",
    "constant.numeric.float",
    "string",
    "string.regexp",
    "string.special",
    "string.special.path",
    "string.special.url",
    "string.special.symbol",
    "escape",
    "comment",
    "comment.line",
    "comment.block",
    "comment.block.documentation",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.other",
    "variable.other.member",
    "label",
    "punctuation",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "keyword",
    "keyword.control",
    "keyword.control.conditional",
    "keyword.control.repeat",
    "keyword.control.import",
    "keyword.control.return",
    "keyword.control.exception",
    "keyword.operator",
    "keyword.directive",
    "keyword.function",
    "keyword.storage",
    "keyword.storage.type",
    "keyword.storage.modifier",
    "operator",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "function.special",
    "tag",
    "tag.builtin",
    "namespace",
    "special",
    "markup",
    "markup.heading",
    "markup.heading.marker",
    "markup.heading.1",
    "markup.heading.2",
    "markup.heading.3",
    "markup.heading.4",
    "markup.heading.5",
    "markup.heading.6",
    "markup.list",
    "markup.list.unnumbered",
    "markup.list.numbered",
    "markup.list.checked",
    "markup.list.unchecked",
    "markup.bold",
    "markup.italic",
    "markup.strikethrough",
    "markup.link",
    "markup.link.url",
    "markup.link.label",
    "markup.link.text",
    "markup.quote",
    "markup.raw",
    "markup.raw.inline",
    "markup.raw.block",
    "diff",
    "diff.plus",
    "diff.minus",
    "diff.delta",
    "diff.delta.moved",
];

pub const ACME: &str = include_str!("../themes/acme.toml");
pub const ADWAITA_DARK: &str = include_str!("../themes/adwaita-dark.toml");
pub const AMBERWOOD: &str = include_str!("../themes/amberwood.toml");
pub const AO: &str = include_str!("../themes/ao.toml");
pub const AYU_DARK: &str = include_str!("../themes/ayu_dark.toml");
pub const AYU_LIGHT: &str = include_str!("../themes/ayu_light.toml");
pub const AYU_MIRAGE: &str = include_str!("../themes/ayu_mirage.toml");
pub const BASE16_DEFAULT_DARK: &str = include_str!("../themes/base16_default_dark.toml");
pub const BASE16_DEFAULT_LIGHT: &str = include_str!("../themes/base16_default_light.toml");
pub const BASE16_TERMINAL: &str = include_str!("../themes/base16_terminal.toml");
pub const BASE16_TRANSPARENT: &str = include_str!("../themes/base16_transparent.toml");
pub const BOGSTER: &str = include_str!("../themes/bogster.toml");
pub const BOGSTER_LIGHT: &str = include_str!("../themes/bogster_light.toml");
pub const BOO_BERRY: &str = include_str!("../themes/boo_berry.toml");
pub const CATPPUCCIN_MOCHA: &str = include_str!("../themes/catppuccin_mocha.toml");
pub const CURZON: &str = include_str!("../themes/curzon.toml");
pub const CYAN_LIGHT: &str = include_str!("../themes/cyan_light.toml");
pub const DARCULA: &str = include_str!("../themes/darcula.toml");
pub const DARK_HIGH_CONTRAST: &str = include_str!("../themes/dark_high_contrast.toml");
pub const DARK_PLUS: &str = include_str!("../themes/dark_plus.toml");
pub const DOOM_ACARIO_DARK: &str = include_str!("../themes/doom_acario_dark.toml");
pub const DRACULA: &str = include_str!("../themes/dracula.toml");
pub const DRACULA_AT_NIGHT: &str = include_str!("../themes/dracula_at_night.toml");
pub const EMACS: &str = include_str!("../themes/emacs.toml");
pub const EVERBLUSH: &str = include_str!("../themes/everblush.toml");
pub const EVERFOREST_DARK: &str = include_str!("../themes/everforest_dark.toml");
pub const EVERFOREST_LIGHT: &str = include_str!("../themes/everforest_light.toml");
pub const FERRA: &str = include_str!("../themes/ferra.toml");
pub const FLATWHITE: &str = include_str!("../themes/flatwhite.toml");
pub const FLEET_DARK: &str = include_str!("../themes/fleet_dark.toml");
pub const FLEXOKI_LIGHT: &str = include_str!("../themes/flexoki_light.toml");
pub const GITHUB_DARK: &str = include_str!("../themes/github_dark.toml");
pub const GITHUB_LIGHT: &str = include_str!("../themes/github_light.toml");
pub const GRUBER_DARKER: &str = include_str!("../themes/gruber-darker.toml");
pub const GRUVBOX: &str = include_str!("../themes/gruvbox.toml");
pub const HEISENBERG: &str = include_str!("../themes/heisenberg.toml");
pub const HEX_STEEL: &str = include_str!("../themes/hex_steel.toml");
pub const HORIZON_DARK: &str = include_str!("../themes/horizon-dark.toml");
pub const ICEBERG_DARK: &str = include_str!("../themes/iceberg-dark.toml");
pub const INGRID: &str = include_str!("../themes/ingrid.toml");
pub const IROASETA: &str = include_str!("../themes/iroaseta.toml");
pub const JELLYBEANS: &str = include_str!("../themes/jellybeans.toml");
pub const JETBRAINS_DARK: &str = include_str!("../themes/jetbrains_dark.toml");
pub const KANAGAWA: &str = include_str!("../themes/kanagawa.toml");
pub const KAOLIN_DARK: &str = include_str!("../themes/kaolin-dark.toml");
pub const MATERIAL_DEEP_OCEAN: &str = include_str!("../themes/material_deep_ocean.toml");
pub const MELIORA: &str = include_str!("../themes/meliora.toml");
pub const MELLOW: &str = include_str!("../themes/mellow.toml");
pub const MERIONETTE: &str = include_str!("../themes/merionette.toml");
pub const MODUS_OPERANDI: &str = include_str!("../themes/modus_operandi.toml");
pub const MONOKAI: &str = include_str!("../themes/monokai.toml");
pub const MONOKAI_PRO: &str = include_str!("../themes/monokai_pro.toml");
pub const MONOKAI_PRO_MACHINE: &str = include_str!("../themes/monokai_pro_machine.toml");
pub const MONOKAI_PRO_OCTAGON: &str = include_str!("../themes/monokai_pro_octagon.toml");
pub const MONOKAI_PRO_RISTRETTO: &str = include_str!("../themes/monokai_pro_ristretto.toml");
pub const MONOKAI_PRO_SPECTRUM: &str = include_str!("../themes/monokai_pro_spectrum.toml");
pub const MONOKAI_SODA: &str = include_str!("../themes/monokai_soda.toml");
pub const NAYSAYER: &str = include_str!("../themes/naysayer.toml");
pub const NEW_MOON: &str = include_str!("../themes/new_moon.toml");
pub const NIGHT_OWL: &str = include_str!("../themes/night_owl.toml");
pub const NIGHTFOX: &str = include_str!("../themes/nightfox.toml");
pub const NOCTIS: &str = include_str!("../themes/noctis.toml");
pub const NOCTIS_BORDO: &str = include_str!("../themes/noctis_bordo.toml");
pub const NORD: &str = include_str!("../themes/nord.toml");
pub const NORD_LIGHT: &str = include_str!("../themes/nord_light.toml");
pub const ONEDARK: &str = include_str!("../themes/onedark.toml");
pub const ONEDARKER: &str = include_str!("../themes/onedarker.toml");
pub const ONELIGHT: &str = include_str!("../themes/onelight.toml");
pub const PAPERCOLOR_LIGHT: &str = include_str!("../themes/papercolor-light.toml");
pub const PENUMBRA_PLUS: &str = include_str!("../themes/penumbra-plus.toml");
pub const POIMANDRES: &str = include_str!("../themes/poimandres.toml");
pub const POP_DARK: &str = include_str!("../themes/pop-dark.toml");
pub const RASMUS: &str = include_str!("../themes/rasmus.toml");
pub const ROSE_PINE: &str = include_str!("../themes/rose_pine.toml");
pub const SERIKA_DARK: &str = include_str!("../themes/serika-dark.toml");
pub const SERIKA_LIGHT: &str = include_str!("../themes/serika-light.toml");
pub const SNAZZY: &str = include_str!("../themes/snazzy.toml");
pub const SOLARIZED_DARK: &str = include_str!("../themes/solarized_dark.toml");
pub const SOLARIZED_LIGHT: &str = include_str!("../themes/solarized_light.toml");
pub const SONOKAI: &str = include_str!("../themes/sonokai.toml");
pub const SPACEBONES_LIGHT: &str = include_str!("../themes/spacebones_light.toml");
pub const STARLIGHT: &str = include_str!("../themes/starlight.toml");
pub const TERM16_DARK: &str = include_str!("../themes/term16_dark.toml");
pub const TOKYONIGHT: &str = include_str!("../themes/tokyonight.toml");
pub const TTOX: &str = include_str!("../themes/ttox.toml");
pub const VARUA: &str = include_str!("../themes/varua.toml");
pub const VIM_DARK_HIGH_CONTRAST: &str = include_str!("../themes/vim_dark_high_contrast.toml");
pub const VOXED: &str = include_str!("../themes/voxed.toml");
pub const YELLOWED: &str = include_str!("../themes/yellowed.toml");
pub const ZED_ONEDARK: &str = include_str!("../themes/zed_onedark.toml");
pub const ZENBURN: &str = include_str!("../themes/zenburn.toml");

pub fn from_token(token: impl AsRef<str>) -> Option<&'static str> {
    match token.as_ref().to_lowercase().as_str() {
        "acme" => Some(ACME),
        "adwaita-dark" => Some(ADWAITA_DARK),
        "amberwood" => Some(AMBERWOOD),
        "ao" => Some(AO),
        "ayu-dark" => Some(AYU_DARK),
        "ayu-light" => Some(AYU_LIGHT),
        "ayu-mirage" => Some(AYU_MIRAGE),
        "base16-default-dark" => Some(BASE16_DEFAULT_DARK),
        "base16-default-light" => Some(BASE16_DEFAULT_LIGHT),
        "base16-terminal" => Some(BASE16_TERMINAL),
        "base16-transparent" => Some(BASE16_TRANSPARENT),
        "bogster" => Some(BOGSTER),
        "bogster-light" => Some(BOGSTER_LIGHT),
        "boo-berry" => Some(BOO_BERRY),
        "catppuccin-mocha" => Some(CATPPUCCIN_MOCHA),
        "curzon" => Some(CURZON),
        "cyan-light" => Some(CYAN_LIGHT),
        "darcula" => Some(DARCULA),
        "dark-high-contrast" => Some(DARK_HIGH_CONTRAST),
        "dark-plus" => Some(DARK_PLUS),
        "doom-acario-dark" => Some(DOOM_ACARIO_DARK),
        "dracula" => Some(DRACULA),
        "dracula-at-night" => Some(DRACULA_AT_NIGHT),
        "emacs" => Some(EMACS),
        "everblush" => Some(EVERBLUSH),
        "everforest-dark" => Some(EVERFOREST_DARK),
        "everforest-light" => Some(EVERFOREST_LIGHT),
        "ferra" => Some(FERRA),
        "flatwhite" => Some(FLATWHITE),
        "fleet-dark" => Some(FLEET_DARK),
        "flexoki-light" => Some(FLEXOKI_LIGHT),
        "github-dark" => Some(GITHUB_DARK),
        "github-light" => Some(GITHUB_LIGHT),
        "gruber-darker" => Some(GRUBER_DARKER),
        "gruvbox" => Some(GRUVBOX),
        "heisenberg" => Some(HEISENBERG),
        "hex-steel" => Some(HEX_STEEL),
        "horizon-dark" => Some(HORIZON_DARK),
        "iceberg-dark" => Some(ICEBERG_DARK),
        "ingrid" => Some(INGRID),
        "iroaseta" => Some(IROASETA),
        "jellybeans" => Some(JELLYBEANS),
        "jetbrains-dark" => Some(JETBRAINS_DARK),
        "kanagawa" => Some(KANAGAWA),
        "kaolin-dark" => Some(KAOLIN_DARK),
        "material-deep-ocean" => Some(MATERIAL_DEEP_OCEAN),
        "meliora" => Some(MELIORA),
        "mellow" => Some(MELLOW),
        "merionette" => Some(MERIONETTE),
        "modus-operandi" => Some(MODUS_OPERANDI),
        "monokai" => Some(MONOKAI),
        "monokai-pro" => Some(MONOKAI_PRO),
        "monokai-pro-machine" => Some(MONOKAI_PRO_MACHINE),
        "monokai-pro-octagon" => Some(MONOKAI_PRO_OCTAGON),
        "monokai-pro-ristretto" => Some(MONOKAI_PRO_RISTRETTO),
        "monokai-pro-spectrum" => Some(MONOKAI_PRO_SPECTRUM),
        "monokai-soda" => Some(MONOKAI_SODA),
        "naysayer" => Some(NAYSAYER),
        "new-moon" => Some(NEW_MOON),
        "night-owl" => Some(NIGHT_OWL),
        "nightfox" => Some(NIGHTFOX),
        "noctis" => Some(NOCTIS),
        "noctis-bordo" => Some(NOCTIS_BORDO),
        "nord" => Some(NORD),
        "nord-light" => Some(NORD_LIGHT),
        "onedark" => Some(ONEDARK),
        "onedarker" => Some(ONEDARKER),
        "onelight" => Some(ONELIGHT),
        "papercolor-light" => Some(PAPERCOLOR_LIGHT),
        "penumbra-plus" => Some(PENUMBRA_PLUS),
        "poimandres" => Some(POIMANDRES),
        "pop-dark" => Some(POP_DARK),
        "rasmus" => Some(RASMUS),
        "rose-pine" => Some(ROSE_PINE),
        "serika-dark" => Some(SERIKA_DARK),
        "serika-light" => Some(SERIKA_LIGHT),
        "snazzy" => Some(SNAZZY),
        "solarized-dark" => Some(SOLARIZED_DARK),
        "solarized-light" => Some(SOLARIZED_LIGHT),
        "sonokai" => Some(SONOKAI),
        "spacebones-light" => Some(SPACEBONES_LIGHT),
        "starlight" => Some(STARLIGHT),
        "term16-dark" => Some(TERM16_DARK),
        "tokyonight" => Some(TOKYONIGHT),
        "ttox" => Some(TTOX),
        "varua" => Some(VARUA),
        "vim-dark-high-contrast" => Some(VIM_DARK_HIGH_CONTRAST),
        "voxed" => Some(VOXED),
        "yellowed" => Some(YELLOWED),
        "zed-onedark" => Some(ZED_ONEDARK),
        "zenburn" => Some(ZENBURN),
        _ => None,
    }
}
