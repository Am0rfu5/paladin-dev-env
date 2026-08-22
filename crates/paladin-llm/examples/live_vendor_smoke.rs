//! Live vendor smoke test — Kimi, Qwen, Grok, Gemini (Phase 17 UAT test 4).
//!
//! Proves, against the REAL vendor endpoints, that for each provider:
//!   1. the documented `base_url` resolves and authenticates,
//!   2. `get_available_models()` returns from the LIVE-FETCH path, not the
//!      curated fallback, and
//!   3. the adapter's default model ID actually exists in that live list.
//!
//! Point 2 is the reason this cannot be a plain "did it return a non-empty
//! list" check: `available_models()` swallows every failure and silently
//! returns the curated `*_FALLBACK_MODELS` constant, so a non-empty result
//! is NOT evidence the network path worked. This binary discriminates by
//! comparing the result against that exact constant.
//!
//! Run with real credentials in the environment:
//!   cargo run -p paladin-llm --example live_vendor_smoke \
//!     --features kimi,qwen,grok,gemini
//!
//! Exits non-zero if any provider fails. Never prints a credential.

use paladin_llm::gemini::adapter::{
    GEMINI_DEFAULT_BASE_URL, GEMINI_DEFAULT_MODEL, GEMINI_FALLBACK_MODELS,
};
use paladin_llm::gemini::{GeminiAdapter, GeminiConfig};
use paladin_llm::grok::adapter::{GROK_DEFAULT_BASE_URL, GROK_DEFAULT_MODEL, GROK_FALLBACK_MODELS};
use paladin_llm::grok::{GrokAdapter, GrokConfig};
use paladin_llm::kimi::adapter::{KIMI_DEFAULT_BASE_URL, KIMI_DEFAULT_MODEL, KIMI_FALLBACK_MODELS};
use paladin_llm::kimi::{KimiAdapter, KimiConfig};
use paladin_llm::qwen::adapter::{QWEN_DEFAULT_BASE_URL, QWEN_DEFAULT_MODEL, QWEN_FALLBACK_MODELS};
use paladin_llm::qwen::{QwenAdapter, QwenConfig};
use paladin_ports::output::llm_port::LlmPort;

/// Outcome of probing one provider.
struct Probe {
    vendor: &'static str,
    key_var: &'static str,
    base_url: &'static str,
    default_model: &'static str,
    /// `Err` = the adapter could not even be constructed (missing key).
    result: Result<Live, String>,
}

struct Live {
    models: Vec<String>,
    /// True when the returned list is byte-identical to the curated fallback,
    /// i.e. the live fetch silently failed.
    is_fallback: bool,
    default_present: bool,
}

fn classify(models: Vec<String>, fallback: &[&str], default_model: &str) -> Live {
    let is_fallback =
        models.len() == fallback.len() && models.iter().zip(fallback.iter()).all(|(m, f)| m == f);
    let default_present = models.iter().any(|m| m == default_model);
    Live {
        models,
        is_fallback,
        default_present,
    }
}

#[tokio::main]
async fn main() {
    let mut probes: Vec<Probe> = Vec::new();

    // ── Kimi (Moonshot) ─────────────────────────────────────────────
    probes.push(Probe {
        vendor: "Kimi",
        key_var: "MOONSHOT_API_KEY",
        base_url: KIMI_DEFAULT_BASE_URL,
        default_model: KIMI_DEFAULT_MODEL,
        result: match KimiConfig::from_env()
            .and_then(|c| KimiAdapter::new(c).map_err(|e| e.to_string()))
        {
            Ok(a) => match a.get_available_models().await {
                Ok(m) => Ok(classify(m, KIMI_FALLBACK_MODELS, KIMI_DEFAULT_MODEL)),
                Err(e) => Err(format!("get_available_models failed: {e}")),
            },
            Err(e) => Err(e),
        },
    });

    // ── Qwen (Alibaba DashScope) ────────────────────────────────────
    probes.push(Probe {
        vendor: "Qwen",
        key_var: "DASHSCOPE_API_KEY",
        base_url: QWEN_DEFAULT_BASE_URL,
        default_model: QWEN_DEFAULT_MODEL,
        result: match QwenConfig::from_env()
            .and_then(|c| QwenAdapter::new(c).map_err(|e| e.to_string()))
        {
            Ok(a) => match a.get_available_models().await {
                Ok(m) => Ok(classify(m, QWEN_FALLBACK_MODELS, QWEN_DEFAULT_MODEL)),
                Err(e) => Err(format!("get_available_models failed: {e}")),
            },
            Err(e) => Err(e),
        },
    });

    // ── Grok (xAI) ──────────────────────────────────────────────────
    probes.push(Probe {
        vendor: "Grok",
        key_var: "XAI_API_KEY",
        base_url: GROK_DEFAULT_BASE_URL,
        default_model: GROK_DEFAULT_MODEL,
        result: match GrokConfig::from_env()
            .and_then(|c| GrokAdapter::new(c).map_err(|e| e.to_string()))
        {
            Ok(a) => match a.get_available_models().await {
                Ok(m) => Ok(classify(m, GROK_FALLBACK_MODELS, GROK_DEFAULT_MODEL)),
                Err(e) => Err(format!("get_available_models failed: {e}")),
            },
            Err(e) => Err(e),
        },
    });

    // ── Gemini (Google) ─────────────────────────────────────────────
    probes.push(Probe {
        vendor: "Gemini",
        key_var: "GEMINI_API_KEY",
        base_url: GEMINI_DEFAULT_BASE_URL,
        default_model: GEMINI_DEFAULT_MODEL,
        result: match GeminiConfig::from_env()
            .and_then(|c| GeminiAdapter::new(c).map_err(|e| e.to_string()))
        {
            Ok(a) => match a.get_available_models().await {
                Ok(m) => Ok(classify(m, GEMINI_FALLBACK_MODELS, GEMINI_DEFAULT_MODEL)),
                Err(e) => Err(format!("get_available_models failed: {e}")),
            },
            Err(e) => Err(e),
        },
    });

    let mut failures = 0usize;

    for p in &probes {
        println!("\n=== {} ({}) ===", p.vendor, p.key_var);
        println!("  base_url      : {}", p.base_url);
        println!("  default model : {}", p.default_model);
        match &p.result {
            Err(e) => {
                println!("  RESULT        : FAIL — {e}");
                failures += 1;
            }
            Ok(live) => {
                println!("  models returned: {}", live.models.len());
                if live.is_fallback {
                    println!(
                        "  live fetch    : NO — result is byte-identical to the curated fallback"
                    );
                    println!("  RESULT        : FAIL (live-fetch path not exercised)");
                    failures += 1;
                } else {
                    println!("  live fetch    : YES — differs from curated fallback");
                    let mut sample: Vec<&str> = live.models.iter().map(|s| s.as_str()).collect();
                    sample.sort_unstable();
                    let shown: Vec<&str> = sample.iter().take(8).copied().collect();
                    println!(
                        "  sample        : {}{}",
                        shown.join(", "),
                        if sample.len() > 8 {
                            format!(", … (+{} more)", sample.len() - 8)
                        } else {
                            String::new()
                        }
                    );
                    if live.default_present {
                        println!("  default model in live list: YES");
                        println!("  RESULT        : PASS");
                    } else {
                        println!("  default model in live list: NO  <-- default model ID is wrong");
                        println!("  RESULT        : FAIL (default model absent from live list)");
                        failures += 1;
                    }
                }
            }
        }
    }

    println!("\n──────────────────────────────────────────");
    println!(
        "{} of {} vendors passed",
        probes.len() - failures,
        probes.len()
    );
    if failures > 0 {
        std::process::exit(1);
    }
}
