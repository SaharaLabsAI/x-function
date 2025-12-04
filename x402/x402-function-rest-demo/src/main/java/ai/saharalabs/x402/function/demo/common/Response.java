package ai.saharalabs.x402.function.demo.common;

import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

@Getter
@Setter
@ToString
public class Response extends DTO {

  private boolean success;
  private String code;
  private String message;

  public static Response buildSuccess() {
    Response response = new Response();
    response.setSuccess(true);
    return response;
  }

  public static Response buildFailure(String code, String message) {
    Response response = new Response();
    response.setSuccess(false);
    response.setCode(code);
    response.setMessage(message);
    return response;
  }
}
