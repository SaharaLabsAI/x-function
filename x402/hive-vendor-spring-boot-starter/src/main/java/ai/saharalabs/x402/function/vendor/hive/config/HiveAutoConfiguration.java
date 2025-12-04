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

package ai.saharalabs.x402.function.vendor.hive.config;

import ai.saharalabs.x402.function.vendor.IDeployVendor;
import ai.saharalabs.x402.function.vendor.VendorException;
import ai.saharalabs.x402.function.vendor.hive.HiveDeployer;
import ai.saharalabs.x402.function.vendor.hive.HiveHttpClient;
import ai.saharalabs.x402.function.vendor.hive.HiveResponse;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.autoconfigure.AutoConfiguration;
import org.springframework.boot.autoconfigure.condition.ConditionalOnClass;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatusCode;
import org.springframework.http.client.ClientHttpResponse;
import org.springframework.web.client.RestClient;
import org.springframework.web.util.UriComponentsBuilder;

@Slf4j
@AutoConfiguration
@EnableConfigurationProperties(HiveApiProperties.class)
@ConditionalOnClass({RestClient.class, ObjectMapper.class, IDeployVendor.class})
@ConditionalOnProperty(prefix = "hive.api", name = "enabled", havingValue = "true", matchIfMissing = true)
public class HiveAutoConfiguration {

  @Bean
  @ConditionalOnMissingBean(name = "hiveRestClient")
  public RestClient hiveRestClient(HiveApiProperties props, ObjectMapper objectMapper) {
    String url = UriComponentsBuilder
        .fromHttpUrl(props.getBaseUrl())
        .pathSegment(props.getAccount())
        .build()
        .toUriString();

    return RestClient.builder()
        .baseUrl(url)
        .defaultHeader(props.getTokenHeaderName(), props.getToken())
        .defaultStatusHandler(HttpStatusCode::isError, (request, response) -> {
          String body = readBodySafely(response);
          HiveResponse<?> hiveResponse = tryParseHiveResponse(objectMapper, body);
          String code =
              hiveResponse != null && hiveResponse.getErrCode() != null
                  ? hiveResponse.getErrCode()
                  : String.valueOf(response.getStatusCode().value());

          String message =
              hiveResponse != null && hiveResponse.getErrMessage() != null
                  ? hiveResponse.getErrMessage()
                  : "HTTP " + response.getStatusCode().value() + " " + body;

          throw new VendorException(code, message);
        })
        .build();
  }

  private String readBodySafely(ClientHttpResponse response) {
    try (InputStream is = response.getBody()) {
      return new String(is.readAllBytes(), StandardCharsets.UTF_8);
    } catch (IOException e) {
      return "";
    }
  }

  private HiveResponse<?> tryParseHiveResponse(ObjectMapper objectMapper, String body) {
    if (body == null || body.isEmpty()) {
      return null;
    }
    try {
      return objectMapper.readValue(body, new TypeReference<>() {
      });
    } catch (Exception e) {
      return null;
    }
  }

  @Bean
  @ConditionalOnMissingBean
  public HiveHttpClient hiveHttpClient(RestClient hiveRestClient) {
    return new HiveHttpClient(hiveRestClient);
  }

  @Bean
  @ConditionalOnMissingBean(IDeployVendor.class)
  public IDeployVendor hiveDeployer(HiveHttpClient hiveHttpClient) {
    log.info("Loaded hive deployer...");
    return new HiveDeployer(hiveHttpClient);
  }
}
