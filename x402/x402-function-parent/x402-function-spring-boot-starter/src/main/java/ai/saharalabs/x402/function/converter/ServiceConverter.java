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

package ai.saharalabs.x402.function.converter;


import ai.saharalabs.x402.function.api.command.ServiceCreationCmd;
import ai.saharalabs.x402.function.api.dto.ServiceDTO;
import ai.saharalabs.x402.function.vendor.DeploymentConfig;
import ai.saharalabs.x402.function.vendor.DeploymentRunConfig;
import ai.saharalabs.x402.function.vendor.DeploymentSourceConfig;
import ai.saharalabs.x402.function.vendor.DeploymentStatus;

public class ServiceConverter {

  public static DeploymentConfig toDeploymentConfig(ServiceCreationCmd cmd) {
    DeploymentConfig config = new DeploymentConfig();
    config.setName(cmd.getName());
    DeploymentSourceConfig sourceConfig = new DeploymentSourceConfig();
    sourceConfig.setGit(cmd.getUrl());
    sourceConfig.setBranch(cmd.getBranch());
    sourceConfig.setDir(cmd.getDir());

    DeploymentRunConfig runConfig = new DeploymentRunConfig();
    runConfig.setPort(cmd.getPort());

    config.setSourceConfig(sourceConfig);
    config.setRunConfig(runConfig);
    return config;
  }

  public static ServiceDTO toDTO(DeploymentStatus status) {
    ServiceDTO dto = new ServiceDTO();
    dto.setId(status.getId());
    dto.setName(status.getName());
    dto.setReady(status.getReady());
    dto.setUrl(status.getUrl());
    dto.setMessage(status.getMessage());
    dto.setExtra(status.getExtra());
    return dto;
  }
}
