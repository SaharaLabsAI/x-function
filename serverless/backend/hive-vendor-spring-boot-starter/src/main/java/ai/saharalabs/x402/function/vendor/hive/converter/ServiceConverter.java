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

package ai.saharalabs.x402.function.vendor.hive.converter;

import ai.saharalabs.x402.function.vendor.DeploymentConfig;
import ai.saharalabs.x402.function.vendor.DeploymentRunConfig;
import ai.saharalabs.x402.function.vendor.DeploymentSourceConfig;
import ai.saharalabs.x402.function.vendor.hive.dto.HiveServiceCreateRequest;

public class ServiceConverter {

  public static HiveServiceCreateRequest toRequest(DeploymentConfig config) {
    HiveServiceCreateRequest request = new HiveServiceCreateRequest();
    request.setName(config.getName());

    HiveServiceCreateRequest.Configuration configuration = new HiveServiceCreateRequest.Configuration();
    DeploymentSourceConfig sourceConfig = config.getSourceConfig();
    // TODO only support GIT type for now.
    configuration.setSourceType("GIT");
    configuration.setSourceUri(sourceConfig.getGit());
    configuration.setSourceBranch(sourceConfig.getBranch());
    configuration.setSourceContextDir(sourceConfig.getDir());

    DeploymentRunConfig runConfig = config.getRunConfig();
    configuration.setPort(runConfig.getPort());

    request.setConfiguration(configuration);
    return request;
  }
}
