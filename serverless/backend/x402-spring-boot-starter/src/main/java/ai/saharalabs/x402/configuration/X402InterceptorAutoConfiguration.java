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

package ai.saharalabs.x402.configuration;

import ai.saharalabs.x402.server.facilitator.FacilitatorClient;
import ai.saharalabs.x402.server.facilitator.HttpFacilitatorClient;
import ai.saharalabs.x402.server.intereptor.X402Interceptor;
import ai.saharalabs.x402.server.intereptor.X402Interceptor.X402InterceptorBuilder;
import ai.saharalabs.x402.server.price.PriceCalculatorHelper;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.autoconfigure.AutoConfiguration;
import org.springframework.boot.autoconfigure.condition.ConditionalOnClass;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.context.ApplicationContext;
import org.springframework.context.annotation.Bean;
import org.springframework.lang.NonNull;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@AutoConfiguration
@ConditionalOnClass(WebMvcConfigurer.class)
@EnableConfigurationProperties(X402Configuration.class)
@ConditionalOnProperty(prefix = "x402", name = "enabled", havingValue = "true")
@Slf4j
public class X402InterceptorAutoConfiguration {

  @ConditionalOnMissingBean
  @Bean
  public PriceCalculatorHelper priceCalculatorHelper(ApplicationContext applicationContext) {
    if (null == applicationContext) {
      throw new IllegalStateException(
          "x402 is enabled but no application context provided");
    }
    return new PriceCalculatorHelper(applicationContext);
  }

  @ConditionalOnMissingBean
  @Bean
  public FacilitatorClient x402FacilitatorClient(X402Configuration props) {
    if (props.getFacilitatorBaseUrl() == null || props.getFacilitatorBaseUrl().isBlank()) {
      throw new IllegalStateException(
          "x402.facilitator-base-url must be configured when x402 is enabled");
    }
    return new HttpFacilitatorClient(props.getFacilitatorBaseUrl());
  }

  /**
   * Provide the {@link X402Interceptor} bean (unless user already defines one) using builder for
   * extensibility.
   */
  @Bean
  @ConditionalOnMissingBean
  public X402Interceptor x402Interceptor(X402Configuration properties,
      FacilitatorClient facilitatorClient, PriceCalculatorHelper priceCalculatorHelper) {
    X402InterceptorBuilder builder = X402Interceptor.builder();
    builder.facilitator(facilitatorClient);
    builder.priceCalculatorHelper(priceCalculatorHelper);
    builder.x402Configuration(properties);
    X402Interceptor interceptor = builder.build();
    log.info(
        "x402 interceptor initialized enabled={} scheme={} network={} asset={} decimals={} payTo={} timeoutSeconds={} facilitatorBaseUrl={} mimeType={} outputSchemaSize={} priceCalculatorHelper={} beanClass={}",
        true,
        properties.getScheme(),
        properties.getNetwork(),
        properties.getAsset(),
        properties.getAssetDecimals(),
        properties.getDefaultPayTo(),
        properties.getMaxTimeoutSeconds(),
        properties.getFacilitatorBaseUrl(),
        properties.getMimeType(),
        properties.getExtra(),
        priceCalculatorHelper.getClass().getSimpleName(),
        interceptor.getClass().getName());
    return interceptor;
  }

  /**
   * Register the interceptor in Spring MVC registry. Users can override ordering by defining their
   * own WebMvcConfigurer.
   */
  @Bean
  public WebMvcConfigurer x402WebMvcConfigurer(X402Interceptor interceptor) {
    return new WebMvcConfigurer() {
      @Override
      public void addInterceptors(@NonNull InterceptorRegistry registry) {
        registry.addInterceptor(interceptor);
      }
    };
  }
}