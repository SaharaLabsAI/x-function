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

package ai.saharalabs.x402.function.service;

import ai.saharalabs.x402.function.api.IService;
import ai.saharalabs.x402.function.api.command.ServiceCreationCmd;
import ai.saharalabs.x402.function.api.dto.ServiceCreateDTO;
import ai.saharalabs.x402.function.api.dto.ServiceDTO;
import ai.saharalabs.x402.function.converter.ServiceConverter;
import ai.saharalabs.x402.function.vendor.DeploymentStatus;
import ai.saharalabs.x402.function.vendor.IDeployVendor;

public class ServiceImpl implements IService {

  private final IDeployVendor deployVendor;

  public ServiceImpl(IDeployVendor deployVendor) {
    this.deployVendor = deployVendor;
  }

  @Override
  public ServiceCreateDTO create(ServiceCreationCmd cmd) {
    String id = deployVendor.deploy(ServiceConverter.toDeploymentConfig(cmd));
    return ServiceCreateDTO.builder().id(id).name(cmd.getName()).build();
  }

  @Override
  public ServiceDTO status(String id) {
    DeploymentStatus status = deployVendor.status(id);
    return ServiceConverter.toDTO(status);
  }
}
