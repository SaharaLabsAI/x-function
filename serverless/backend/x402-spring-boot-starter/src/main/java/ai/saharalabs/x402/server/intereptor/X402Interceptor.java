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

package ai.saharalabs.x402.server.intereptor;

import ai.saharalabs.x402.configuration.X402Configuration;
import ai.saharalabs.x402.model.PaymentPayload;
import ai.saharalabs.x402.model.PaymentRequiredResponse;
import ai.saharalabs.x402.model.PaymentRequirements;
import ai.saharalabs.x402.model.SettlementResponse;
import ai.saharalabs.x402.model.SettlementResponseHeader;
import ai.saharalabs.x402.model.VerificationResponse;
import ai.saharalabs.x402.server.annotation.X402Payment;
import ai.saharalabs.x402.server.facilitator.FacilitatorClient;
import ai.saharalabs.x402.server.price.PriceCalculatorHelper;
import ai.saharalabs.x402.util.Json;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.math.BigDecimal;
import java.nio.charset.StandardCharsets;
import java.util.Base64;
import lombok.Builder;
import lombok.Getter;
import lombok.Setter;
import lombok.extern.slf4j.Slf4j;
import org.springframework.lang.NonNull;
import org.springframework.lang.Nullable;
import org.springframework.util.StringUtils;
import org.springframework.web.method.HandlerMethod;
import org.springframework.web.servlet.HandlerInterceptor;

@Slf4j
@Getter
@Setter
@Builder
public class X402Interceptor implements HandlerInterceptor {

  /**
   * x402 protocol version supported by this interceptor.
   */
  private static final int PROTOCOL_VERSION = 1;

  /**
   * Default token decimals if not provided (e.g., USDC = 6).
   */
  private static final int DEFAULT_ASSET_DECIMALS = 6;

  /**
   * Request header carrying the payment payload (Base64 encoded).
   */
  private static final String HEADER_PAYMENT = "X-PAYMENT";
  /**
   * Response header carrying settlement information (Base64 encoded).
   */
  private static final String HEADER_PAYMENT_RESPONSE = "X-PAYMENT-RESPONSE";
  /**
   * CORS header exposing payment response header to browser clients.
   */
  private static final String HEADER_ACCESS_CONTROL_EXPOSE = "Access-Control-Expose-Headers";

  /**
   * Request attribute: verified payment requirements.
   */
  private static final String ATTR_REQUIREMENTS = "x402.payment.requirements";
  /**
   * Request attribute: original payment header value.
   */
  private static final String ATTR_HEADER = "x402.payment.header";
  /**
   * Request attribute: decoded & verified payment payload.
   */
  private static final String ATTR_PAYLOAD = "x402.payment.payload";

  private final FacilitatorClient facilitator;
  private final PriceCalculatorHelper priceCalculatorHelper;
  private final X402Configuration x402Configuration;

  @Override
  public boolean preHandle(@NonNull HttpServletRequest request,
      @NonNull HttpServletResponse response, @NonNull Object handler) {
    X402Payment annotation = resolveAnnotation(handler);
    if (annotation == null) { // Non-payment endpoint, allow
      return true;
    }
    final String url = request.getRequestURL().toString();

    PaymentRequirements requirements = buildRequirements(request, url, annotation);

    String header = request.getHeader(HEADER_PAYMENT);
    if (!StringUtils.hasText(header)) {
      log.info("x402 missing payment header URL: {}", url);
      respond402(response, requirements, HEADER_PAYMENT + " header is required");
      return false;
    }

    PaymentPayload payload = decodePaymentHeader(header, url, response, requirements);
    if (payload == null) { // already responded
      return false;
    }

    VerificationResponse verification = performVerification(payload, requirements, url, header,
        response);
    if (verification == null) { // failure already responded
      return false;
    }

    if (!verification.isValid) {
      log.info("x402 payment verification failed URL: {} reason: {}", url,
          verification.invalidReason);
      respond402(response, requirements, verification.invalidReason);
      return false;
    }

    // Store for afterCompletion settlement phase
    request.setAttribute(ATTR_REQUIREMENTS, requirements);
    request.setAttribute(ATTR_HEADER, header);
    request.setAttribute(ATTR_PAYLOAD, payload);

    return true;
  }

  /* ======================== preHandle: /verify ======================== */

  @Override
  public void afterCompletion(@NonNull HttpServletRequest request,
      @NonNull HttpServletResponse response, @NonNull Object handler, @Nullable Exception ex) {
    X402Payment annotation = resolveAnnotation(handler);
    if (annotation == null) { // Non-payment endpoint
      return;
    }
    if (response.getStatus() >= 400) { // Error response, skip settlement
      return;
    }

    PaymentRequirements requirements = (PaymentRequirements) request.getAttribute(
        ATTR_REQUIREMENTS);
    PaymentPayload payload = (PaymentPayload) request.getAttribute(ATTR_PAYLOAD);
    String header = (String) request.getAttribute(ATTR_HEADER);

    if (requirements == null || payload == null || header == null) { // Verification didn't pass
      return;
    }

    final String url = request.getRequestURL().toString();
    try {
      SettlementResponse settlement = facilitator.settle(payload, requirements);
      log.info("x402 settlement response URL: {} response: {}", url,
          Json.MAPPER.writeValueAsString(settlement));

      if (settlement == null || !settlement.success) {
        if (!response.isCommitted()) {
          String reason =
              (settlement != null && settlement.errorReason != null) ? settlement.errorReason
                  : "settlement failed";
          log.error("x402 settlement failed URL: {} error: {}", url, reason);
          respond402(response, requirements, reason);
        }
        return;
      }

      attachSettlementHeader(response, payload, settlement, header, url);
    } catch (Exception e2) {
      log.error("x402 settlement error URL: {} header: {}", url, header, e2);
      if (!response.isCommitted()) {
        respond402(response, requirements, "settlement error: " + e2.getMessage());
      }
    }
  }

  /* ======================== afterCompletion: /settle ======================== */

  @Nullable
  private X402Payment resolveAnnotation(Object handler) {
    if (!(handler instanceof HandlerMethod hm)) {
      return null;
    }
    X402Payment methodAnn = hm.getMethodAnnotation(X402Payment.class);
    if (methodAnn != null) {
      return methodAnn;
    }
    return hm.getBeanType().getAnnotation(X402Payment.class);
  }

  /* ======================== Helpers ======================== */

  /**
   * Write a 402 Payment Required response body.
   */
  private void respond402(HttpServletResponse resp, PaymentRequirements requirements,
      String error) {
    try {
      PaymentRequiredResponse prr = new PaymentRequiredResponse();
      prr.x402Version = PROTOCOL_VERSION;
      prr.accepts.add(requirements);
      prr.error = error;
      respondJson(resp, HttpServletResponse.SC_PAYMENT_REQUIRED,
          Json.MAPPER.writeValueAsString(prr));
    } catch (IOException e) {
      log.error("x402 error serializing 402 JSON", e);
      respondJson(resp, HttpServletResponse.SC_PAYMENT_REQUIRED,
          "{\"error\":\"Payment required\"}");
    }
  }

  /**
   * Attach settlement header to successful response.
   */
  private void attachSettlementHeader(HttpServletResponse response, PaymentPayload payload,
      SettlementResponse sr, String originalHeader, String url) {
    try {
      String base64Header = createPaymentResponseHeader(sr, sr.payer);
      response.setHeader(HEADER_PAYMENT_RESPONSE, base64Header);
      response.setHeader(HEADER_ACCESS_CONTROL_EXPOSE, HEADER_PAYMENT_RESPONSE);
    } catch (Exception buildEx) {
      log.error("x402 error creating settlement header URL: {} header: {}", url, originalHeader,
          buildEx);
      if (!response.isCommitted()) {
        respond500(response, "Failed to create settlement response header");
      }
    }
  }

  /**
   * Generic JSON response utility to reduce duplication.
   */
  private void respondJson(HttpServletResponse resp, int status, String body) {
    if (resp.isCommitted()) {
      return;
    }
    try {
      resp.resetBuffer();
      resp.setStatus(status);
      resp.setContentType("application/json");
      resp.getWriter().write(body);
      resp.flushBuffer();
    } catch (IOException e) {
      log.error("x402 error writing response status={} body={}", status, body, e);
    }
  }

  /**
   * Build Base64 settlement response header string.
   */
  private String createPaymentResponseHeader(SettlementResponse sr, String payer) throws Exception {
    SettlementResponseHeader settlementHeader = new SettlementResponseHeader(true,
        sr.transaction != null ? sr.transaction : "", sr.network != null ? sr.network : "", payer);
    String jsonString = Json.MAPPER.writeValueAsString(settlementHeader);
    return Base64.getEncoder().encodeToString(jsonString.getBytes(StandardCharsets.UTF_8));
  }

  /**
   * Write a 500 Internal Server Error JSON response.
   */
  private void respond500(HttpServletResponse resp, String message) {
    String safe = message.replace("\"", "\\\"");
    respondJson(resp, HttpServletResponse.SC_INTERNAL_SERVER_ERROR, "{\"error\":\"" + safe + "\"}");
  }

  /**
   * Decode the payment header value into a {@link PaymentPayload}.
   */
  @Nullable
  private PaymentPayload decodePaymentHeader(String header, String url,
      HttpServletResponse response, PaymentRequirements requirements) {
    try {
      return PaymentPayload.fromHeader(header);
    } catch (IllegalArgumentException ex) {
      log.error("x402 invalid payment header URL: {} header: {}", url, header, ex);
      respond402(response, requirements, "malformed " + HEADER_PAYMENT + " header");
    } catch (IOException ex) {
      log.error("x402 decode IO error URL: {} header: {}", url, header, ex);
      respond500(response, "Payment decode failed: " + ex.getMessage());
    } catch (Exception ex) {
      log.error("x402 unexpected decode error URL: {} header: {}", url, header, ex);
      respond500(response, "Internal server error decoding payment header");
    }
    return null;
  }

  /**
   * Perform payment verification via facilitator.
   */
  @Nullable
  private VerificationResponse performVerification(PaymentPayload payload,
      PaymentRequirements requirements, String url, String header, HttpServletResponse response) {
    try {
      return facilitator.verify(payload, requirements);
    } catch (IOException ex) {
      log.error("x402 facilitator communication error URL: {} header: {}", url, header, ex);
      respond500(response, "Payment verification failed: " + ex.getMessage());
    } catch (Exception ex) {
      log.error("x402 unexpected verification error URL: {} header: {}", url, header, ex);
      respond500(response, "Internal server error during payment verification");
    }
    return null;
  }

  /**
   * Build immutable payment requirements for the request + annotation.
   */
  private PaymentRequirements buildRequirements(HttpServletRequest request, String path,
      X402Payment ann) {
    String priceStr = ann.price();
//    if (!StringUtils.hasText(priceStr)) {
//      throw new IllegalStateException("@X402Payment.price must not be empty");
//    }
    if (!StringUtils.hasText(priceStr) && ann.priceCalculator() == null) {
      throw new IllegalStateException(
          "Either @X402Payment.price or @X402Payment.priceCalculator must be provided and non-empty.");
    }

    // Use price if annotation price provided
    BigDecimal atomic = StringUtils.hasText(priceStr) ? toAtomicUnits(priceStr)
        : toAtomicUnits(priceCalculatorHelper.calculate(request, ann.priceCalculator()));

    String payTo =
        StringUtils.hasText(ann.payTo()) ? ann.payTo() : x402Configuration.getDefaultPayTo();

    PaymentRequirements pr = new PaymentRequirements();
    pr.scheme = x402Configuration.getScheme();
    pr.network = x402Configuration.getNetwork();
    pr.maxAmountRequired = atomic.toPlainString();
    pr.asset = x402Configuration.getAsset();
    pr.description = ann.description();
    pr.resource = path;
    pr.mimeType = x402Configuration.getMimeType();
    pr.payTo = payTo;
    pr.maxTimeoutSeconds = x402Configuration.getMaxTimeoutSeconds();
    pr.extra = x402Configuration.getExtra();
    pr.outputSchema = x402Configuration.getOutputSchema();
    return pr;
  }

  /**
   * Convert human-readable token units to atomic units based on configured decimals.
   */
  private BigDecimal toAtomicUnits(String human) {
    return new BigDecimal(human).movePointRight(x402Configuration.getAssetDecimals())
        .setScale(0, java.math.RoundingMode.DOWN);
  }
}
