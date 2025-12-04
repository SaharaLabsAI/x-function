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

/**
 * JSON returned by POST /settle on the facilitator.
 *
 * @see <a href="https://github.com/coinbase/x402/blob/main/specs/x402-specification.md">X402
 * Protocol Specification</a>
 */
public class SettlementResponse {

  /**
   * Whether the payment settlement succeeded.
   */
  public boolean success;

  /**
   * Error reason if settlement failed (omitted if successful)
   */
  public String errorReason;

  /**
   * Blockchain transaction hash (empty string if settlement failed)
   */
  public String transaction;

  /**
   * Blockchain network identifier
   */
  public String network;

  /**
   * Address of the payer's wallet
   */
  public String payer;
}
