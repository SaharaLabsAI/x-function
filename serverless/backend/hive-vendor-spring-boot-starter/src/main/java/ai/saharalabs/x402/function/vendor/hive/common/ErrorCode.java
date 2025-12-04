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

package ai.saharalabs.x402.function.vendor.hive.common;

import lombok.Getter;

@Getter
public enum ErrorCode {
  SYSTEM_ERROR("SYSTEM_ERROR",
      "An unexpected system error occurred. Please contact support if the issue persists."),
  BAD_REQUEST("BAD_REQUEST", "One or more request parameters are invalid"),
  SERVICE_NOT_FOUND("SERVICE_NOT_FOUND", "Service not found"),
  USER_NOT_AUTHORIZED("USER_NOT_AUTHORIZED", "User not authorized"),

  VENDOR_ERROR("VENDOR_ERROR", "An error occurred while communicating with the vendor");

  private final String code;
  private final String message;

  ErrorCode(String code, String message) {
    this.code = code;
    this.message = message;
  }
}
