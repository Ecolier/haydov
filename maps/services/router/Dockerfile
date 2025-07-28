FROM ghcr.io/valhalla/valhalla:3.5.1 AS builder

WORKDIR /app

COPY ./valhalla.json* ./valhalla.json

RUN if [ ! -e valhalla.json ]; then \
  valhalla_build_config \
  --mjolnir-tile-dir ${PWD}/tiles \
  --mjolnir-tile-extract ${PWD}/tiles.tar \
  --mjolnir-timezone ${PWD}/tiles/timezones.sqlite \
  --mjolnir-admin ${PWD}/tiles/admins.sqlite \
  > valhalla.json ; fi

RUN --mount=type=bind,source=./data,target=./data \
  valhalla_build_admins -c valhalla.json ./data/*.osm.pbf & \
  valhalla_build_tiles -c valhalla.json ./data/*.osm.pbf

RUN valhalla_build_extract -c valhalla.json -v

FROM ubuntu:latest AS runner

RUN apt-get update && apt-get install -y libprotobuf-lite32 libcurl4 libgdal34 libzmq5 libczmq4

COPY --from=builder /usr/local /usr/local

WORKDIR /app

ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:$PATH
ENV LD_LIBRARY_PATH=/usr/local/lib:/lib/x86_64-linux-gnu:/usr/lib/x86_64-linux-gnu:/lib32:/usr/lib32

COPY --from=builder /app/valhalla.json ./valhalla.json
COPY --from=builder /app/tiles.tar ./tiles.tar

EXPOSE 8002
CMD [ "valhalla_service", "valhalla.json", "1" ]