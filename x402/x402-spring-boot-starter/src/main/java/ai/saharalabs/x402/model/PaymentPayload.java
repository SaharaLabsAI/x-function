/*
 * MIT License
 *
 * Copyright © 2025 Sahara AI
 *
 * This file is part of the x402-function project.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

package ai.saharalabs.x402.model;

import ai.saharalabs.x402.util.Json;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Base64;
import java.util.Map;

/**
 * Base64-encoded JSON object carried in the X-PAYMENT request header. Fields follow the x402
 * “PaymentPayload” schema.
 *
 * @see <a href="https://github.com/coinbase/x402/blob/main/specs/x402-specification.md">X402
 * Protocol Specification</a>
 */
public class PaymentPayload {

  /**
   * x402 protocol version. Integer. Must be a supported version (currently 1).
   */
  public int x402Version;

  /**
   * Payment scheme identifier. Must equal the selected paymentRequirements.scheme.
   */
  public String scheme;

  /**
   * Blockchain network identifier. Must equal paymentRequirements.network (e.g., "base",
   * "base-sepolia").
   */
  public String network;

  /**
   * Scheme-specific payload object. Structure depends on the chosen scheme and network. For EVM
   * "exact", uses an ERC-3009 authorization payload.
   */
  public Map<String, Object> payload;

  /**
   * Decode from an X-PAYMENT header value. Step 1: Base64 decode. Step 2: JSON deserialize into
   * this type.
   */
  public static PaymentPayload fromHeader(String header) throws IOException {
    byte[] decoded = Base64.getDecoder().decode(header);
    return Json.MAPPER.readValue(decoded, PaymentPayload.class);
  }

  /**
   * Serialize to JSON and Base64-encode for use as the X-PAYMENT header value.
   */
  public String toHeader() {
    try {
      String json = Json.MAPPER.writeValueAsString(this);
      return Base64.getEncoder().encodeToString(json.getBytes(StandardCharsets.UTF_8));
    } catch (IOException e) {
      throw new IllegalStateException("Unable to encode payment header", e);
    }
  }
}
