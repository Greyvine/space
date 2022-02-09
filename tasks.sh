#!/usr/bin/env bash

set -euxo pipefail

declare -r virtual_base_path=/imgs

main() {
    local -r file=/home/pepe/Pictures/planets/GRAY_HR_SR_OB_DR.tif
    local -r base_path=$(dirname "$file")
    local -r resolution=512

    warp $file front.tiff $resolution 0 0 &
    warp $file right.tiff $resolution 0 90 &
    warp $file back.tiff $resolution 0 180 &
    warp $file left.tiff $resolution 0 -90 &
    warp $file top.tiff $resolution 90 0 &
    warp $file bottom.tiff $resolution -90 0 &
    wait

    docker run -v "$base_path:$virtual_base_path" dpokidov/imagemagick \
        $virtual_base_path/right.tiff \
        $virtual_base_path/left.tiff \
        $virtual_base_path/top.tiff \
        $virtual_base_path/bottom.tiff \
        $virtual_base_path/back.tiff \
        $virtual_base_path/front.tiff \
        -append $virtual_base_path/combined.png
}

warp() {
    local -r input=$1
    local -r output=$2
    local -r res=$3
    local -r lat=$4
    local -r lon=$5

    local -r extent=6378137
    local -r input_base_path=$(dirname "$input")
    local -r input_base_name=$(basename "$input")

    docker run --rm --mount type=bind,source="$input_base_path",target="$virtual_base_path" osgeo/gdal \
        gdalwarp \
            -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84 +lat_0=$lat +lon_0=$lon" \
            -overwrite \
            -wo SOURCE_EXTRA=100 \
            -wo SAMPLE_GRID=YES \
            -te -"$extent" -"$extent" "$extent" "$extent" \
            -ts "$res" "$res" \
            "$virtual_base_path/$input_base_name" "$virtual_base_path/$output"
}

main "$@"