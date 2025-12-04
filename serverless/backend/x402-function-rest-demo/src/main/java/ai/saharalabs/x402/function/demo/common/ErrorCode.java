package ai.saharalabs.x402.function.demo.common;

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
