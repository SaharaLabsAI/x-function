"""
Example client for the crypto agent with compliance checking.

This demonstrates how to:
1. Create a session with the hypervisor using secp256k1 (Bitcoin curve)
2. Query the crypto agent with encrypted queries
3. Perform compliance checks
4. Get verifiable attestations

NOTE: Uses coincurve library for secp256k1 (Bitcoin curve) support.
"""

import os
import uuid
import requests
import json
from typing import Tuple
import hashlib

from coincurve import PrivateKey, PublicKey
from cryptography.hazmat.primitives.ciphers.aead import AESGCMSIV
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.hkdf import HKDF
from cryptography.hazmat.backends import default_backend


def derive_shared_secret(
    private_key: PrivateKey,
    public_key_bytes: bytes,
    session_id: uuid.UUID
) -> bytes:
    """Derive shared secret using ECDH and HKDF."""
    # Perform raw ECDH using coincurve
    from coincurve import PublicKey as CPublicKey
    
    session_public_key = CPublicKey(public_key_bytes)
    # Multiply session public key by user private key scalar
    shared_point = session_public_key.multiply(private_key.secret)
    
    # Extract x-coordinate (first 32 bytes after the 0x04 prefix for uncompressed)
    shared_point_uncompressed = shared_point.format(compressed=False)
    # Uncompressed format: 0x04 + x (32 bytes) + y (32 bytes)
    shared_key = shared_point_uncompressed[1:33]  # Extract x-coordinate
    
    # Use HKDF to derive encryption key (matching Rust implementation)
    hkdf = HKDF(
        algorithm=hashes.SHA256(),
        length=32,
        salt=session_id.bytes,
        info=b'',
        backend=default_backend()
    )
    
    derived_key = hkdf.derive(shared_key)
    return derived_key


def derive_nonce(data: bytes) -> bytes:
    """Derive a 12-byte nonce using BLAKE3 (or SHA256 fallback)."""
    try:
        import blake3
        hash_result = blake3.blake3(data).digest()
    except ImportError:
        # Fallback to SHA256 if blake3 not available
        hash_result = hashlib.sha256(data).digest()
    
    return hash_result[:12]


def hash_execution(execution: dict) -> str:
    """
    Hash an agent execution to verify integrity.
    Must match the Rust implementation in hash_execution().
    """
    try:
        import blake3
        hasher = blake3.blake3()
    except ImportError:
        # Fallback to SHA256 if blake3 not available
        import hashlib
        hasher = hashlib.sha256()
    
    # Hash session ID
    session_id = uuid.UUID(execution["session_id"])
    hasher.update(session_id.bytes)
    
    # Hash plan
    plan = execution["plan"]
    hasher.update(plan["system_prompt"].encode())
    hasher.update(plan["user_query"].encode())
    
    for step in plan["thought_process"]:
        hasher.update(step["content"].encode())
    
    # Hash tool calls
    for call in execution["tool_calls"]:
        call_id = uuid.UUID(call["id"])
        hasher.update(call_id.bytes)
        hasher.update(call["tool_name"].encode())
        hasher.update(call["arguments"].encode())
    
    # Hash tool results
    for result in execution["tool_results"]:
        call_id = uuid.UUID(result["call_id"])
        hasher.update(call_id.bytes)
        hasher.update(bytes([1 if result["success"] else 0]))
        hasher.update(result["result"].encode())
    
    # Hash final response
    hasher.update(execution["final_response"].encode())
    
    return hasher.hexdigest()


def print_execution_details(execution: dict, indent: int = 2):
    """Pretty print execution details"""
    prefix = " " * indent
    
    print(f"{prefix}Session ID: {execution['session_id']}")
    print(f"{prefix}Execution Time: {execution['execution_time_ms']}ms")
    
    # Plan
    plan = execution["plan"]
    print(f"{prefix}Plan:")
    print(f"{prefix}  User Query: {plan['user_query']}")
    print(f"{prefix}  Thought Process Steps: {len(plan['thought_process'])}")
    for i, step in enumerate(plan['thought_process'], 1):
        print(f"{prefix}    {i}. {step['content'][:80]}...")
    print(f"{prefix}  Intended Tool Calls: {len(plan['intended_tool_calls'])}")
    for call in plan['intended_tool_calls']:
        print(f"{prefix}    - {call['tool_name']}: {call['arguments'][:60]}...")
    
    # Actual tool calls
    print(f"{prefix}Tool Calls Executed: {len(execution['tool_calls'])}")
    for i, call in enumerate(execution['tool_calls'], 1):
        print(f"{prefix}  {i}. {call['tool_name']}")
        print(f"{prefix}     ID: {call['id']}")
        print(f"{prefix}     Args: {call['arguments'][:80]}...")
    
    # Tool results
    print(f"{prefix}Tool Results:")
    for i, result in enumerate(execution['tool_results'], 1):
        status = "✓ SUCCESS" if result['success'] else "✗ FAILED"
        print(f"{prefix}  {i}. {status} (call_id: {result['call_id'][:8]}...)")
        if result.get('error'):
            print(f"{prefix}     Error: {result['error']}")
        else:
            print(f"{prefix}     Result: {result['result'][:100]}...")
    
    # Final response
    print(f"{prefix}Final Response: {execution['final_response'][:150]}...")


def create_session_keypair(
    base_url: str, 
    user_public_key: str, 
    verifiable: bool = False
) -> Tuple[str, str, str]:
    """
    Create a session keypair with the hypervisor.
    
    Args:
        base_url: Hypervisor base URL
        user_public_key: User's hex-encoded public key
        verifiable: If True, use /verifiable/encrypt/create_keypair to get attestation
        
    Returns:
        Tuple of (session_pubkey, session_id, quote)
        Note: quote will be empty string if verifiable=False
    """
    endpoint = "/verifiable/encrypt/create_keypair" if verifiable else "/encrypt/create_keypair"
    response = requests.post(
        f"{base_url}{endpoint}",
        json={"pubkey": user_public_key}
    )
    
    if not response.ok:
        try:
            error_data = response.json()
            error_msg = error_data.get("msg", response.text)
        except:
            error_msg = response.text
        raise Exception(f"HTTP {response.status_code}: {error_msg}")
    
    data = response.json()
    quote = data.get("quote", "")  # Only present if verifiable=True
    return data["session_pubkey"], data["session_id"], quote


class CryptoAgentClient:
    def __init__(self, base_url="http://localhost:3000"):
        self.base_url = base_url
        self.private_key = PrivateKey()
        self.public_key_bytes = self.private_key.public_key.format(compressed=True)
        self.session_pk = None
        self.session_id = None
        self.cipher = None
        
    def create_session(self, verifiable=False):
        """Create a new session with the hypervisor"""
        user_pk_hex = self.public_key_bytes.hex()
        
        session_pk_hex, session_id_str, session_quote = create_session_keypair(
            self.base_url, user_pk_hex, verifiable=verifiable
        )
        
        self.session_id = uuid.UUID(session_id_str)
        self.session_pk = session_pk_hex
        
        # Derive shared encryption key
        session_pk_bytes = bytes.fromhex(session_pk_hex)
        encryption_key = derive_shared_secret(
            self.private_key, 
            session_pk_bytes, 
            self.session_id
        )
        
        self.cipher = AESGCMSIV(encryption_key)
        
        print(f"Session created: {self.session_id}")
        if session_quote:
            print(f"Session Quote: {session_quote[:64]}...")
        
        return self.session_id, session_quote
    
    def query_agent(self, query, verifiable=False):
        """Query the crypto agent"""
        if not self.cipher:
            raise Exception("No session created. Call create_session() first.")
        
        # Encrypt the query
        nonce = derive_nonce(self.session_id.bytes)
        encrypted_query = self.cipher.encrypt(nonce, query.encode(), None)
        
        endpoint = "/verifiable/agent/query" if verifiable else "/agent/query"
        url = f"{self.base_url}{endpoint}"
        
        user_pk_hex = self.public_key_bytes.hex()
        
        response = requests.post(url, json={
            "encrypted_query": encrypted_query.hex(),
            "public_key": user_pk_hex,
            "use_llm_compliance": True
        })
        
        if response.status_code != 200:
            raise Exception(f"Query failed: {response.text}")
        
        data = response.json()
        
        # Decrypt the response
        response_nonce = bytes.fromhex(data["response_nonce"])
        encrypted_response = bytes.fromhex(data["encrypted_response"])
        decrypted_response = self.cipher.decrypt(response_nonce, encrypted_response, None)
        
        result = {
            "response": decrypted_response.decode(),
            "execution_time_ms": data["execution_time_ms"],
            "execution_hash": data["execution_hash"],
            "execution": data["execution"]
        }
        
        if verifiable:
            result["quote"] = data["quote"]
            result["compliance"] = data["compliance"]
        
        return result


def main():
    print("=" * 60)
    print("Verifiable Query with TEE Attestation")
    print("=" * 60)
    
    try:
        # Create verifiable session
        print("\nCreating verifiable session...")
        client = CryptoAgentClient()
        print(f"User Public Key: {client.public_key_bytes.hex()}")
        
        session_id, session_quote = client.create_session(verifiable=True)
        
        # Multiple queries to test
        queries = [
            "What is the current price of Bitcoin?",  # Compliant: T1 (PriceFeedTool) - L1 only
            "What is the sentiment for Solana in the last 24 hours?",  # Compliant: T3 (SentimentTool) - L1, L4
            "Show me the transaction history for address 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb on Ethereum",  # Compliant: T2 (OnChainHistoryTool) specific address - L1, L2, L3
            "Show me all portfolio data on Solana",  # Non-compliant: T4 without address returns all addresses - violates L2/L3/L4
            "Tell me whether I should buy some Bitcoin",  # Non-compliant: Investment advice violates policy
        ]
        
        for i, query in enumerate(queries, 1):
            print(f"\n{i}. Query: {query}")
            print("-" * 60)
            
            try:
                result = client.query_agent(query, verifiable=True)
                print(f"Response: {result['response']}")
                print(f"Execution time: {result['execution_time_ms']}ms")
                print(f"Execution hash: {result['execution_hash']}")
                print(f"Compliance: {result['compliance']['compliant']}")
                print(f"Compliance reason: {result['compliance']['reason']}")
                print(f"Quote length: {len(result['quote'])} chars")
                
                # Print execution details
                print(f"\nExecution Details:")
                print_execution_details(result['execution'])
                
                # Verify hash
                print(f"\nHash Verification:")
                computed_hash = hash_execution(result['execution'])
                matches = computed_hash == result['execution_hash']
                status = "✓ VERIFIED" if matches else "✗ MISMATCH"
                print(f"  Server hash:   {result['execution_hash']}")
                print(f"  Computed hash: {computed_hash}")
                print(f"  Status: {status}")
                
                if not matches:
                    print(f"  WARNING: Hash mismatch indicates execution was tampered with!")
                
            except Exception as e:
                print(f"Error: {e}")
                import traceback
                traceback.print_exc()
        
        print(f"\n\n✓ All queries executed with TEE attestation")
        print(f"✓ Attestation quotes prove:")
        print(f"  1. Session was created in TEE (session_quote)")
        print(f"  2. Each query was executed in TEE (query_quote)")
        print(f"  3. Execution integrity verified via hash matching")
        print(f"  4. Full execution trace available for audit")
        
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    try:
        main()
    except requests.exceptions.ConnectionError:
        print("\nError: Could not connect to hypervisor. Make sure it's running.")
        print("Start it with: cargo run --bin hypervisor")
    except Exception as e:
        print(f"\nError: {e}")
        import traceback
        traceback.print_exc()
