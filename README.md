# 1brc challenge


- Create a workspace and generate measurements.txt using the original Java program via docker
```
 original 
mkdir 1brc-rust && cd 1brc-rust && git clone git@github.com:gunnarmorling/1brc.git && docker run --rm -it -v ./1brc:/1brc -w /1brc openjdk:21-rc-bookworm bash -c "./mvnw clean verify && ./create_measurements.sh 1000000000" && mv 1brc/measurements.txt .
```

- Since `docker run` runs with `root` and the docker command is very simple. Cleaning up the cloned will require `sudo rm -rf 1brc/` due to compilation artifacts belonging to another user.

- Verify measuremetnst against the original
```
docker run --rm -it -v ./1brc:/1brc -w /1brc openjdk:21-rc-bookworm bash -c "./mvnw clean verify && ./calculate_average_baseline.sh"
```
