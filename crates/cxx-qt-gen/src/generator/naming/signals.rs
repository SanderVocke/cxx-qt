// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::naming::CombinedIdent;
use crate::parser::signals::ParsedSignal;
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

/// Names for parts of a Q_SIGNAL
pub struct QSignalName {
    pub enum_name: Ident,
    pub name: CombinedIdent,
    pub emit_name: CombinedIdent,
    pub connect_name: CombinedIdent,
    pub on_name: Ident,
}

impl From<&ParsedSignal> for QSignalName {
    fn from(signal: &ParsedSignal) -> Self {
        // Check if there is a cxx ident that should be used
        let cxx_ident = if let Some(cxx_name) = &signal.cxx_name {
            format_ident!("{}", cxx_name)
        } else {
            signal.ident.clone()
        };

        Self {
            enum_name: signal.ident.clone(),
            name: CombinedIdent::from_signal(&signal.ident, &cxx_ident),
            emit_name: CombinedIdent::emit_from_signal(&signal.ident, &cxx_ident),
            connect_name: CombinedIdent::connect_from_signal(&signal.ident, &cxx_ident),
            on_name: on_from_signal(&signal.ident),
        }
    }
}

fn on_from_signal(ident: &Ident) -> Ident {
    format_ident!("on_{}", ident.to_string().to_case(Case::Snake))
}

impl CombinedIdent {
    /// For a given signal ident generate the Rust and C++ names
    fn from_signal(ident: &Ident, cxx_ident: &Ident) -> Self {
        Self {
            cpp: format_ident!("{}", cxx_ident.to_string().to_case(Case::Camel)),
            // Note that signal names are in camel case so we need to convert to snake and can't clone
            rust: format_ident!("{}", ident.to_string().to_case(Case::Snake)),
        }
    }

    /// For a given signal ident generate the Rust and C++ emit name
    fn emit_from_signal(ident: &Ident, cxx_ident: &Ident) -> Self {
        Self {
            cpp: format_ident!("emit{}", cxx_ident.to_string().to_case(Case::Pascal)),
            rust: format_ident!("emit_{}", ident.to_string().to_case(Case::Snake)),
        }
    }

    fn connect_from_signal(ident: &Ident, cxx_ident: &Ident) -> Self {
        Self {
            // Use signalConnect instead of onSignal here so that we don't
            // create a C++ name that is similar to the QML naming scheme for signals
            cpp: format_ident!("{}Connect", cxx_ident.to_string().to_case(Case::Camel)),
            rust: format_ident!("connect_{}", ident.to_string().to_case(Case::Snake)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_signal() {
        let qsignal = ParsedSignal {
            ident: format_ident!("DataChanged"),
            parameters: vec![],
            cxx_name: None,
            inherit: false,
        };

        let names = QSignalName::from(&qsignal);
        assert_eq!(names.enum_name, format_ident!("DataChanged"));
        assert_eq!(names.name.cpp, format_ident!("dataChanged"));
        assert_eq!(names.name.rust, format_ident!("data_changed"));
        assert_eq!(names.emit_name.cpp, format_ident!("emitDataChanged"));
        assert_eq!(names.emit_name.rust, format_ident!("emit_data_changed"));
        assert_eq!(names.connect_name.cpp, format_ident!("dataChangedConnect"));
        assert_eq!(
            names.connect_name.rust,
            format_ident!("connect_data_changed")
        );
        assert_eq!(names.on_name, format_ident!("on_data_changed"));
    }

    #[test]
    fn test_parsed_signal_existing_cxx_name() {
        let qsignal = ParsedSignal {
            ident: format_ident!("ExistingSignal"),
            parameters: vec![],
            cxx_name: Some("baseName".to_owned()),
            inherit: true,
        };

        let names = QSignalName::from(&qsignal);
        assert_eq!(names.enum_name, format_ident!("ExistingSignal"));
        assert_eq!(names.name.cpp, format_ident!("baseName"));
        assert_eq!(names.name.rust, format_ident!("existing_signal"));
        assert_eq!(names.emit_name.cpp, format_ident!("emitBaseName"));
        assert_eq!(names.emit_name.rust, format_ident!("emit_existing_signal"));
        assert_eq!(names.connect_name.cpp, format_ident!("baseNameConnect"));
        assert_eq!(
            names.connect_name.rust,
            format_ident!("connect_existing_signal")
        );
        assert_eq!(names.on_name, format_ident!("on_existing_signal"));
    }
}
