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

import java.util.Map;

/**
 * Defines an acceptable way to pay for a resource. Matches the x402 PaymentRequirements schema.
 *
 * @see <a href="https://github.com/coinbase/x402/blob/main/specs/x402-specification.md">X402
 * Protocol Specification</a>
 */
public class PaymentRequirements {

  /**
   * Payment scheme identifier (e.g., "exact"). Required.
   */
  public String scheme;

  /**
   * Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet"). Required.
   */
  public String network;

  /**
   * Required payment amount in atomic token units. Required.
   */
  public String maxAmountRequired;

  /**
   * Token contract address. Required.
   */
  public String asset;

  /**
   * Recipient wallet address for the payment. Required.
   */
  public String payTo;

  /**
   * URL of the protected resource. Required.
   */
  public String resource;

  /**
   * Human-readable description of the resource. Required.
   */
  public String description;

  /**
   * MIME type of the expected response. Optional.
   */
  public String mimeType;

  /**
   * JSON schema describing the response format. Optional.
   */
  public Map<String, Object> outputSchema;

  /**
   * Maximum time allowed for payment completion. Required.
   */
  public int maxTimeoutSeconds;

  /**
   * Scheme-specific additional information. Optional.
   */
  public Map<String, Object> extra;
}
