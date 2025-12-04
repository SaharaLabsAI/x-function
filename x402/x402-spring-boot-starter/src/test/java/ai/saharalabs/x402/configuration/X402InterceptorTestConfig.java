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
import ai.saharalabs.x402.server.intereptor.X402Interceptor;
import ai.saharalabs.x402.server.intereptor.X402Interceptor.X402InterceptorBuilder;
import ai.saharalabs.x402.server.price.PriceCalculatorHelper;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.context.ApplicationContext;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Primary;

@Configuration
public class X402InterceptorTestConfig {

  @Bean
  @Primary
  public FacilitatorClient facilitatorClient() {
    return org.mockito.Mockito.mock(FacilitatorClient.class);
  }

  @Bean
  @ConditionalOnMissingBean
  public X402Interceptor x402Interceptor(X402Configuration properties,
      FacilitatorClient facilitatorClient, PriceCalculatorHelper priceCalculatorHelper) {
    X402InterceptorBuilder builder = X402Interceptor.builder();
    builder.facilitator(facilitatorClient);
    builder.priceCalculatorHelper(priceCalculatorHelper);
    builder.x402Configuration(properties);
    return builder.build();
  }

  @Bean
  public PriceCalculatorHelper priceCalculatorHelper(ApplicationContext applicationContext) {
    if (null == applicationContext) {
      throw new IllegalStateException(
          "x402 is enabled but no application context provided");
    }
    return new PriceCalculatorHelper(applicationContext);
  }
}
