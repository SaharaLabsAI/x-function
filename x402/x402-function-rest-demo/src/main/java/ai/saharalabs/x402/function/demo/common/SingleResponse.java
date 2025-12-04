package ai.saharalabs.x402.function.demo.common;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class SingleResponse<T> extends Response {

  private T data;

  public static SingleResponse buildSuccess() {
    SingleResponse response = new SingleResponse();
    response.setSuccess(true);
    return response;
  }

  public static SingleResponse buildFailure(String code, String message) {
    SingleResponse response = new SingleResponse();
    response.setSuccess(false);
    response.setCode(code);
    response.setMessage(message);
    return response;
  }

  public static <T> SingleResponse<T> of(T data) {
    SingleResponse<T> response = new SingleResponse<>();
    response.setSuccess(true);
    response.setData(data);
    return response;
  }
}
