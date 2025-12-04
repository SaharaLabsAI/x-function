package ai.saharalabs.x402.function.demo.config;

import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@Configuration
public class CorsConfig implements WebMvcConfigurer {

  private final AppCorsProps appCorsProps;

  public CorsConfig(AppCorsProps appCorsProps) {
    this.appCorsProps = appCorsProps;
  }

  @Override
  public void addCorsMappings(CorsRegistry registry) {
    registry.addMapping("/**")
        .allowedOriginPatterns(appCorsProps.getOrigins().toArray(String[]::new))
        .allowedMethods(appCorsProps.getMethods().toArray(String[]::new))
        .allowedHeaders(appCorsProps.getHeaders().toArray(String[]::new))
        .allowCredentials(Boolean.TRUE.equals(appCorsProps.getCredentials()))
        .maxAge(appCorsProps.getMaxAge() != null ? appCorsProps.getMaxAge() : 3600);
  }


}
