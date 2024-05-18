use strum_macros::{Display as EnumDisplay, EnumIter, EnumMessage, EnumString};
use gettextrs::gettext;

#[derive(Clone, Copy, Default, PartialEq, EnumString, EnumDisplay, EnumIter)]
pub enum SessionType {
    #[default]
    Simple,
    Advanced,
    Custom,
}

impl SessionType {
    pub fn ui_string(&self) -> String {
        match self {
            SessionType::Simple => gettext("Simple"),
            SessionType::Advanced => gettext("Advanced"),
            SessionType::Custom => gettext("Custom"),
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, EnumString, EnumDisplay, EnumIter)]
pub enum SessionDuration {
    #[default]
    Sec15,
    Sec30,
    Min1,
    Min5,
    Min10,
}

impl SessionDuration {
    pub fn ui_string(&self) -> String {
        match self {
            SessionDuration::Sec15 => gettext("15 seconds"),
            SessionDuration::Sec30 => gettext("30 seconds"),
            SessionDuration::Min1 => gettext("1 minute"),
            SessionDuration::Min5 => gettext("5 minutes"),
            SessionDuration::Min10 => gettext("10 minutes"),
        }
    }
}

#[derive(Clone, Copy, Default, EnumDisplay, EnumString, EnumIter, EnumMessage, PartialEq)]
pub enum Language {
    #[default]
    #[strum(message = "English (US)", to_string = "en_US")]
    EnglishUS,
    #[strum(message = "Norsk bokmål", to_string = "nb_NO")]
    NorwegianBokmaal,
    #[strum(message = "Norsk nynorsk", to_string = "nn_NO")]
    NorwegianNynorsk,
    #[strum(message = "Español", to_string = "es_ES")]
    Spanish,
    #[strum(message = "Svenska", to_string = "se_SE")]
    Swedish,
}
