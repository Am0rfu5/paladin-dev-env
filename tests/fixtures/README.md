# Test Fixtures for Vision Integration Tests

This directory contains sample images used for environment-gated integration tests.

## Files

- `sample_chart.jpg` - A sample chart image for testing vision capabilities
- `sample_diagram.png` - A sample diagram image for testing vision capabilities

## Note

These are placeholder files. Replace with actual test images before running integration tests with real APIs.

To run vision integration tests:
```bash
ENABLE_VISION_TESTS=true OPENAI_API_KEY=your_key ANTHROPIC_API_KEY=your_key cargo test --test vision_integration_test
```
