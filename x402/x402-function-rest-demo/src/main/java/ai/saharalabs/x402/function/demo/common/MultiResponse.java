package ai.saharalabs.x402.function.demo.common;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.List;

public class MultiResponse<T> extends Response {

  private Collection<T> data;

  public static MultiResponse buildSuccess() {
    MultiResponse response = new MultiResponse();
    response.setSuccess(true);
    return response;
  }

  public static MultiResponse buildFailure(String code, String message) {
    MultiResponse response = new MultiResponse();
    response.setSuccess(false);
    response.setCode(code);
    response.setMessage(message);
    return response;
  }

  public static <T> MultiResponse<T> of(Collection<T> data) {
    MultiResponse<T> response = new MultiResponse<>();
    response.setSuccess(true);
    response.setData(data);
    return response;
  }

  public List<T> getData() {
    if (null == data) {
      return Collections.emptyList();
    }
    if (data instanceof List) {
      return (List<T>) data;
    }
    return new ArrayList<>(data);
  }

  public void setData(Collection<T> data) {
    this.data = data;
  }

  public boolean isNotEmpty() {
    return !isEmpty();
  }

  public boolean isEmpty() {
    return data == null || data.isEmpty();
  }
}
