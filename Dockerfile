# Staging image (busybox:musl)
# FROM busybox:musl

# Production image (scratch)
FROM scratch
COPY ./dist/brickpack-sampleapp /
ENV PIPELINE development
ENV CLIENT_TOKEN 9admin9
EXPOSE 8000
ENTRYPOINT ["/brickpack-sampleapp"]
