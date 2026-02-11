#!/usr/bin/env python3
"""
Script to add ..Default::default() to PaladinResult struct initializations
"""
import re
import sys
from pathlib import Path

def fix_file(filepath):
    """Fix a single file by adding ..Default::default() where missing"""
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    # Pattern to match PaladinResult { ... } without ..Default::default()
    # This is complex, so we'll use a multi-step approach
    
    # Find all PaladinResult { ... } blocks
    pattern = r'(PaladinResult\s*\{[^}]+stop_reason:[^,}]+)(,?\s*)(})'
    
    def replacer(match):
        block = match.group(1)
        comma = match.group(2)
        closing = match.group(3)
        
        # Check if ..Default::default() is already present
        if '..Default::default()' in block:
            return match.group(0)  # No change needed
        
        # Add ..Default::default() before the closing brace
        if not comma.strip():
            # No trailing comma, add one
            return block + ',\n            ..Default::default()\n        ' + closing
        else:
            # Has trailing comma
            return block + comma.rstrip() + '\n            ..Default::default()\n        ' + closing
    
    content = re.sub(pattern, replacer, content, flags=re.DOTALL)
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    # Get all Rust files
    base_path = Path('/home/jamatulli/Development/ai/paladin')
    
    rust_files = []
    for pattern in ['src/**/*.rs', 'tests/**/*.rs', 'examples/**/*.rs', 'benches/**/*.rs']:
        rust_files.extend(base_path.glob(pattern))
    
    fixed_count = 0
    for filepath in rust_files:
        if fix_file(filepath):
            print(f"Fixed: {filepath}")
            fixed_count += 1
    
    print(f"\nTotal files fixed: {fixed_count}")

if __name__ == '__main__':
    main()
