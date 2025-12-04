# Vendor SPI

This module provides a Service Provider Interface (SPI) for integrating custom deployment vendors
with the x402-function ecosystem. Developers can implement their own vendor logic by following the
interface contract.

## How to Develop Your Own Vendor

1. **Implement the `IDeployVendor` Interface**

Create a class that implements the `ai.saharalabs.x402.function.vendor.IDeployVendor` interface.
This interface defines the required methods for vendor integration:

```java
public interface IDeployVendor {

  String getVendorId();

  String deploy(DeploymentConfig config) throws VendorException;

  DeploymentStatus status(String id) throws VendorException;
}
```

- `getVendorId()`: Returns a unique identifier for your vendor (e.g., "hive", "vercel").
- `deploy(DeploymentConfig config)`: Handles deployment logic using the provided configuration.
  Returns a deployment ID or reference.
- `status(String id)`: Returns the current status of a deployment by its ID.

2. **Handle Deployment Logic**

Implement the deployment and status checking logic specific to your vendor platform. Throw
`VendorException` for error handling.

3. **Register Your Vendor**

Ensure your implementation is discoverable by the x402-function system (e.g., via Spring Boot
auto-configuration, SPI registration, or explicit wiring).

## Example

```java
public class MyVendor implements IDeployVendor {

  @Override
  public String getVendorId() {
    return "myvendor";
  }

  @Override
  public String deploy(DeploymentConfig config) throws VendorException {
    // Implement deployment logic here
    return "deployment-id";
  }

  @Override
  public DeploymentStatus status(String id) throws VendorException {
    // Implement status check logic here
    return new DeploymentStatus();
  }
}
```

## Notes

- Ensure your vendor implementation is robust and handles errors gracefully.
- Consult the SPI documentation and existing vendor examples for best practices.

## License

See the LICENSE file for details.

