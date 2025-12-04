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

package ai.saharalabs.x402.function.vendor.hive.dto;

import java.util.List;
import lombok.Builder;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class HiveServiceCreateRequest {

  private String name;
  private Configuration configuration;

  @Getter
  @Setter
  public static class Configuration {

    private String sourceType;
    private String sourceUri;
    private String sourceBranch;
    private String sourceContextDir;
    private Integer port;
    private List<Env> envs;
    private Integer concurrencyLimit;
    private String readinessProbe;
    private String livenessProbe;
    private String cpuRequest;
    private String memoryRequest;
    private String cpuLimit;
    private String memoryLimit;
    private Integer minScale;
    private Integer maxScale;
    private Integer initScale;
    private String windowScale;
    private String metric;
    private Integer target;
    private Integer utilization;
    private String dockerConfig;
    private String pvcSize;
    private List<Env> buildEnvs;

    @Getter
    @Setter
    @Builder
    public static class Env {

      private String name;
      private String value;
    }
  }
}
