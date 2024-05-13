# KFS

## Prerequisites

- [QEMU](https://www.qemu.org/download/) (We use `qemu-system-i386` to boot our image)
- [Docker compose](https://docs.docker.com/compose/install/) (used to compile kernel dependencies)

## Quick start

To initialize the container, please follow the instructions below:

```bash
make docker.up
# equivalent to
docker compose up -d
```

After initialization, you have the capability to build the project using the command:

```bash
make docker.build
# equivalent to
docker compose exec kfs make
```

Following this step, the ISO image will be built, and you can run it on qemu (on the host machine):

```bash
make run
```

Please note that a volume is set up to establish a link between the directory on your host machine and the container. This provides you with the flexibility to develop without the need for constant container restarts or rebuilding the image.

## Authors

[mli42](https://github.com/mli42)

[lsarrazi](https://github.com/lsarrazi)
