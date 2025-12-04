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

package ai.saharalabs.x402.controller;

import ai.saharalabs.x402.server.annotation.X402Payment;
import ai.saharalabs.x402.test.BodyPriceCalculator;
import ai.saharalabs.x402.test.ParamPriceCalculator;
import ai.saharalabs.x402.test.TestPriceCalculator;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class TestController {

  @GetMapping("/hi")
  public String hi() {
    return "hi!";
  }

  @X402Payment(price = "0.01")
  @GetMapping("/pay")
  public String pay() {
    return "Payment";
  }

  @GetMapping("/price")
  @X402Payment(priceCalculator = TestPriceCalculator.class)
  public String price() {
    return "price";
  }

  @GetMapping("/paramPrice")
  @X402Payment(priceCalculator = ParamPriceCalculator.class)
  public String paramPrice() {
    return "price";
  }

  @PostMapping("/bodyPrice")
  @X402Payment(priceCalculator = BodyPriceCalculator.class)
  public String bodyPrice() {
    return "price";
  }
}
