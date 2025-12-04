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

package ai.saharalabs.x402.server.facilitator;

import ai.saharalabs.x402.model.Kind;
import ai.saharalabs.x402.model.PaymentPayload;
import ai.saharalabs.x402.model.PaymentRequirements;
import ai.saharalabs.x402.model.SettlementResponse;
import ai.saharalabs.x402.model.VerificationResponse;
import java.io.IOException;
import java.util.Set;

/**
 * Contract for calling an x402 facilitator (HTTP, gRPC, mock, etc.).
 */
public interface FacilitatorClient {

  /**
   * Verifies a payment header against the given requirements.
   *
   * @param paymentPayload the X-402 payment header to verify
   * @param req            the payment requirements to validate against
   * @return verification response indicating if payment is valid
   * @throws IOException          if HTTP request fails or returns non-200 status
   * @throws InterruptedException if the request is interrupted
   */
  VerificationResponse verify(PaymentPayload paymentPayload,
      PaymentRequirements req)
      throws IOException, InterruptedException;

  /**
   * Settles a verified payment on the blockchain.
   *
   * @param paymentHeader the X-402 payment header to settle
   * @param req           the payment requirements for settlement
   * @return settlement response with transaction details if successful
   * @throws IOException          if HTTP request fails or returns non-200 status
   * @throws InterruptedException if the request is interrupted
   */
  SettlementResponse settle(PaymentPayload paymentHeader,
      PaymentRequirements req)
      throws IOException, InterruptedException;

  /**
   * Retrieves the set of payment kinds supported by this facilitator.
   *
   * @return set of supported payment kinds (scheme/network combinations)
   * @throws IOException          if HTTP request fails or returns non-200 status
   * @throws InterruptedException if the request is interrupted
   */
  Set<Kind> supported() throws IOException, InterruptedException;
}
