//! Vision Analysis Example
//!
//! Demonstrates basic single-image analysis using the Sentinel Vision System.
//! This example shows how to:
//! - Create a vision-enabled Paladin
//! - Analyze an image from a file
//! - Process the analysis results
//!
//! Run with: `cargo run --example vision_analysis`

use paladin::application::ports::output::llm_port::LlmPort;
use paladin::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::vision::{ImageDetail, VisionContent};
use paladin::infrastructure::adapters::llm::openai_adapter::{OpenAIAdapter, OpenAIConfig};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🎯 Sentinel Vision System - Image Analysis Example\n");

    // Step 1: Configure OpenAI with vision-capable model
    println!("📋 Step 1: Configuring OpenAI with GPT-4o (vision-capable)...");
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");

    let config = OpenAIConfig {
        api_key,
        base_url: "https://api.openai.com/v1".to_string(),
        organization: None,
        timeout_seconds: 300,
        max_retries: 3,
    };

    let llm_adapter: Arc<dyn LlmPort> =
        Arc::new(OpenAIAdapter::new(config).expect("Failed to create OpenAI adapter"));

    // Verify vision support
    let capabilities = llm_adapter.get_capabilities();
    if !capabilities.supports_vision {
        eprintln!("❌ Error: Selected model does not support vision!");
        eprintln!("Please use a vision-capable model like gpt-4o or gpt-4-vision-preview");
        std::process::exit(1);
    }
    println!("✅ Vision support confirmed\n");

    // Step 2: Create circuit breaker for fault tolerance
    println!("📋 Step 2: Setting up circuit breaker...");
    let circuit_breaker = Arc::new(CircuitBreaker::new(
        5,                       // failure_threshold: 5 failures before opening
        3,                       // success_threshold: 3 successes to close
        Duration::from_secs(60), // timeout: wait 60s before retry
    ));
    println!("✅ Circuit breaker configured\n");

    // Step 3: Build vision-enabled Paladin
    println!("📋 Step 3: Building vision-enabled Paladin...");
    let paladin = PaladinBuilder::new(llm_adapter.clone())
        .name("ImageAnalyzer")
        .system_prompt(
            "You are an expert image analyst. \
             Provide detailed, accurate descriptions of images. \
             Focus on key objects, scenes, actions, colors, and composition.",
        )
        .enable_vision(true) // ⚠️  Critical: Enable vision capabilities
        .model("gpt-4o")
        .temperature(0.7)
        .max_loops(1)
        .timeout_seconds(120)
        .build()
        .await
        .expect("Failed to build Paladin");

    println!(
        "✅ Paladin '{}' created with vision enabled\n",
        paladin.node.name
    );

    // Step 4: Create execution service
    println!("📋 Step 4: Creating execution service...");
    let execution_service = PaladinExecutionService::new(
        llm_adapter,
        circuit_breaker,
        None, // No garrison (memory) for this simple example
        None, // No arsenal (tools) for this example
    );
    println!("✅ Execution service ready\n");

    // Step 5: Prepare image for analysis
    println!("📋 Step 5: Preparing image for analysis...");

    // Option 1: Analyze a local file (recommended for this example)
    // Create a test image or point to an existing one
    let image_path = PathBuf::from("./examples/assets/sample_image.jpg");

    if !image_path.exists() {
        println!("⚠️  Warning: Sample image not found at {:?}", image_path);
        println!("   Creating example with image URL instead...\n");

        // Option 2: Use a publicly accessible image URL
        let vision_content = vec![VisionContent::ImageUrl {
            url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png".to_string(),
            detail: ImageDetail::Auto,
        }];

        analyze_image(
            &execution_service,
            &paladin,
            vision_content,
            "What do you see in this image? Describe it in detail.",
        )
        .await?;
    } else {
        // Use local file
        let vision_content = vec![VisionContent::ImageFile {
            path: image_path.clone(),
            detail: ImageDetail::Auto, // Let the model decide optimal detail level
        }];

        println!("📸 Image: {:?}", image_path);
        println!("🔍 Detail Level: Auto (balanced speed/quality)\n");

        analyze_image(
            &execution_service,
            &paladin,
            vision_content,
            "What do you see in this image? Describe it in detail.",
        )
        .await?;
    }

    // Step 6: Demonstrate different detail levels
    println!();
    println!("{}", "=".repeat(80));
    println!("📊 Demonstrating Different Detail Levels");
    println!("{}", "=".repeat(80));
    println!();

    // Low detail - faster and cheaper
    let low_detail_image = vec![VisionContent::ImageUrl {
        url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png".to_string(),
        detail: ImageDetail::Low,  // Max 512x512, ~85 tokens
    }];

    println!("🔽 LOW DETAIL (Fast & Cheap - ~85 tokens)");
    analyze_image(
        &execution_service,
        &paladin,
        low_detail_image,
        "Quick summary of this image.",
    )
    .await?;

    // High detail - more accurate but slower
    let high_detail_image = vec![VisionContent::ImageUrl {
        url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png".to_string(),
        detail: ImageDetail::High,  // Up to 2048x2048, ~170 tokens per tile
    }];

    println!("\n🔼 HIGH DETAIL (Accurate & Detailed - ~170+ tokens)");
    analyze_image(
        &execution_service,
        &paladin,
        high_detail_image,
        "Provide an extremely detailed analysis of this image.",
    )
    .await?;

    println!();
    println!("{}", "=".repeat(80));
    println!("✅ Vision analysis example completed successfully!");
    println!("{}", "=".repeat(80));

    Ok(())
}

/// Helper function to analyze an image and display results
async fn analyze_image(
    service: &PaladinExecutionService,
    paladin: &paladin::core::platform::container::paladin::Paladin,
    images: Vec<VisionContent>,
    task: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("💭 Task: {}", task);
    println!("⏳ Analyzing...\n");

    let start = std::time::Instant::now();

    // Execute vision analysis
    let result = service.execute_with_vision(paladin, task, images).await?;

    let duration = start.elapsed();

    // Display results
    println!("{}", "─".repeat(80));
    println!("📊 ANALYSIS RESULTS");
    println!("{}", "─".repeat(80));
    println!("🤖 Paladin: {}", paladin.node.name);
    println!("⏱️  Execution Time: {:.2}s", duration.as_secs_f64());
    println!("🔄 Loops: {}", result.loop_count);
    println!("🎫 Tokens: {}", result.token_count);
    println!("🛑 Stop Reason: {:?}", result.stop_reason);
    println!("{}", "─".repeat(80));
    println!("📝 OUTPUT:\n");
    println!("{}", result.output);
    println!("{}", "─".repeat(80));

    Ok(())
}
