//! Commander with Council Strategy Example
//!
//! Demonstrates using Commander to orchestrate Council discussions with different
//! turn-taking strategies and termination conditions.
//!
//! This example showcases:
//! - Commander automatically selecting Council strategy
//! - Different turn-taking strategies (RoundRobin, ModeratorDirected)
//! - Various termination conditions (MaxRounds, Consensus, Keyword)
//! - Formatted discussion output
//!
//! # Running the Example
//!
//! ```bash
//! cargo run --example commander_council
//! ```

use paladin::core::platform::container::battalion::council::{
    CouncilBuilder, TerminationCondition, TurnStrategy,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║         Commander with Council Strategy Examples               ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    println!("This example demonstrates the Commander pattern orchestrating");
    println!("Council discussions with various configurations.");
    println!();
    println!("────────────────────────────────────────────────────────────────");
    println!();

    // Example 1: RoundRobin with MaxRounds
    println!("📋 Example 1: RoundRobin Turn Strategy with MaxRounds");
    println!();
    println!("Configuration:");
    println!("   • Strategy: Council (automatic detection from 'discussion' keyword)");
    println!("   • Turn Strategy: RoundRobin");
    println!("   • Participants: 3 experts");
    println!("   • Max Rounds: 3");
    println!("   • Termination: MaxRounds");
    println!();

    let council1 = CouncilBuilder::new()
        .name("Architecture Review Council")
        .add_participant("SecurityExpert")
        .add_participant("PerformanceExpert")
        .add_participant("ScalabilityExpert")
        .max_rounds(3)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::MaxRounds)
        .build()?;

    println!("   🎯 Use Case: \"Discuss the pros and cons of microservices architecture\"");
    println!();
    println!("   Expected Behavior:");
    println!("      1. Each expert speaks in order (Security → Performance → Scalability)");
    println!("      2. Pattern repeats for 3 complete rounds");
    println!("      3. Discussion terminates after round 3");
    println!("      4. All experts get equal speaking time");
    println!();
    println!("   Outcome: Structured, balanced discussion with equal participation");
    println!();

    println!("────────────────────────────────────────────────────────────────");
    println!();

    // Example 2: ModeratorDirected Strategy
    println!("📋 Example 2: ModeratorDirected Turn Strategy");
    println!();
    println!("Configuration:");
    println!("   • Strategy: Council");
    println!("   • Turn Strategy: ModeratorDirected");
    println!("   • Moderator: ChiefArchitect");
    println!("   • Participants: 4 experts");
    println!("   • Max Rounds: 5");
    println!("   • Termination: ModeratorDecision");
    println!();

    let council2 = CouncilBuilder::new()
        .name("Critical Decision Council")
        .add_participant("ChiefArchitect")  // Moderator
        .add_participant("FrontendLead")
        .add_participant("BackendLead")
        .add_participant("DevOpsLead")
        .moderator("ChiefArchitect")
        .max_rounds(5)
        .turn_strategy(TurnStrategy::ModeratorDirected)
        .termination_condition(TerminationCondition::ModeratorDecision)
        .build()?;

    println!("   🎯 Use Case: \"Decide on technology stack for new project\"");
    println!();
    println!("   Expected Behavior:");
    println!("      1. Moderator directs conversation flow dynamically");
    println!("      2. Can call on specific experts based on discussion context");
    println!("      3. Moderator decides when discussion is complete");
    println!("      4. More flexible than fixed turn order");
    println!();
    println!("   Outcome: Dynamic, context-aware discussion with expert guidance");
    println!();

    println!("────────────────────────────────────────────────────────────────");
    println!();

    // Example 3: Consensus Termination
    println!("📋 Example 3: Consensus-Based Termination");
    println!();
    println!("Configuration:");
    println!("   • Strategy: Council");
    println!("   • Turn Strategy: RoundRobin");
    println!("   • Participants: 3 team leads");
    println!("   • Max Rounds: 10 (safety limit)");
    println!("   • Termination: Consensus");
    println!();

    let council3 = CouncilBuilder::new()
        .name("Decision Making Council")
        .add_participant("ProductLead")
        .add_participant("EngineeringLead")
        .add_participant("DesignLead")
        .max_rounds(10)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::Consensus)
        .build()?;

    println!("   🎯 Use Case: \"Should we launch feature X next quarter?\"");
    println!();
    println!("   Expected Behavior:");
    println!("      1. Participants discuss until consensus is reached");
    println!("      2. System detects agreement keywords/patterns");
    println!("      3. Terminates early when all agree");
    println!("      4. Falls back to max_rounds if no consensus");
    println!();
    println!("   Outcome: Discussion continues until team alignment is achieved");
    println!();

    println!("────────────────────────────────────────────────────────────────");
    println!();

    // Example 4: Keyword Termination
    println!("📋 Example 4: Keyword-Based Termination");
    println!();
    println!("Configuration:");
    println!("   • Strategy: Council");
    println!("   • Turn Strategy: RoundRobin");
    println!("   • Participants: 3 reviewers");
    println!("   • Max Rounds: 8");
    println!("   • Termination: Keyword(\"APPROVED\")");
    println!();

    let council4 = CouncilBuilder::new()
        .name("Code Review Council")
        .add_participant("SeniorDev1")
        .add_participant("SeniorDev2")
        .add_participant("TechLead")
        .max_rounds(8)
        .turn_strategy(TurnStrategy::RoundRobin)
        .termination_condition(TerminationCondition::Keyword("APPROVED".to_string()))
        .build()?;

    println!("   🎯 Use Case: \"Review pull request #123 for merge approval\"");
    println!();
    println!("   Expected Behavior:");
    println!("      1. Reviewers discuss code quality, testing, security");
    println!("      2. Discussion continues until someone says \"APPROVED\"");
    println!("      3. Terminates immediately on keyword detection");
    println!("      4. Ensures explicit approval before proceeding");
    println!();
    println!("   Outcome: Discussion continues until explicit approval given");
    println!();

    println!("────────────────────────────────────────────────────────────────");
    println!();

    println!("📊 Council Strategy Summary:");
    println!();
    println!("Turn Strategies:");
    println!("   • RoundRobin: Fixed order, equal participation");
    println!("   • ModeratorDirected: Dynamic flow, expert guidance");
    println!("   • Random: Unpredictable order, creative discussions");
    println!("   • VoluntaryWithTimeout: Self-organized participation");
    println!();
    println!("Termination Conditions:");
    println!("   • MaxRounds: Fixed number of discussion rounds");
    println!("   • Consensus: Continue until agreement detected");
    println!("   • ModeratorDecision: Moderator controls termination");
    println!("   • Keyword: Terminate on specific word/phrase");
    println!();

    println!("✅ Commander Council examples completed!");
    println!();
    println!("💡 Key Takeaways:");
    println!("   • Commander can automatically detect Council strategy");
    println!("   • Different turn strategies suit different discussion types");
    println!("   • Termination conditions control discussion length");
    println!("   • Council pattern enables structured multi-agent collaboration");
    println!();
    println!("🔧 Configuration Tips:");
    println!("   • Use RoundRobin for balanced, structured discussions");
    println!("   • Use ModeratorDirected when expert guidance is needed");
    println!("   • Set max_rounds as safety limit for all termination types");
    println!("   • Choose termination condition based on decision-making needs");

    Ok(())
}
