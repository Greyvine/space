#!/usr/bin/env bash

set -euxo pipefail





# directory=/home/pepe/Pictures/planets
# docker run -v "$directory":/imgs dpokidov/imagemagick /imgs/frontside2* -append /imgs/combin.png







# file=/app/GRAY_HR_SR_OB_DR.tif
# # file=/app/pluto.jpg
# # file=/app/gebco_08_rev_elev_21600x10800.png
# resolution=512
# extent=6378137



# docker run --rm --mount type=bind,source=/home/pepe/Pictures/planets,target=/app osgeo/gdal \
#     gdalwarp \
#     -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84 +lat_0=0 +lon_0=0" \
#     -overwrite \
#     -wo SOURCE_EXTRA=100 \
#     -wo SAMPLE_GRID=YES \
#     -te -"$extent" -"$extent" "$extent" "$extent" \
#     -ts "$resolution" "$resolution" \
#     "$file" /app/frontside.tiff


# dpokidov/imagemagick



    # -to SRC_METHOD=NO_GEOTRANSFORM \

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

    docker run -v "$base_path":/imgs dpokidov/imagemagick \
        /imgs/right.tiff \
        /imgs/left.tiff \
        /imgs/top.tiff \
        /imgs/bottom.tiff \
        /imgs/back.tiff \
        /imgs/front.tiff \
        -append /imgs/combined.png
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
    local -r virtual_base_path=/imgs

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