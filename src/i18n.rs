/*
Git Broom
Copyright (C) 2023  All contributors.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use fluent::{FluentArgs, FluentBundle, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use rust_embed::RustEmbed;
use sys_locale::get_locale;
use unic_langid::{langid, LanguageIdentifier};

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct LocalizationAsset;

pub struct Localization {
    bundle: FluentBundle<FluentResource>,
}

impl Localization {
    pub fn new() -> Self {
        Self {
            bundle: Self::init(),
        }
    }

    pub fn get_message_with_one_arg(
        &self,
        id: &str,
        arg_name: String,
        arg_value: String,
    ) -> String {
        let mut args = FluentArgs::new();
        args.set(arg_name, arg_value);
        self.get_message_with_args(id, args)
    }

    pub fn get_message_with_count(&self, id: &str, count: usize) -> String {
        let mut args = FluentArgs::new();
        args.set("count", count);
        self.get_message_with_args(id, args)
    }

    pub fn get_message_with_count_and_one_arg(
        &self,
        id: &str,
        count: usize,
        arg_name: String,
        arg_value: String,
    ) -> String {
        let mut args = FluentArgs::new();
        args.set("count", count);
        args.set(arg_name, arg_value);
        self.get_message_with_args(id, args)
    }

    pub fn get_message(&self, id: &str) -> String {
        if let Some(message) = self.bundle.get_message(id) {
            if let Some(pattern) = message.value() {
                let mut errors = vec![];

                self.bundle
                    .format_pattern(pattern, None, &mut errors)
                    .to_string()
            } else {
                String::from(id)
            }
        } else {
            String::from(id)
        }
    }

    fn get_message_with_args(&self, id: &str, args: FluentArgs) -> String {
        if let Some(message) = self.bundle.get_message(id) {
            if let Some(pattern) = message.value() {
                let mut errors = vec![];

                self.bundle
                    .format_pattern(pattern, Some(&args), &mut errors)
                    .to_string()
            } else {
                String::from(id)
            }
        } else {
            String::from(id)
        }
    }

    fn init() -> FluentBundle<FluentResource> {
        let default_locale = langid!("en-US");
        let available = Self::get_available_locales();

        let requested: Vec<LanguageIdentifier> = vec![get_locale()
            .unwrap_or_else(|| String::from("en-US"))
            .as_str()
            .parse::<LanguageIdentifier>()
            .unwrap_or_else(|_| langid!("en-US"))];

        let resolved_locales = negotiate_languages(
            &requested,
            &available,
            Some(&default_locale),
            NegotiationStrategy::Filtering,
        );

        let selected_locale = resolved_locales
            .get(0)
            .cloned()
            .unwrap_or_else(|| &default_locale);

        let data = LocalizationAsset::get(&format!("{}.ftl", selected_locale.to_string())).unwrap();
        let str_data = String::from_utf8_lossy(data.data.as_ref());

        let mut bundle = FluentBundle::new(resolved_locales.into_iter().cloned().collect());
        bundle.set_use_isolating(false);

        if let Ok(resource) = FluentResource::try_new(str_data.parse().unwrap()) {
            bundle.add_resource(resource).unwrap_or_default();
        }

        bundle
    }

    fn get_available_locales() -> Vec<LanguageIdentifier> {
        let mut locales: Vec<LanguageIdentifier> = Vec::new();
        for file in LocalizationAsset::iter() {
            let mut locale = String::from(file.as_ref());
            locale.truncate(locale.len() - 4);
            locales.push(locale.as_str().parse().unwrap_or_else(|_| langid!("en-US")));
        }

        locales
    }
}
