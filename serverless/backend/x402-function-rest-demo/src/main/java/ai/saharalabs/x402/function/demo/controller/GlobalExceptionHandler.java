package ai.saharalabs.x402.function.demo.controller;

import ai.saharalabs.x402.function.demo.common.BizException;
import ai.saharalabs.x402.function.demo.common.ErrorCode;
import ai.saharalabs.x402.function.demo.common.Response;
import ai.saharalabs.x402.function.vendor.VendorException;
import jakarta.servlet.http.HttpServletRequest;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;


@Slf4j
@RestControllerAdvice
public class GlobalExceptionHandler {

  /**
   * Handle JSR 303 Exception
   *
   * @param ex MethodArgumentNotValidException
   * @return 400 with response
   */
  @ExceptionHandler(MethodArgumentNotValidException.class)
  public ResponseEntity<Response> handleValidationExceptions(MethodArgumentNotValidException ex) {
    StringBuilder sb = new StringBuilder();
    ex.getBindingResult().getFieldErrors().forEach(error -> {
      sb.append(error.getField()).append(": ").append(error.getDefaultMessage()).append("; ");
    });
    String msg = !sb.isEmpty() ? sb.toString() : ErrorCode.BAD_REQUEST.getMessage();
    return ResponseEntity.status(HttpStatus.BAD_REQUEST)
        .body(Response.buildFailure(ErrorCode.BAD_REQUEST.getCode(), msg));
  }

  @ExceptionHandler(IllegalArgumentException.class)
  public ResponseEntity<Response> handleIllegalArgumentException(IllegalArgumentException ex) {
    return new ResponseEntity<>(
        Response.buildFailure(ErrorCode.BAD_REQUEST.getCode(), ex.getMessage()),
        HttpStatus.BAD_REQUEST);
  }

  @ExceptionHandler(BizException.class)
  public ResponseEntity<Response> handleBusinessException(BizException ex) {
    return ResponseEntity.status(HttpStatus.BAD_REQUEST)
        .body(Response.buildFailure(ex.getCode(), ex.getMessage()));
  }

  @ExceptionHandler(VendorException.class)
  public ResponseEntity<Response> handleVendorException(VendorException ex) {
    return ResponseEntity.status(HttpStatus.BAD_GATEWAY)
        .body(Response.buildFailure(ex.getCode(), ex.getMessage()));
  }

  /**
   * Handle all other exceptions
   *
   * @param ex Exception
   * @return 500 with response
   */
  @ExceptionHandler(Throwable.class)
  public ResponseEntity<Response> handleAllExceptions(Exception ex, HttpServletRequest request) {
    Response resp = Response.buildFailure(ErrorCode.SYSTEM_ERROR.getCode(),
        ErrorCode.SYSTEM_ERROR.getMessage());
    log.error("[EXCEPTION] {} {} | params: {} | response: {} | error: {}", request.getMethod(),
        request.getRequestURI(), request.getQueryString(), resp, ex.getMessage(), ex);
    return new ResponseEntity<>(resp, HttpStatus.INTERNAL_SERVER_ERROR);
  }
}
