# x402 Spring Boot Starter

## Overview

The x402 Spring Boot Starter provides seamless integration of the x402 Server SDK and vendor SPI
into Spring Boot applications. It enables automatic payment interception and settlement for web
endpoints, supporting blockchain-based payments (e.g., USDC on Base Sepolia).

## Features

- Integrates with vendor SPI for payment verification and settlement.
- Integrates with x402 Server SDK for blockchain payment processing.
- Auto-configures payment interceptors for Spring MVC endpoints.
- Supports custom payment requirements via annotations.
- Supports both static and dynamic pricing via pluggable price calculators.

## Installation

Add the following dependency to your Maven `pom.xml`:

```xml
<dependency>
  <groupId>ai.saharalabs.x402-function</groupId>
  <artifactId>x402-spring-boot-starter</artifactId>
  <version>1.0-SNAPSHOT</version>
</dependency>
```

## Configuration

Configure the starter in your `application.properties`:

```properties
x402.enabled=true
x402.scheme=exact
x402.default-pay-to=your-payee-address
x402.network=base-sepolia
x402.asset=USDC
x402.max-timeout-seconds=30
x402.facilitator-base-url=https://facilitator.example.com
x402.extra.name=USDC
x402.extra.version=2
x402.asset-decimals=6
x402.mimeType=application/json
#x402.outputSchema.<key>=<value>
```

## Usage

Annotate your controller or method with `@X402Payment` to require payment for access (static price
example):

```java
@RestController
public class DemoController {

  // "10000" (atomic units) = 0.01 USDC when asset-decimals = 6
  @X402Payment(price = "10000", payTo = "your-payee-address")
  @GetMapping("/protected")
  public String protectedEndpoint() {
    return "This endpoint requires payment.";
  }
}
```

## Pricing & Dynamic Price Calculators

x402 supports two pricing strategies:

1. Static price: Provide a literal amount in `@X402Payment.price`.
2. Dynamic price: Supply a calculator implementation via `@X402Payment.priceCalculator` that derives
   the human-readable price from the incoming request (query params, headers, body, etc.).

### Priority Rules

- If `price` is non-empty, it is used directly (static pricing wins).
- If `price` is empty, the framework invokes the configured `priceCalculator` class.
- The default calculator is `DefaultPriceCalculator.class` (returns empty / requires override).
- If both `price` and the calculator result are empty, an exception is thrown:
  `Either @X402Payment.price or @X402Payment.priceCalculator must be provided and non-empty.`

### Calculator Interface

```java
public interface IPriceCalculator {

  /**
   * @param request current HTTP request
   * @return human-readable token amount, e.g. "0.03" (NOT atomic units)
   */
  String calculatePrice(HttpServletRequest request);
}
```

### Custom Calculator Examples

Query parameter driven pricing:

```java

@Component
public class ParamPriceCalculator implements IPriceCalculator {

  @Override
  public String calculatePrice(HttpServletRequest request) {
    String tier = request.getParameter("tier");
    if ("pro".equalsIgnoreCase(tier))
      return "0.12"; // 0.12 USDC
    if ("basic".equalsIgnoreCase(tier))
      return "0.03"; // 0.03 USDC
    return "0.01"; // fallback
  }
}
```

Controller usage:

```java

@RestController
public class DynamicPriceController {

  @X402Payment(priceCalculator = ParamPriceCalculator.class)
  @GetMapping("/paramPrice")
  public String byParam() {
    return "pricing via param";
  }
}
```

### Atomic Unit Conversion (`asset-decimals`)

- Human-readable price is converted to atomic units using `x402.asset-decimals`.
- Formula: `atomic = humanPrice * 10^(decimals)` (rounded down to integer string).
- Example: `0.03` USDC at 6 decimals → `30000` atomic units.
- The 402 response field `maxAmountRequired` contains the atomic unit string.

### Annotation Parameter Reference (`@X402Payment`)

| Field           | Type                              | Required | Description                                                                                              |
|-----------------|-----------------------------------|----------|----------------------------------------------------------------------------------------------------------|
| price           | String                            | No       | Human-readable price (e.g. "0.03") or atomic unit string (e.g. "30000"). If non-empty, takes precedence. |
| payTo           | String                            | No       | Overrides default recipient address; falls back to `x402.default-pay-to`.                                |
| description     | String                            | No       | Human-friendly description returned in the 402 response.                                                 |
| priceCalculator | Class<? extends IPriceCalculator> | No       | Calculator used when `price` is empty. Default `DefaultPriceCalculator.class`.                           |

## How It Works

- The starter auto-configures a payment interceptor (`X402Interceptor`) for your web endpoints.
- Payment requirements can be set globally or per endpoint using the `@X402Payment` annotation.
- The interceptor verifies and settles payments using the configured facilitator client.

## Protocol Specification

Refer to
the [x402 Protocol Specification](https://github.com/coinbase/x402/blob/main/specs/x402-specification.md)
for details.

## License

See the LICENSE file for details.

## Notice

See the NOTICE file for details.
