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
    /// The base URL the run ACTUALLY used, read back off the resolved config —
    /// NOT the `*_DEFAULT_BASE_URL` constant. A `*_BASE_URL` environment
    /// override changes what goes on the wire, and a record that printed the
    /// constant regardless would attribute a result to the wrong endpoint.
    base_url: String,
    default_model: String,
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
    probes.push(match KimiConfig::from_env() {
        Err(e) => Probe {
            vendor: "Kimi",
            key_var: "MOONSHOT_API_KEY",
            base_url: KIMI_DEFAULT_BASE_URL.to_string(),
            default_model: KIMI_DEFAULT_MODEL.to_string(),
            result: Err(e),
        },
        Ok(cfg) => {
            let (base_url, model) = (cfg.base_url.clone(), cfg.model.clone());
            let result = match KimiAdapter::new(cfg) {
                Err(e) => Err(e.to_string()),
                Ok(a) => match a.get_available_models().await {
                    Ok(m) => Ok(classify(m, KIMI_FALLBACK_MODELS, &model)),
                    Err(e) => Err(format!("get_available_models failed: {e}")),
                },
            };
            Probe {
                vendor: "Kimi",
                key_var: "MOONSHOT_API_KEY",
                base_url,
                default_model: model,
                result,
            }
        }
    });

    // ── Qwen (Alibaba DashScope) ────────────────────────────────────
    probes.push(match QwenConfig::from_env() {
        Err(e) => Probe {
            vendor: "Qwen",
            key_var: "DASHSCOPE_API_KEY",
            base_url: QWEN_DEFAULT_BASE_URL.to_string(),
            default_model: QWEN_DEFAULT_MODEL.to_string(),
            result: Err(e),
        },
        Ok(cfg) => {
            let (base_url, model) = (cfg.base_url.clone(), cfg.model.clone());
            let result = match QwenAdapter::new(cfg) {
                Err(e) => Err(e.to_string()),
                Ok(a) => match a.get_available_models().await {
                    Ok(m) => Ok(classify(m, QWEN_FALLBACK_MODELS, &model)),
                    Err(e) => Err(format!("get_available_models failed: {e}")),
                },
            };
            Probe {
                vendor: "Qwen",
                key_var: "DASHSCOPE_API_KEY",
                base_url,
                default_model: model,
                result,
            }
        }
    });

    // ── Grok (xAI) ──────────────────────────────────────────────────
    probes.push(match GrokConfig::from_env() {
        Err(e) => Probe {
            vendor: "Grok",
            key_var: "XAI_API_KEY",
            base_url: GROK_DEFAULT_BASE_URL.to_string(),
            default_model: GROK_DEFAULT_MODEL.to_string(),
            result: Err(e),
        },
        Ok(cfg) => {
            let (base_url, model) = (cfg.base_url.clone(), cfg.model.clone());
            let result = match GrokAdapter::new(cfg) {
                Err(e) => Err(e.to_string()),
                Ok(a) => match a.get_available_models().await {
                    Ok(m) => Ok(classify(m, GROK_FALLBACK_MODELS, &model)),
                    Err(e) => Err(format!("get_available_models failed: {e}")),
                },
            };
            Probe {
                vendor: "Grok",
                key_var: "XAI_API_KEY",
                base_url,
                default_model: model,
                result,
            }
        }
    });

    // ── Gemini (Google) ─────────────────────────────────────────────
    probes.push(match GeminiConfig::from_env() {
        Err(e) => Probe {
            vendor: "Gemini",
            key_var: "GEMINI_API_KEY",
            base_url: GEMINI_DEFAULT_BASE_URL.to_string(),
            default_model: GEMINI_DEFAULT_MODEL.to_string(),
            result: Err(e),
        },
        Ok(cfg) => {
            let (base_url, model) = (cfg.base_url.clone(), cfg.model.clone());
            let result = match GeminiAdapter::new(cfg) {
                Err(e) => Err(e.to_string()),
                Ok(a) => match a.get_available_models().await {
                    Ok(m) => Ok(classify(m, GEMINI_FALLBACK_MODELS, &model)),
                    Err(e) => Err(format!("get_available_models failed: {e}")),
                },
            };
            Probe {
                vendor: "Gemini",
                key_var: "GEMINI_API_KEY",
                base_url,
                default_model: model,
                result,
            }
        }
    });

    let mut failures = 0usize;

    for p in &probes {
        println!("\n=== {} ({}) ===", p.vendor, p.key_var);
        let shipped_default = match p.vendor {
            "Kimi" => KIMI_DEFAULT_BASE_URL,
            "Qwen" => QWEN_DEFAULT_BASE_URL,
            "Grok" => GROK_DEFAULT_BASE_URL,
            _ => GEMINI_DEFAULT_BASE_URL,
        };
        println!(
            "  base_url      : {}{}",
            p.base_url,
            if p.base_url == shipped_default {
                String::new()
            } else {
                format!(
                    "\n                  [OVERRIDE via *_BASE_URL — shipped default is {shipped_default}]"
                )
            }
        );
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
