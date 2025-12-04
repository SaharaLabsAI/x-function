# x402-function-spring-boot-starter

## Overview

This project provides a thin implementation layer for integrating x402 Server functionality into
Spring Boot applications.

## Installation

Add the following dependency to your Maven `pom.xml`:

```xml
<dependency>
    <groupId>ai.saharalabs</groupId>
    <artifactId>x402-function-spring-boot-starter</artifactId>
    <version>1.0-SNAPSHOT</version>
</dependency>
```

## Configuration

Configure the starter in your `application.properties` file:

```properties
# x402 configuration
x402.enabled=true
x402.default-pay-to=
x402.network=base-sepolia
x402.asset=0x036CbD53842c5426634e7929541eC2318f3dCF7e
x402.max-timeout-seconds=30
x402.facilitator-base-url=https://www.x402.org/facilitator
```

## Usage

Annotate your Spring Boot application to enable the x402 function starter:

```java
@SpringBootApplication
public class Application {
    public static void main(String[] args) {
        SpringApplication.run(Application.class, args);
    }
}
```

You can now inject and use x402-related beans provided by the starter in your services or
controllers.

## License

See the LICENSE file for details.

## Notice

See the NOTICE file for details.
