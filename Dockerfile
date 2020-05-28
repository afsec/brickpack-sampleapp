# Staging image (busybox:musl)
# FROM busybox:musl

# Production image (scratch)
FROM scratch
COPY ./dist/brickpack-sampleapp /
ENV PIPELINE development
ENV BP_SERVER_TOKEN 9admin9
EXPOSE 8000
ENTRYPOINT ["/brickpack-sampleapp"]
