# Staging image (busybox:musl)
# FROM busybox:musl

# Production image (scratch)
FROM scratch
ENV PIPELINE development
COPY ./dist/brickpack-sampleapp /
EXPOSE 8000
ENTRYPOINT ["/brickpack-sampleapp"]
