FROM eclipse-temurin:21-jdk-jammy

# Set timezone to UTC
ENV TZ=UTC

# Create non-root user and group
RUN groupadd --system --gid 1000 appgroup && \
    useradd --system --uid 1000 --gid appgroup --shell /bin/false --home-dir /app appuser && \
    mkdir -p /app/jvm-logs /app/dumps && \
    chown -R appuser:appgroup /app

# Set working directory
WORKDIR /app

# Configure JVM options with reasonable defaults
ENV DEFAULT_JAVA_OPTS="\
-XX:+UseG1GC \
-XX:MaxRAMPercentage=75.0 \
-XX:InitiatingHeapOccupancyPercent=45 \
-XX:MaxGCPauseMillis=200 \
-XX:+HeapDumpOnOutOfMemoryError \
-XX:HeapDumpPath=/app/dumps \
-Xlog:gc*:file=/app/jvm-logs/gc.log:time,uptime,level,tags:filecount=5,filesize=10M \
"

#ENV JAVA_OPTS=""

# Copy the locally built Spring Boot executable JAR
# Assumes `mvn clean package` has been run and the JAR is in the target/ directory
COPY --chown=appuser:appgroup x402-function-rest-demo/target/x402-function-rest-demo-*.jar app.jar

# Switch to non-root user
USER appuser:appgroup

# Expose application port (default 8080)
EXPOSE 8080

## Entrypoint with support for additional JVM parameters and environment variables
#ENTRYPOINT ["sh", "-c", "exec java $DEFAULT_JAVA_OPTS $JAVA_OPTS -jar /app/app.jar $SPRING_BOOT_OPTS"]
ENTRYPOINT ["java", "-jar", "/app/app.jar"]
