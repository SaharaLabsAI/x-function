package ai.saharalabs.x402.function.demo.controller;

import ai.saharalabs.x402.function.api.IService;
import ai.saharalabs.x402.function.api.command.ServiceCreationCmd;
import ai.saharalabs.x402.function.api.dto.ServiceCreateDTO;
import ai.saharalabs.x402.function.api.dto.ServiceDTO;
import ai.saharalabs.x402.function.demo.common.SingleResponse;
import ai.saharalabs.x402.server.annotation.X402Payment;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.util.ObjectUtils;
import org.springframework.util.StringUtils;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/apis/x402/v1")
@Tag(name = "Service Management", description = "APIs for managing services")
public class X402HiveController {

  private final IService service;

  @Value("${demo.git-repo.url}")
  private String demoGitRepoUrl;

  public X402HiveController(IService service) {
    this.service = service;
  }

  // Must set {} as outputSchema to avoid error.
  @Operation(summary = "Create a new service", description = "Creates a new service by GitHub repo and pays 0.01 USDC for deployment.")
  @X402Payment(price = "0.01") // 0.01 USDC
  @PostMapping("/services")
  public ResponseEntity<SingleResponse<ServiceCreateDTO>> create(
      @RequestBody @Validated ServiceCreationCmd cmd) {
    // TODO for demo purpose
    if (!demoGitRepoUrl.equals(cmd.getUrl())) {
      throw new IllegalArgumentException("Only demo git repo is allowed: " + demoGitRepoUrl);
    }
    // TODO for demo purpose, generate a name if not provided
    if (!StringUtils.hasLength(cmd.getName())) {
      String generatedName = generateServiceName("demo");
      cmd.setName(generatedName);
    }
    // TODO for demo purpose, set default port if not provided
    if (ObjectUtils.isEmpty(cmd.getPort())) {
      cmd.setPort(8081);
    }
    // TODO for demo purpose, set default branch if not provided
    if (!StringUtils.hasLength(cmd.getBranch())) {
      cmd.setBranch("main");
    }

    return ResponseEntity.status(HttpStatus.CREATED).body(SingleResponse.of(service.create(cmd)));
  }

  // TODO for demo purpose, generate a unique service name
  private String generateServiceName(String prefix) {
    long timestamp = System.currentTimeMillis();
    return prefix + "-" + timestamp;
  }

  @Operation(summary = "Get service status", description = "Gets the status of a service by its ID.")
  @GetMapping("/services/{id}")
  public ResponseEntity<SingleResponse<ServiceDTO>> statusServiceById(
      @PathVariable("id") String id) {
    return ResponseEntity.ok(SingleResponse.of(service.status(id)));
  }
}