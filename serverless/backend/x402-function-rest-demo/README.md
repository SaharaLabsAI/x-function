# x402-function-rest-demo Dependency Usage Guide

## Overview
This project demonstrates how to use the x402-function BOM and x402-function-spring-boot-starter in a Spring Boot application.

## Quick Start

### 1. Add Dependencies
Add the following to your `pom.xml`:

```xml
<dependencyManagement>
  <dependencies>
    <!-- x402-function BOM for version alignment -->
    <dependency>
      <groupId>ai.saharalabs.x402-function</groupId>
      <artifactId>x402-function-bom</artifactId>
      <version>1.0-SNAPSHOT</version>
      <type>pom</type>
      <scope>import</scope>
    </dependency>
  </dependencies>
</dependencyManagement>

<dependencies>
  <!-- x402-function Spring Boot Starter -->
  <dependency>
    <groupId>ai.saharalabs.x402-function</groupId>
    <artifactId>x402-function-spring-boot-starter</artifactId>
  </dependency>
  <!-- x402-function Spring Boot Starter end -->
  <!-- Hive Vendor Spring Boot Starter -->
  <dependency>
    <artifactId>hive-vendor-spring-boot-starter</artifactId>
    <groupId>ai.saharalabs.x402-function</groupId>
    <version>1.0-SNAPSHOT</version>
  </dependency>
  <!-- Hive Vendor Spring Boot Starter end -->
</dependencies>
```

## Notes
- The BOM ensures consistent versions for all x402-function modules.
- The starter provides core functionality for x402-function integration in your Spring Boot project.

## Reference
- [x402-function Documentation](https://github.com/saharalabs/x402-function)

For more details, refer to the official documentation or the source code.
