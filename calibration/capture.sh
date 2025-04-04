#!/bin/bash
#gst-launch-1.0 -e v4l2src device=/dev/video2 ! video/x-raw,format=NV12,width=3840,height=2160,framerate=30/1 \
#    ! tee name=t ! queue ! videoconvert ! autovideosink sync=false \
#    t. ! queue ! videoconvert ! x264enc bitrate=50000 tune=zerolatency qp-min=10 qp-max=30 speed-preset=veryslow \
#    ! matroskamux ! filesink location=calibration_video.mkv
#
gst-launch-1.0 -e v4l2src device=/dev/video2 ! image/jpeg,width=1920,height=1080,framerate=30/1 \
    ! tee name=t \
    t. ! queue ! jpegdec ! videoconvert ! autovideosink sync=false \
    t. ! queue ! jpegdec ! videoconvert ! x264enc bitrate=50000 tune=zerolatency qp-min=10 qp-max=30 speed-preset=veryslow \
    ! matroskamux ! filesink location=calibration_video.mkv
