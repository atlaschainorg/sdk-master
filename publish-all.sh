#!/bin/bash
set -e

echo "Publishing Atlas SDK packages to crates.io..."
echo "This script will publish packages in dependency order."
echo ""

# Leaf packages (no atlas-* dependencies)
LEAF_PACKAGES=(
    "atlas-sanitize"
    "atlas-atomic-u64"
    "atlas-msg"
    "atlas-define-syscall"
    "atlas-program-memory"
    "atlas-native-token"
    "atlas-logger"
    "atlas-frozen-abi-macro"
)

# Level 1 packages (depend on leaf packages)
LEVEL1_PACKAGES=(
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
)

# Level 2 packages
LEVEL2_PACKAGES=(
    "atlas-address"
    "atlas-fee-calculator"
    "atlas-instruction-error"
    "atlas-transaction-error"
    "atlas-short-vec"
)

# Level 3 packages
LEVEL3_PACKAGES=(
    "atlas-pubkey"
    "atlas-sdk-ids"
    "atlas-sysvar-id"
    "atlas-bincode"
    "atlas-seed-phrase"
    "atlas-epoch-rewards-hasher"
)

# Level 4 packages
LEVEL4_PACKAGES=(
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
)

# Level 5 packages
LEVEL5_PACKAGES=(
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
)

# Level 6 packages
LEVEL6_PACKAGES=(
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
)

# Level 7 packages
LEVEL7_PACKAGES=(
    "atlas-serde"
    "atlas-genesis-config"
    "atlas-stake-interface"
    "atlas-vote-interface"
    "atlas-loader-v3-interface"
    "atlas-loader-v4-interface"
    "atlas-feature-gate-interface"
    "atlas-secp256k1-program"
    "atlas-decode-error"
)

# Level 8 packages
LEVEL8_PACKAGES=(
    "atlas-program"
    "atlas-sdk-wasm-js"
    "atlas-system-wasm-js"
    "atlas-package-metadata"
    "atlas-package-metadata-macro"
)

# Final package
FINAL_PACKAGE="atlas-sdk"

# Function to publish a package
publish_package() {
    local package=$1
    echo "Publishing $package..."
    
    # Check if package is already published
    if cargo search "$package" --limit 1 2>/dev/null | grep -q "^$package "; then
        echo "⏭ Skipping $package (already published)"
        return 0
    fi
    
    if cargo publish -p "$package" --no-verify 2>&1 | tee /tmp/publish_output.txt | grep -q "already exists on crates.io"; then
        echo "⏭ Skipping $package (already published)"
    elif grep -q "Published $package" /tmp/publish_output.txt; then
        echo "✓ Successfully published $package"
        sleep 5  # Wait a bit between publishes to avoid rate limiting
    elif grep -q "Too Many Requests" /tmp/publish_output.txt; then
        echo "⚠ Rate limited. Waiting 60 seconds before continuing..."
        sleep 60
        # Retry once
        if cargo publish -p "$package" --no-verify; then
            echo "✓ Successfully published $package"
            sleep 5
        else
            echo "✗ Failed to publish $package due to rate limit"
            echo "Please wait and try again later or contact help@crates.io"
            exit 1
        fi
    else
        cat /tmp/publish_output.txt
        echo "✗ Failed to publish $package"
        echo "Check the error above for details"
        exit 1
    fi
}

# Publish packages in order
echo "=== Publishing Leaf Packages ==="
for pkg in "${LEAF_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 1 Packages ==="
for pkg in "${LEVEL1_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 2 Packages ==="
for pkg in "${LEVEL2_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 3 Packages ==="
for pkg in "${LEVEL3_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 4 Packages ==="
for pkg in "${LEVEL4_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 5 Packages ==="
for pkg in "${LEVEL5_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 6 Packages ==="
for pkg in "${LEVEL6_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 7 Packages ==="
for pkg in "${LEVEL7_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Level 8 Packages ==="
for pkg in "${LEVEL8_PACKAGES[@]}"; do
    publish_package "$pkg"
done

echo ""
echo "=== Publishing Final Package ==="
publish_package "$FINAL_PACKAGE"

echo ""
echo "🎉 All packages published successfully!"
echo "You can view them at: https://crates.io/users/your-username"