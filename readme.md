# KFS

## Quick start

To initialize the container, please follow the instructions below:

```bash
docker compose up -d
```

After initialization, you have the capability to build the project using the command:

```bash
docker compose exec kfs make
```

Following this step, the ISO image will be built, and you can run it on qemu (on the host machine):

```bash
make run
```

Please note that a volume is set up to establish a link between the directory on your host machine and the container. This provides you with the flexibility to develop without the need for constant container restarts or rebuilding the image.

Ensure that the software qemu-system-x86_64 is properly installed on your host machine in order to execute the make run command.

## Authors

[mli42](https://github.com/mli42)

[lsarrazi](https://github.com/lsarrazi)