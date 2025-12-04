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

package ai.saharalabs.x402.test;

import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.content;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.result.MockMvcResultHandlers;

@SpringBootTest(classes = ai.saharalabs.x402.X402SpringBootStarterTestApplication.class)
@AutoConfigureMockMvc
class X402InterceptorWebTest {

  @Autowired
  MockMvc mockMvc;

  @Test
  void testNoPaymentAnnotation_ShouldPass() throws Exception {
    mockMvc.perform(get("/hi"))
        .andExpect(status().isOk())
        .andExpect(content().string("hi!"));
  }

  @Test
  void testPaymentAnnotation_ShouldPass() throws Exception {
    mockMvc.perform(get("/pay"))
        .andDo(MockMvcResultHandlers.print())
        .andExpect(status().isPaymentRequired());
  }

  @Test
  void testPaymentAnnotationWithPriceCalculator_ShouldPass() throws Exception {
    mockMvc.perform(get("/price"))
        .andDo(MockMvcResultHandlers.print())
        .andExpect(status().isPaymentRequired())
        .andExpect(jsonPath("$.accepts[0].maxAmountRequired").value("120000"));
  }

  @Test
  void testPaymentAnnotationWithParamPriceCalculator_ShouldPass() throws Exception {
    mockMvc.perform(get("/paramPrice").param("param", "1"))
        .andDo(MockMvcResultHandlers.print())
        .andExpect(status().isPaymentRequired())
        .andExpect(jsonPath("$.accepts[0].maxAmountRequired").value("110000"));
  }

  @Test
  void testPaymentAnnotationWithBodyPriceCalculator_ShouldPass() throws Exception {
    mockMvc.perform(post("/bodyPrice")
            .content("{\"price\":\"0.03\"}")
            .contentType(MediaType.APPLICATION_JSON))
        .andDo(MockMvcResultHandlers.print())
        .andExpect(status().isPaymentRequired())
        .andExpect(jsonPath("$.accepts[0].maxAmountRequired").value("30000"));
  }
}
