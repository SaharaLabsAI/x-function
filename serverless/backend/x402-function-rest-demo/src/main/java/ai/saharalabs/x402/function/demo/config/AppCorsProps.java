package ai.saharalabs.x402.function.demo.config;

import java.util.List;
import lombok.Getter;
import lombok.Setter;
import org.springframework.boot.context.properties.ConfigurationProperties;

@ConfigurationProperties(prefix = "app.cors")
@Getter
@Setter
public class AppCorsProps {

  private List<String> origins;
  private List<String> methods;
  private List<String> headers;
  private Boolean credentials;
  private Long maxAge;
}
