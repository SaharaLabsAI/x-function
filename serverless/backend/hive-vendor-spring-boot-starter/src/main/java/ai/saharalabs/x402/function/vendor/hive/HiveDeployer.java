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

package ai.saharalabs.x402.function.vendor.hive;

import ai.saharalabs.x402.function.vendor.DeploymentConfig;
import ai.saharalabs.x402.function.vendor.DeploymentStatus;
import ai.saharalabs.x402.function.vendor.IDeployVendor;
import ai.saharalabs.x402.function.vendor.VendorException;
import ai.saharalabs.x402.function.vendor.hive.common.ErrorCode;
import ai.saharalabs.x402.function.vendor.hive.converter.ServiceConverter;
import ai.saharalabs.x402.function.vendor.hive.dto.ServiceCreateResultDTO;
import ai.saharalabs.x402.function.vendor.hive.dto.ServiceResultDTO;
import java.util.HashMap;
import java.util.Objects;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class HiveDeployer implements IDeployVendor {

  private final HiveHttpClient hiveHttpClient;

  public HiveDeployer(HiveHttpClient hiveHttpClient) {
    this.hiveHttpClient = hiveHttpClient;
  }

  @Override
  public String deploy(DeploymentConfig config) {
    HiveResponse<ServiceCreateResultDTO> response =
        hiveHttpClient.createService(ServiceConverter.toRequest(config));

    if (Objects.nonNull(response) && !response.isSuccess()) {
      log.error("Failed to deploy service {} to hive: {} - {}",
          config.getName(), response.getErrCode(), response.getErrMessage());
      throw new VendorException(ErrorCode.VENDOR_ERROR.getCode(),
          "Failed to deploy service to hive.");
    }

    return response.getData().getId();
  }

  @Override
  public DeploymentStatus status(String id) {
    HiveResponse<ServiceResultDTO> response = hiveHttpClient.getServiceStatusById(id);

    if (Objects.nonNull(response) && !response.isSuccess()) {
      DeploymentStatus deploymentStatus = new DeploymentStatus();
      deploymentStatus.setId(id);
      deploymentStatus.setReady(false);
      deploymentStatus.setMessage(response.getErrMessage());
      deploymentStatus.setExtra(new HashMap<>());
      return deploymentStatus;
    }
    return convertStatus(response.getData());
  }

  private DeploymentStatus convertStatus(ServiceResultDTO dto) {
    DeploymentStatus status = new DeploymentStatus();
    status.setId(dto.getId());
    status.setName(dto.getName());
    status.setUrl(dto.getUrl());
    status.setReady(dto.getReady());
    status.setMessage(dto.getMessage());
    HashMap<String, Object> extra = new HashMap<>();
    extra.put("details", dto.getDeployStatuses());
    status.setExtra(extra);
    return status;
  }
}
