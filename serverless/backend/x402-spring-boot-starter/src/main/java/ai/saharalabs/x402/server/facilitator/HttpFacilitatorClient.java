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
import ai.saharalabs.x402.util.Json;
import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * Synchronous facilitator client using Java 17 HttpClient.
 */
public class HttpFacilitatorClient implements FacilitatorClient {

  private final HttpClient http =
      HttpClient.newBuilder()
          .connectTimeout(Duration.ofSeconds(5))
          .build();

  private final String baseUrl;   // without trailing “/”

  /**
   * Creates a new HTTP facilitator client.
   *
   * @param baseUrl the base URL of the facilitator service (trailing slash will be removed)
   */
  public HttpFacilitatorClient(String baseUrl) {
    this.baseUrl = baseUrl.endsWith("/")
        ? baseUrl.substring(0, baseUrl.length() - 1)
        : baseUrl;
  }

  /* ------------------------------------------------ verify ------------- */

  @Override
  public VerificationResponse verify(PaymentPayload paymentPayload,
      PaymentRequirements req)
      throws IOException, InterruptedException {

    Map<String, Object> body = Map.of(
        "x402Version", 1,
        "paymentPayload", paymentPayload,
        "paymentRequirements", req
    );

    HttpRequest request = HttpRequest.newBuilder()
        .uri(URI.create(baseUrl + "/verify"))
        .header("Content-Type", "application/json")
        .POST(HttpRequest.BodyPublishers.ofString(
            Json.MAPPER.writeValueAsString(body)))
        .build();

    HttpResponse<String> response = http.send(request, HttpResponse.BodyHandlers.ofString());
    if (response.statusCode() != 200) {
      throw new IOException("HTTP " + response.statusCode() + ": " + response.body());
    }
    return Json.MAPPER.readValue(response.body(), VerificationResponse.class);
  }

  /* ------------------------------------------------ settle ------------- */

  @Override
  public SettlementResponse settle(PaymentPayload paymentPayload,
      PaymentRequirements req)
      throws IOException, InterruptedException {

    Map<String, Object> body = Map.of(
        "x402Version", 1,
        "paymentPayload", paymentPayload,
        "paymentRequirements", req
    );

    HttpRequest request = HttpRequest.newBuilder()
        .uri(URI.create(baseUrl + "/settle"))
        .header("Content-Type", "application/json")
        .POST(HttpRequest.BodyPublishers.ofString(
            Json.MAPPER.writeValueAsString(body)))
        .build();

    HttpResponse<String> response = http.send(request, HttpResponse.BodyHandlers.ofString());
    if (response.statusCode() != 200) {
      throw new IOException("HTTP " + response.statusCode() + ": " + response.body());
    }
    return Json.MAPPER.readValue(response.body(), SettlementResponse.class);
  }

  /* ------------------------------------------------ supported ---------- */

  @Override
  public Set<Kind> supported() throws IOException, InterruptedException {
    HttpRequest request = HttpRequest.newBuilder()
        .uri(URI.create(baseUrl + "/supported"))
        .GET()
        .build();

    HttpResponse<String> response = http.send(request, HttpResponse.BodyHandlers.ofString());
    if (response.statusCode() != 200) {
      throw new IOException("HTTP " + response.statusCode() + ": " + response.body());
    }

    @SuppressWarnings("unchecked")
    Map<String, Object> map = Json.MAPPER.readValue(response.body(), Map.class);
    List<?> kinds = (List<?>) map.getOrDefault("kinds", List.of());

    Set<Kind> out = new HashSet<>();
    for (Object k : kinds) {
      out.add(Json.MAPPER.convertValue(k, Kind.class));
    }
    return out;
  }
}
