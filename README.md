<div align="center">

# ✂️ pizza-analysis-stemmers

**Snowball stemmers (33 languages) plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--stemmers-blue)](https://github.com/pizza-rs/analysis-stemmers)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue)](LICENSE)

</div>

---

## Overview

`pizza-analysis-stemmers` provides algorithmic stemming for 33 languages using the [Snowball](https://snowballstem.org/) framework for the [INFINI Pizza](https://pizza.rs) search engine.

### Supported Languages

| | | | |
|:-|:-|:-|:-|
| 🇸🇦 Arabic | 🇦🇲 Armenian | 🇪🇸 Basque | 🇪🇸 Catalan |
| 🇨🇿 Czech | 🇩🇰 Danish | 🇳🇱 Dutch | 🇬🇧 English |
| 🇪🇪 Estonian | 🇫🇮 Finnish | 🇫🇷 French | 🇩🇪 German |
| 🇬🇷 Greek | 🇮🇳 Hindi | 🇭🇺 Hungarian | 🇮🇩 Indonesian |
| 🇮🇪 Irish | 🇮🇹 Italian | 🇱🇹 Lithuanian | 🇳🇵 Nepali |
| 🇳🇴 Norwegian | 🇵🇱 Polish | 🇵🇹 Portuguese | 🇷🇴 Romanian |
| 🇷🇺 Russian | 🇪🇸 Spanish | 🇸🇪 Swedish | 🇹🇷 Turkish |
| 🇮🇱 Yiddish | | | |

### Key Features

- **33 Snowball Algorithms** — Proven, well-tested stemming rules
- **Multiple Variants** — Some languages offer aggressive + light variants
- **Configurable Filter** — `stemmer` filter accepts any language name
- **Pure Rust** — No C dependencies, fully `no_std` compatible

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| Filter | `stemmer` | Configurable multi-language Snowball stemmer |
| Filter | `snowball_*` | Per-language named filters (e.g., `snowball_french`) |

## Installation

```toml
[dependencies]
pizza-analysis-stemmers = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["stemmers"] }
```

## Usage

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_stemmers::register_all(&mut factory);
```

## License

BSD-3-Clause

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
