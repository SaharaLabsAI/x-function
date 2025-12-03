#!/usr/bin/env python3
"""
Generate a secp256k1 (NOT secp256r1) keypair for WASM Hypervisor API.
The server uses k256 crate which is secp256k1!
"""

import requests
import json

# We need to use secp256k1, not secp256r1
# The cryptography library doesn't support secp256k1 directly,
# so we need to use a different library

from coincurve import PrivateKey

# Generate secp256k1 keypair
private_key = PrivateKey()
public_key_bytes = private_key.public_key.format(compressed=True)

# Convert to hex
pubkey_hex = public_key_bytes.hex()

print("Generated secp256k1 keypair (Bitcoin curve)")
print("=" * 60)
print(f"Public key (hex): {pubkey_hex}")
print(f"Length: {len(pubkey_hex)} chars ({len(public_key_bytes)} bytes)")
print(f"First byte: 0x{public_key_bytes[0]:02x}")
print()

# Create JSON payload
payload = {"pubkey": pubkey_hex}
payload_json = json.dumps(payload)

print("JSON payload:")
print(payload_json)
print()

# Save private key (hex format)
with open('private_key_secp256k1.txt', 'w') as f:
    f.write(private_key.to_hex())
print("Private key saved to: private_key_secp256k1.txt")
print()

# Test with server
print("=" * 60)
print("Testing with Python requests library...")
print()

try:
    response = requests.post(
        "http://localhost:3000/verifiable/encrypt/create_keypair",
        json=payload,
        timeout=5
    )
    
    print(f"Status: {response.status_code}")
    print(f"Response body: {response.text}")
    print()
    
    if response.status_code == 200:
        print("✅ SUCCESS!")
        data = response.json()
        print(f"Session ID: {data['session_id']}")
        print(f"Session pubkey: {data['session_pubkey']}")
        with open('response.json','w') as wf:
            json.dump(data,wf,indent=2)
    else:
        print(f"❌ FAILED with status {response.status_code}")
        
except Exception as e:
    print(f"❌ Error: {e}")
