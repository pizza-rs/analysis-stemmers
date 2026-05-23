<div align="center">

# 🌍 pizza-analysis-stemmers

**Snowball stemmers (33 languages) plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--stemmers-blue)](https://github.com/pizza-rs/analysis-stemmers)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-orange)](LICENSE)

</div>

---


This library bundles several OSS to provide a collection of stemming algorithms for various languages as a Pizza tokenizer. 
[Pizza](https://github.com/pizza-rs/pizza) is a distributed Real-Time Search & AI-Native innovation engine that written in Rust. 
This library originally forked from [tantivy-stemmers](https://github.com/testuj-to/tantivy-stemmers) by testuj-to.

Pizza-stemmers is part of the [Pizza](https://github.com/pizza-rs/pizza) search engine project, visit [Pizza's site](http://pizza.rs) for more details.

This library compiles several OSS projects into 1 library:
- [`snowballstem/snowball`](https://github.com/snowballstem/snowball)

  All the raw algorithms in this library are written in the [Snowball](https://snowballstem.org) language, then complied into a Rust code using the Snowball compiler - all these generated algorithms are located at `src/snowball/algorithms/*`. 
- A Snowball *environment* is then needed to execute the generated algorithm. This environment is comprised of files `src/snowball/among.rs` and `src/snowball/env.rs` - both files have been provided (ie. copied) from the official Snowball repository: [`rust/src/snowball`](https://github.com/snowballstem/snowball/tree/master/rust).

- **Algorithms**

  Most, if not all, stemming algorithms are obtained from the official [Snowball website](https://snowballstem.org) and compiled using the Snowball compiler into Rust. More information about individual algorithm licenses are noted below - most are published under the BSD license.

## Cargo features

As this library bundles many algorithms and contains lots of generated code, it would be nice not to have to include it all in our final build. 
For this reason, each algorithm is published as a Cargo feature. In order to use a specific algorithm, you have to install the appropriate feature first. 
By `default` enabled all algorithms, adjust your own feature on purpose:

```toml
# ...
[dependencies]
pizza-stemmers = { version = "0.1.0", features = ["default"] }
# ...
```

**See the features table under [Supported algorithms](#supported-algorithms) below.**

## Integration with `pizza-analysis-core`

This crate is a sibling of [`pizza-analysis-core`](../analysis-core). Both
register components into the same shared
[`pizza_engine::analysis::AnalysisFactory`]. The app wires them together:

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_core::analyzers::register_all(&mut factory);
pizza_analysis_stemmers::register_all(&mut factory); // adds Snowball stemmers + overrides
```

`pizza_analysis_stemmers::register_all` registers:

- **Named token filters**: `snowball_<lang>` (e.g. `snowball_polish`,
  `snowball_swedish`, `snowball_turkish`) for use in custom analyzer configs.
- **Language analyzers**: full `lowercase + stop + snowball` pipelines for
  `armenian`, `basque`, `catalan`, `estonian`, `lithuanian`, `polish`,
  `swedish`, and `turkish`. These **override** the stop-only fallbacks
  registered by `analysis-core`.

You can also register only the token filters (skip overriding analyzers)
via `pizza_analysis_stemmers::register_token_filters(&mut factory)`.

> **Tip:** Instead of wiring each plugin by hand, use
> [`pizza-analysis-all`](../analysis-all) — an auto-generated meta-crate
> produced by [`pizza-plugin-discovery`](../plugin-discovery) — and call
> `pizza_analysis_all::register_all(&mut factory)` once.

## Supported algorithms

### List of available Cargo features

Certainly! Here's the updated table without the "Default" column:

| Feature                        | Language     | Notes                                                             |
|--------------------------------|--------------|-------------------------------------------------------------------|
| `arabic`                       | Arabic        |                                                                   |
| `armenian_mkrtchyan`           | Armenian      |                                                                   |
| `basque`                       | Basque        |                                                                   |
| `catalan`                      | Catalan       |                                                                   |
| `czech_dolamic_aggressive`     | Czech         |                                                                   |
| `czech_dolamic_light`          | Czech         |                                                                   |
| `danish`                       | Danish        |                                                                   |
| `dutch`                        | Dutch         |                                                                   |
| `english_lovins`               | English       |                                                                   |
| `english_porter`               | English       | *Porter* has been deprecated in favour of *Porter 2*             |
| `english_porter_2`             | English       |                                                                   |
| `estonian_freienthal`          | Estonian      |                                                                   |
| `finnish`                      | Finnish       |                                                                   |
| `french`                       | French        |                                                                   |
| `german`                       | German        |                                                                   |
| `greek`                        | Greek         |                                                                   |
| `hindi_lightweight`            | Hindi         |                                                                   |
| `hungarian`                    | Hungarian     |                                                                   |
| `indonesian_tala`              | Indonesian    |                                                                   |
| `irish_gaelic`                 | Irish         |                                                                   |
| `italian`                      | Italian       |                                                                   |
| `lithuanian_jocas`             | Lithuanian    |                                                                   |
| `nepali`                       | Nepali        |                                                                   |
| `norwegian_bokmal`             | Norwegian     |                                                                   |
| `polish_yarovoy`               | Polish        | Non-Snowball algorithm                                            |
| `polish_yarovoy_unaccented`    | Polish        | Non-Snowball algorithm; removes accents                           |
| `portuguese`                   | Portuguese    |                                                                   |
| `romanian_heidelberg`          | Romanian      |                                                                   |
| `romanian_tirdea`              | Romanian      |                                                                   |
| `romanian`                     | Romanian      |                                                                   |
| `russian`                      | Russian       |                                                                   |
| `spanish`                      | Spanish       |                                                                   |
| `swedish`                      | Swedish       |                                                                   |
| `turkish_cilden`               | Turkish       |                                                                   |
| `yiddish_urieli`               | Yiddish       |                                                                   |


### Notes on individual algorithms and their sources

- **Arabic**

  The Arabic Snowball algorithm was developed by **Assem Chelli** and **Abdelkrim Aries**. Its source code has been obtained under the BSD license from the official [Snowball GitHub repository](https://github.com/snowballstem/snowball/blob/master/algorithms/arabic.sbl).

- **Armenian**

  The Armenian Snowball algorithm was developed by **Astghik Mkrtchyan** and source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/armenian/stemmer.html).

- **Basque**

  The Basque Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/basque/stemmer.html).

- **Catalan**

  The Catalan Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/catalan/stemmer.html).

- **Czech**

  Currently only a single algorithm (in an `aggressive` and `light` variants) is available: `Dolamic`. This algorithm has been developed by **Ljiljana Dolamic** & Jacques Savoy and published under the BSD license. It's written in the [Snowball language](https://snowballstem.org/) and is available on the [Snowball website](https://snowballstem.org/algorithms/czech/stemmer.html).

  *There is 1 more stemming algorithm for the Czech language: `Hellebrand`. This algorithm has been developed by **David Hellebrand** & Petr Chmelař. It's also written in the Snowball language and is available as a [Master's thesis here](https://www.fit.vut.cz/research/product/133). However, this algorithm has been published under the GNU license and **is therefore not included in this library as we'd like to keep the BSD license** on this library. (If you wish, you can always compile the `Hellebrand` algorithm from Snowball to Rust and include it yourself.)*

- **Danish**

  The Danish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/danish/stemmer.html).

- **Dutch**

  The Dutch Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/dutch/stemmer.html).

- **English**

  Three english algorithms in Snowball are available from the official Snowball website - the Porter, Porter 2 and Lovins. (At least) the first two algorithms have been developed by **Dr. Martin Porter**. **The Porter algorithm (original) is used as a default algorithm in this library.** If you wish, you can specify to use the newer Porter 2 algorithm (`Algorithm::EnglishPorter2`) or the Lovins algorithm (`Algorithm::EnglishLovins`).

- **Estonian**

  The Estonian Snowball algorithm was developed by **Linda Freienthal** in 2019 and obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/estonian/stemmer.html).

- **Finnish**

  The Finnish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/finnish/stemmer.html).

- **French**

  The French Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/french/stemmer.html).

- **German**

  The German Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/german/stemmer.html).

- **Greek**

  The Greek Snowball algorithm has been developed by **Georgios Ntais** in 2006 and later enhanced by **Spyridon Saroukos** in 2008. The source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/greek/stemmer.html).

- **Hindi**

  The Hindi (lightweight) Snowball algorithm was developed by **A. Ramanathan** and **D. Rao** in 2003. Its source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/hindi/stemmer.html).

- **Hungarian**

  The Hungarian Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/hungarian/stemmer.html).

- **Indonesian**

  The Indonesian Snowball algorithm was developed by **Fadillah Z. Tala** in 2003 and source codes has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/indonesian/stemmer.html).

- **Irish (Gaelic)**

  The Irish (Gaelic) Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/irish/stemmer.html).

- **Italian**

  The Italian Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/italian/stemmer.html).

- **Lithuanian**

  The Lithuanian Snowball algorithm (`LithuanianJocas`) was contributed by Dainius Jocas. Its source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/lithuanian/stemmer.html).

- **Nepali**

  The Nepali Snowball algorithm (`LithuanianJocas`) was contributed by Dainius Jocas. Its source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/lithuanian/stemmer.html).
  The Nepali Snowball algorithm was developed by **Ingroj Shrestha**, **Oleg Bartunov** and **Shreeya Singh**. Its source code has been obtained under the BSD license from the official [Snowball GitHub repository](https://github.com/snowballstem/snowball/blob/master/algorithms/nepali.sbl).

- **Norwegian (Bokmål)**

  The Norwegian (Bokmål variant) Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/norwegian/stemmer.html).

- **Polish**

  While there are a few distinct stemming algorithms for the Polish language, there's not a single Polish (OSS) stemming algorithm implemented in the Snowball language. Namely, the most popular stemming algorithm [Stempel](http://www.getopt.org/stempel) is implemented in Java. There are also its ports to [Python](https://github.com/dzieciou/pystempel) and [Go](https://github.com/blevesearch/stempel).

  There is 1 Polish stemming algorithm with 2 variants in this library: `polish_yarovoy` and `polish_yarovoy_unaccented`. It has been ported to Rust from a [Go implementation](https://github.com/nickspring/simple_polish_stemmer) by **Nikolay Yarovoy**, which in turn was inspired by [Python implementation](https://github.com/Tutanchamon/pl_stemmer) by **Błażej Kubiński**.<br>
  There are 2 variants of this algorithm: `polish_yarovoy` stems Polish words and leaves accents as are, while the `polish_yarovoy_unaccented` stems Polish words and also removes all the accents.

- **Portuguese**

  The Portuguese Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/portuguese/stemmer.html).

- **Romanian**

  Three Snowball algorithms for the Romainian language are available: `Romanian`, `RomanianHeidelberg` and `RomanianTirdea`. All algorithm were obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/romanian/stemmer.html) and [Snowball website](https://snowballstem.org/otherapps/romanian/).

  The `RomanianHeidelberg` algorithm has been developed in 2006 by **Marina Stegarescu**, **Doina Gliga** and **Erwin Glockner** at the Ruprecht-Karls-University of Heidelberg (Department of Computational Linguistics).

  The `RomanianTirdea` has been developed in 2006 by **Irina Tirdea**.

- **Russian**

  The Russian Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/russian/stemmer.html).

- **Spanish**

  The Spanish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/spanish/stemmer.html).

- **Swedish**

  The Swedish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/swedish/stemmer.html).

- **Turkish**

  The Turkish Snowball algorithm was developed by **Evren (Kapusuz) Çilden** in 2007. The source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/turkish/stemmer.html).

  ##### Note from the Snowball website

  > The Turkish stemming algorithm was provided by Evren Kapusuz Cilden. It stems only noun and nominal verb suffixes because noun stems are more important for information retrieval, and only handling these simplifies the algorithm significantly.

  > In her paper (linked above) Evren explains

  > The stemmer can be enhanced to stem all kinds of verb suffixes. In Turkish, there are over fifty suffixes that can be affixed to verbs [2]. The morphological structure of verb suffixes is more complicated than noun suffixes. Despite this, one can use the methodology presented in this paper to enhance the stemmer to find stems of all kinds of Turkish words.

- **Yiddish**

  The Yiddish Snowball algorithm was created by Assaf Urieli in 2020 and obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/yiddish/stemmer.html).
