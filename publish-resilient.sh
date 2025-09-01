#!/bin/bash
set -e

echo "Resilient Atlas SDK Publisher"
echo "============================="
echo "This script handles rate limits gracefully and can be resumed."
echo ""

# File to track published packages
PUBLISHED_FILE=".published-packages.txt"
touch "$PUBLISHED_FILE"

# All packages in dependency order
ALL_PACKAGES=(
    # Leaf packages
    "atlas-sanitize"
    "atlas-atomic-u64"
    "atlas-msg"
    "atlas-define-syscall"
    "atlas-program-memory"
    "atlas-native-token"
    "atlas-logger"
    "atlas-frozen-abi-macro"
    # Level 1
    "atlas-frozen-abi"
    "atlas-program-error"
    "atlas-hash"
    "atlas-sha256-hasher"
    "atlas-keccak-hasher"
    "atlas-blake3-hasher"
    "atlas-packet"
    "atlas-derivation-path"
    "atlas-seed-derivable"
    "atlas-hard-forks"
    # Level 2
    "atlas-address"
    "atlas-fee-calculator"
    "atlas-instruction-error"
    "atlas-transaction-error"
    "atlas-short-vec"
    # Level 3
    "atlas-pubkey"
    "atlas-sdk-ids"
    "atlas-sysvar-id"
    "atlas-bincode"
    "atlas-seed-phrase"
    "atlas-epoch-rewards-hasher"
    # Level 4
    "atlas-account-info"
    "atlas-instruction"
    "atlas-signature"
    "atlas-slot-hashes"
    "atlas-clock"
    "atlas-epoch-schedule"
    "atlas-rent"
    "atlas-epoch-rewards"
    "atlas-last-restart-slot"
    "atlas-slot-history"
    "atlas-shred-version"
    "atlas-nonce"
    "atlas-big-mod-exp"
    "atlas-bn254"
    "atlas-secp256k1-recover"
    "atlas-offchain-message"
    # Level 5
    "atlas-signer"
    "atlas-keypair"
    "atlas-message"
    "atlas-program-entrypoint"
    "atlas-system-interface"
    "atlas-example-mocks"
    "atlas-sysvar"
    "atlas-account"
    "atlas-cpi"
    "atlas-program-option"
    "atlas-program-pack"
    "atlas-stable-layout"
    "atlas-borsh"
    "atlas-instructions-sysvar"
    "atlas-time-utils"
    "atlas-epoch-stake"
    "atlas-serde-varint"
    "atlas-epoch-info"
    "atlas-inflation"
    # Level 6
    "atlas-transaction"
    "atlas-nonce-account"
    "atlas-system-transaction"
    "atlas-address-lookup-table-interface"
    "atlas-compute-budget-interface"
    "atlas-ed25519-program"
    "atlas-loader-v2-interface"
    "atlas-presigner"
    "atlas-validator-exit"
    "atlas-reward-info"
    "atlas-serialize-utils"
    "atlas-fee-structure"
    "atlas-poh-config"
    "atlas-cluster-type"
    "atlas-commitment-config"
    "atlas-file-download"
    "atlas-quic-definitions"
    "atlas-client-traits"
    "atlas-signer-store"
    # Level 7
    "atlas-serde"
    "atlas-genesis-config"
    "atlas-stake-interface"
    "atlas-vote-interface"
    "atlas-loader-v3-interface"
    "atlas-loader-v4-interface"
    "atlas-feature-gate-interface"
    "atlas-secp256k1-program"
    "atlas-decode-error"
    # Level 8
    "atlas-program"
    "atlas-sdk-wasm-js"
    "atlas-system-wasm-js"
    "atlas-package-metadata"
    "atlas-package-metadata-macro"
    # Final
    "atlas-sdk"
)

# Function to check if package is already published
is_published() {
    local package=$1
    # Check our tracking file
    if grep -q "^$package$" "$PUBLISHED_FILE"; then
        return 0
    fi
    # Also check crates.io
    if cargo search "$package" --limit 1 2>/dev/null | grep -q "^$package "; then
        echo "$package" >> "$PUBLISHED_FILE"
        return 0
    fi
    return 1
}

# Function to publish with retry logic
publish_with_retry() {
    local package=$1
    local max_retries=5
    local retry_count=0
    local wait_time=60
    
    while [ $retry_count -lt $max_retries ]; do
        echo "Attempting to publish $package (attempt $((retry_count + 1))/$max_retries)..."
        
        if cargo publish -p "$package" --no-verify 2>&1 | tee /tmp/publish_output.txt; then
            if grep -q "Published $package" /tmp/publish_output.txt; then
                echo "$package" >> "$PUBLISHED_FILE"
                echo "✓ Successfully published $package"
                return 0
            fi
        fi
        
        if grep -q "already exists on crates.io" /tmp/publish_output.txt; then
            echo "$package" >> "$PUBLISHED_FILE"
            echo "⏭ Package $package already exists"
            return 0
        fi
        
        if grep -q "Too Many Requests" /tmp/publish_output.txt; then
            echo "⚠ Rate limited. Waiting $wait_time seconds before retry..."
            sleep $wait_time
            # Increase wait time for next retry
            wait_time=$((wait_time * 2))
            retry_count=$((retry_count + 1))
        else
            echo "✗ Failed to publish $package"
            return 1
        fi
    done
    
    echo "✗ Failed to publish $package after $max_retries attempts"
    return 1
}

# Main publishing loop
total=${#ALL_PACKAGES[@]}
published=0
failed=0

echo "Starting publication of $total packages..."
echo ""

for package in "${ALL_PACKAGES[@]}"; do
    if is_published "$package"; then
        echo "⏭ Skipping $package (already published)"
        published=$((published + 1))
    else
        if publish_with_retry "$package"; then
            published=$((published + 1))
            # Small delay between successful publishes
            sleep 5
        else
            failed=$((failed + 1))
            echo "⚠ Failed to publish $package. Continuing with next package..."
        fi
    fi
    
    echo "Progress: $published/$total published, $failed failed"
    echo ""
done

echo "========================================="
echo "Publishing complete!"
echo "Published: $published packages"
echo "Failed: $failed packages"
echo ""

if [ $failed -gt 0 ]; then
    echo "Some packages failed to publish. You can run this script again to retry."
    exit 1
else
    echo "🎉 All packages published successfully!"
fi