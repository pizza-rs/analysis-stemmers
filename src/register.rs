//! Wire Snowball stemmers into a [`pizza_engine::analysis::AnalysisFactory`].
//!
//! This module exposes the Snowball stemmers from this crate as named token
//! filters and full language analyzers.

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use pizza_analysis_core::token_filters::stopwords;
use pizza_analysis_core::LowercaseTokenFilter;
use pizza_analysis_core::StopTokenFilter;

use crate::algorithms;
use crate::StemmerFilter;

/// Register all Snowball stemmers and the language analyzers they enable
/// into the given [`AnalysisFactory`].
pub fn register_all(factory: &mut AnalysisFactory) {
    register_token_filters(factory);
    register_language_analyzers(factory);
}

/// Register each Snowball algorithm as a named token filter
/// (e.g. `snowball_polish`, `snowball_swedish`).
pub fn register_token_filters(factory: &mut AnalysisFactory) {
    macro_rules! register {
        ($name:literal, $algo:path) => {
            factory.register_token_filter($name, Box::new(StemmerFilter::new($algo)));
        };
    }

    #[cfg(feature = "arabic")]
    register!("snowball_arabic", algorithms::arabic);
    #[cfg(feature = "armenian_mkrtchyan")]
    register!("snowball_armenian", algorithms::armenian_mkrtchyan);
    #[cfg(feature = "basque")]
    register!("snowball_basque", algorithms::basque);
    #[cfg(feature = "catalan")]
    register!("snowball_catalan", algorithms::catalan);
    #[cfg(feature = "czech_dolamic_aggressive")]
    register!("snowball_czech", algorithms::czech_dolamic_aggressive);
    #[cfg(feature = "czech_dolamic_light")]
    register!("snowball_czech_light", algorithms::czech_dolamic_light);
    #[cfg(feature = "danish")]
    register!("snowball_danish", algorithms::danish);
    #[cfg(feature = "dutch")]
    register!("snowball_dutch", algorithms::dutch);
    #[cfg(feature = "english_porter_2")]
    register!("snowball_english", algorithms::english_porter_2);
    #[cfg(feature = "english_porter")]
    register!("snowball_english_porter", algorithms::english_porter);
    #[cfg(feature = "english_lovins")]
    register!("snowball_english_lovins", algorithms::english_lovins);
    #[cfg(feature = "estonian_freienthal")]
    register!("snowball_estonian", algorithms::estonian_freienthal);
    #[cfg(feature = "finnish")]
    register!("snowball_finnish", algorithms::finnish);
    #[cfg(feature = "french")]
    register!("snowball_french", algorithms::french);
    #[cfg(feature = "german")]
    register!("snowball_german", algorithms::german);
    #[cfg(feature = "greek")]
    register!("snowball_greek", algorithms::greek);
    #[cfg(feature = "hindi_lightweight")]
    register!("snowball_hindi", algorithms::hindi_lightweight);
    #[cfg(feature = "hungarian")]
    register!("snowball_hungarian", algorithms::hungarian);
    #[cfg(feature = "indonesian_tala")]
    register!("snowball_indonesian", algorithms::indonesian_tala);
    #[cfg(feature = "irish_gaelic")]
    register!("snowball_irish", algorithms::irish_gaelic);
    #[cfg(feature = "italian")]
    register!("snowball_italian", algorithms::italian);
    #[cfg(feature = "lithuanian_jocas")]
    register!("snowball_lithuanian", algorithms::lithuanian_jocas);
    #[cfg(feature = "nepali")]
    register!("snowball_nepali", algorithms::nepali);
    #[cfg(feature = "norwegian_bokmal")]
    register!("snowball_norwegian", algorithms::norwegian_bokmal);
    #[cfg(feature = "polish_yarovoy")]
    register!("snowball_polish", algorithms::polish_yarovoy);
    #[cfg(feature = "polish_yarovoy_unaccented")]
    register!(
        "snowball_polish_unaccented",
        algorithms::polish_yarovoy_unaccented
    );
    #[cfg(feature = "portuguese")]
    register!("snowball_portuguese", algorithms::portuguese);
    #[cfg(feature = "romanian")]
    register!("snowball_romanian", algorithms::romanian);
    #[cfg(feature = "russian")]
    register!("snowball_russian", algorithms::russian);
    #[cfg(feature = "spanish")]
    register!("snowball_spanish", algorithms::spanish);
    #[cfg(feature = "swedish")]
    register!("snowball_swedish", algorithms::swedish);
    #[cfg(feature = "turkish_cilden")]
    register!("snowball_turkish", algorithms::turkish_cilden);
    #[cfg(feature = "yiddish_urieli")]
    register!("snowball_yiddish", algorithms::yiddish_urieli);
}

/// Register/override language analyzers for languages where this crate adds a
/// stemmer that `analysis-core` lacks. Each is `lowercase + stop + snowball`.
pub fn register_language_analyzers(factory: &mut AnalysisFactory) {
    #[cfg(feature = "armenian_mkrtchyan")]
    factory.register_analyzer(
        "armenian",
        build("armenian", algorithms::armenian_mkrtchyan),
    );
    #[cfg(feature = "basque")]
    factory.register_analyzer("basque", build("basque", algorithms::basque));
    #[cfg(feature = "catalan")]
    factory.register_analyzer("catalan", build_catalan());
    #[cfg(feature = "estonian_freienthal")]
    factory.register_analyzer(
        "estonian",
        build("estonian", algorithms::estonian_freienthal),
    );
    #[cfg(feature = "lithuanian_jocas")]
    factory.register_analyzer(
        "lithuanian",
        build("lithuanian", algorithms::lithuanian_jocas),
    );
    #[cfg(feature = "polish_yarovoy")]
    factory.register_analyzer("polish", build("polish", algorithms::polish_yarovoy));
    #[cfg(feature = "swedish")]
    factory.register_analyzer("swedish", build_swedish());
    #[cfg(feature = "turkish_cilden")]
    factory.register_analyzer("turkish", build_turkish());
}

/// Build a standard `lowercase + stop + snowball` analyzer.
fn build(lang: &str, algo: algorithms::Algorithm) -> Analyzer {
    let stop = stopwords::get_stop_words(lang).unwrap_or(&[]);
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(LowercaseTokenFilter::new()),
        Box::new(StopTokenFilter::new(stop)),
        Box::new(StemmerFilter::new(algo)),
    ];
    Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters)
}

#[cfg(feature = "catalan")]
fn build_catalan() -> Analyzer {
    use pizza_analysis_core::ElisionTokenFilter;
    let stop = stopwords::get_stop_words("catalan").unwrap_or(&[]);
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(ElisionTokenFilter::new(&["l", "d", "qu", "m", "n", "s"])),
        Box::new(LowercaseTokenFilter::new()),
        Box::new(StopTokenFilter::new(stop)),
        Box::new(StemmerFilter::new(algorithms::catalan)),
    ];
    Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters)
}

#[cfg(feature = "swedish")]
fn build_swedish() -> Analyzer {
    use pizza_analysis_core::ScandinavianFoldingTokenFilter;
    use pizza_analysis_core::ScandinavianNormalizationTokenFilter;
    let stop = stopwords::get_stop_words("swedish").unwrap_or(&[]);
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(LowercaseTokenFilter::new()),
        Box::new(StopTokenFilter::new(stop)),
        Box::new(ScandinavianNormalizationTokenFilter::new()),
        Box::new(ScandinavianFoldingTokenFilter::new()),
        Box::new(StemmerFilter::new(algorithms::swedish)),
    ];
    Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters)
}

#[cfg(feature = "turkish_cilden")]
fn build_turkish() -> Analyzer {
    use pizza_analysis_core::ApostropheTokenFilter;
    use pizza_analysis_core::TurkishLowercaseTokenFilter;
    let stop = stopwords::get_stop_words("turkish").unwrap_or(&[]);
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(ApostropheTokenFilter::new()),
        Box::new(TurkishLowercaseTokenFilter::new()),
        Box::new(StopTokenFilter::new(stop)),
        Box::new(StemmerFilter::new(algorithms::turkish_cilden)),
    ];
    Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters)
}
